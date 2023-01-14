/*
 * Copyright 2022-2023 Jochen Kupperschmidt
 * License: MIT
 */

use anyhow::Result;
use log::Level;
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr, DurationSeconds};
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[serde_as]
#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    #[serde(default = "get_default_log_level")]
    #[serde_as(as = "DisplayFromStr")]
    pub(crate) log_level: Level,

    pub(crate) feed_cookie_value: String,
    pub(crate) feed_url: String,
    pub(crate) feed_max_new_entries: usize,

    pub(crate) last_processed_id_filename: PathBuf,

    pub(crate) webhook_text_template: String,
    pub(crate) webhook_url: String,

    #[serde(rename = "interval_in_seconds")]
    #[serde_as(as = "Option<DurationSeconds<u64>>")]
    pub(crate) interval: Option<Duration>,
}

fn get_default_log_level() -> Level {
    Level::Error
}

/// Load configuration from TOML file.
pub(crate) fn load_config(path: &Path) -> Result<Config> {
    let text = read_to_string(path)?;
    let config: Config = toml::from_str(&text)?;
    Ok(config)
}
