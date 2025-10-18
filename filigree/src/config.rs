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
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ModuleKind {
    Binary,
}
