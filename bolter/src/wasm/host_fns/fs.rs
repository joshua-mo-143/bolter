use serde::{Deserialize, Serialize};

pub fn read_dir(body: Vec<u8>) -> Result<String, i32> {
    let path: String = String::from_utf8_lossy(&body).to_string();

    let dir_entries = std::fs::read_dir(&path)
        .unwrap()
        .map(|x| x.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    Ok(dir_entries)
}

pub fn read_file(body: Vec<u8>) -> Result<String, i32> {
    let path: String = String::from_utf8_lossy(&body).to_string();

    let dir_entries = std::fs::read_dir(&path)
        .unwrap()
        .map(|x| x.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    Ok(dir_entries)
}

#[derive(Deserialize, Serialize)]
struct WriteFileRequest {
    path: String,
    contents: String,
}

pub fn write_file(body: Vec<u8>) -> Result<(), i32> {
    let WriteFileRequest { path, contents } = serde_json::from_slice(&body).unwrap();

    std::fs::write(path, contents).unwrap();

    Ok(())
}
