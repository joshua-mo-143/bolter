fn main() {
    println!("Hello world from test-member1!");
}

pub mod foo {
    use schemars::JsonSchema;

    #[derive(serde::Deserialize, JsonSchema)]
    pub struct Foo {
        pub bar: String,
    }
}

#[macros::wasi_tool]
pub fn my_tool(_input: foo::Foo) -> String {
    String::from("Hello world!")
}
