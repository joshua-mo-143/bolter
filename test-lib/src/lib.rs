#[derive(serde::Deserialize)]
pub struct Foo {
    pub bar: String,
}

#[macros::wasi_tool]
pub fn my_tool(_input: Foo) -> String {
    String::from("Hello world from test-lib!")
}
