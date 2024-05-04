use std::{fs, path::Path};

use serde::Deserialize;

use crate::{accounts::AccountType, Result};

#[derive(Deserialize)]
pub struct Config {
    destinations: Vec<Destination>,
}

impl Config {
    pub fn get_pot_for_deposit(&self, source: &AccountType) -> Option<&Destination> {
        self.destinations.iter().find(|d| d.source == *source)
    }
}

#[derive(Deserialize)]
pub struct Destination {
    source: AccountType,
    pub pot_name: String,
}

pub fn load(file: &str) -> Result<Config> {
    let path = Path::new(&file);
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

// [[destinations]]
// source = "personal"
// pot_name = "personal-test-pot"

// [[destinations]]
// source = "joint"
// pot_name = "joint-test-pot"
