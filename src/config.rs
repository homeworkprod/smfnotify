/*
 * Copyright 2022 Jochen Kupperschmidt
 * License: MIT
 */

use anyhow::Result;
use serde::Deserialize;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub feed_cookie_value: String,
    pub feed_url: String,
    pub last_processed_id_filename: PathBuf,
    pub webhook_text_template: String,
    pub webhook_url: String,
}

/// Load configuration from TOML file.
pub(crate) fn load_config(path: &Path) -> Result<Config> {
    let text = read_to_string(path)?;
    let config: Config = toml::from_str(&text)?;
    Ok(config)
}
