use rig::{
    OneOrMany,
    agent::{Agent, Text},
    completion::{Completion, CompletionModel},
    message::{AssistantContent, Message, ToolResult, ToolResultContent, UserContent},
};

use crate::wasm::runtime::WasmRuntime;

pub trait AgentRuntimeExt<M>
where
    M: CompletionModel,
{
    fn with_wasm_runtime(self, runtime: WasmRuntime) -> AgentWithRuntime<M>;
}

impl<M> AgentRuntimeExt<M> for Agent<M>
where
    M: CompletionModel,
{
    fn with_wasm_runtime(self, runtime: WasmRuntime) -> AgentWithRuntime<M> {
        AgentWithRuntime {
            model: self,
            runtime,
        }
    }
}

pub struct AgentWithRuntime<M>
where
    M: CompletionModel,
{
    model: Agent<M>,
    runtime: WasmRuntime,
}

impl<M> AgentWithRuntime<M>
where
    M: CompletionModel,
{
    pub async fn prompt(
        &mut self,
        prompt: impl Into<rig::message::Message> + rig::wasm_compat::WasmCompatSend,
    ) -> Result<String, rig::completion::PromptError> {
        let tooldefs = self.runtime.get_tooldefs();

        let mut chat_history: Vec<Message> = Vec::new();

        let mut current_prompt: Message = prompt.into();

        loop {
            let res = self
                .model
                .completion(current_prompt.clone(), chat_history.clone())
                .await?
                .tools(tooldefs.clone())
                .send()
                .await
                .unwrap();

            let mut tool_calls = Vec::new();
            let mut current_response = String::new();

            for choice in res.choice.iter() {
                match choice {
                    AssistantContent::Text(text) => {
                        current_response = text.text.clone();
                    }
                    AssistantContent::ToolCall(tc) => tool_calls.push(tc.clone()),
                    _ => {}
                }
            }

            chat_history.push(Message::Assistant {
                id: None,
                content: res.choice,
            });

            if tool_calls.is_empty() {
                break Ok(current_response);
            } else {
                let mut tool_results = Vec::new();

                for tc in tool_calls {
                    println!(
                        "Calling function {name} with args {args:?}",
                        name = tc.function.name,
                        args = tc.function.arguments
                    );

                    let text = self
                        .runtime
                        .run_tool(&tc.function.name, tc.function.arguments.clone())
                        .unwrap();

                    println!(
                        "Tool returned result: {text} Tool callID: {id:?} Tool call: {call:?}",
                        id = tc.call_id,
                        call = tc.id
                    );

                    tool_results.push(UserContent::ToolResult(ToolResult {
                        id: tc.id.clone(),
                        call_id: tc.call_id.clone(),
                        content: OneOrMany::one(ToolResultContent::Text(Text { text })),
                    }));
                }

                current_prompt = Message::User {
                    content: OneOrMany::many(tool_results).unwrap(),
                };
            }
        }
    }
}
