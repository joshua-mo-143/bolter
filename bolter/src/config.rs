use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub data: Vec<Module>,
}

impl Config {
    pub fn from_file<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let bytes = std::fs::read(path).unwrap();

        serde_json::from_slice(&bytes).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Module {
    pub path: PathBuf,
    pub title: String,
    pub description: String,
    pub module_type: ModuleKind,
    #[serde(default)]
    pub permissions: Permissions,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ModuleKind {
    Binary,
    Tool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Permissions {
    http: bool,
}

impl Permissions {
    pub fn http(&self) -> bool {
        self.http
    }
}
