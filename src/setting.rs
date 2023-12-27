use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Setting {
    pub username: String,
    pub password: String,
    pub crtpath: String,
    pub excel: String,
    pub factory: Factory,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Factory {
    pub cisco: Vec<String>,
    pub ruijie: Vec<String>,
    pub maipu: Vec<String>,
    pub h3c: Vec<String>,
    pub huawei: Vec<String>,
}

fn read_setting_from_file<P: AsRef<Path>>(path: P) -> Result<Setting, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let s = serde_json::from_reader(reader)?;

    Ok(s)
}

impl Setting {
    pub fn new() -> Self {
        read_setting_from_file("./config.json").unwrap()
    }
}

