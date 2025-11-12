//! Host functions.

use crate::secrets::PlaintextOrSecret;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Deserialize, Serialize)]
struct HttpRequest {
    pub body: Vec<u8>,
    pub headers: BTreeMap<String, PlaintextOrSecret>,
    pub url: String,
}

pub fn fetch_url(body: Vec<u8>) -> Result<String, i32> {
    let req: HttpRequest = serde_json::from_slice(&body).unwrap();

    let ctx = reqwest::blocking::Client::new();
    let mut body = ctx.get(req.url);

    for (key, header) in req.headers {
        match header {
            PlaintextOrSecret::Plaintext(val) => {
                body = body.header(key, &val);
            }
            PlaintextOrSecret::Secret(val) => {
                let entry = keyring::Entry::new_with_target("Bolter", "secrets", &val)
                    .unwrap()
                    .get_password()
                    .unwrap();
                body = body.header(key, entry);
            }
        }
    }

    let text = body.send().unwrap().text().unwrap();

    Ok(text)
}

pub fn post_url(body: Vec<u8>) -> Result<String, i32> {
    let req: HttpRequest = serde_json::from_slice(&body).unwrap();

    let ctx = reqwest::blocking::Client::new();
    let mut body = ctx.post(req.url);

    for (key, header) in req.headers {
        match header {
            PlaintextOrSecret::Plaintext(val) => {
                body = body.header(key, &val);
            }
            PlaintextOrSecret::Secret(val) => {
                let entry = keyring::Entry::new_with_target("Bolter", "secrets", &val)
                    .unwrap()
                    .get_password()
                    .unwrap();
                body = body.header(key, entry);
            }
        }
    }

    let text = body.body(req.body).send().unwrap().text().unwrap();

    Ok(text)
}
