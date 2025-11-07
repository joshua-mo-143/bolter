//! Host functions.

pub fn fetch_url(_ptr: i32, _len: i32) -> Result<(), i32> {
    let url = "https://example.com";

    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Response: {}", &body[..body.len().min(100)]); // print first 100 chars
    Ok(())
}

pub fn post_url(body: Vec<u8>) -> Result<String, i32> {
    let url = "https://httpbin.org/post";

    let body = reqwest::blocking::Client::new()
        .post(url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .unwrap()
        .text()
        .unwrap();

    Ok(body)
}
