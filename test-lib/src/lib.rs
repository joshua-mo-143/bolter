use schemars::JsonSchema;

#[derive(serde::Deserialize, JsonSchema)]
pub struct Foo {
    pub bar: String,
}

#[macros::wasi_tool]
pub fn my_tool(input: Foo) -> String {
    let Foo { bar } = input;
    format!("Hello {bar}")
}
