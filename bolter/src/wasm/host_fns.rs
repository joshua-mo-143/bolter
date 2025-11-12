//! Host functions.

#[derive(Deserialize, Serialize)]
struct HttpRequest {
    pub body: Vec<u8>,
    pub headers: BTreeMap<String, PlaintextOrSecret>,
    pub url: String,
}

pub fn fetch_url(body: Vec<u8>) -> Result<String, i32> {
    let req: HttpRequest = serde_json::from_slice(&body).unwrap();

    let body = reqwest::blocking::get(req.url).unwrap().text().unwrap();

    Ok(body)
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
