/*
 * Copyright 2022 Jochen Kupperschmidt
 * License: MIT
 */

use anyhow::Result;
use std::fs;
use std::path::Path;

pub(crate) fn read_text_file(path: &Path) -> Option<String> {
    fs::read_to_string(path)
        .map(|s| s.trim_end().to_string())
        .map(Some)
        .unwrap_or(None)
}

pub(crate) fn write_text_file(path: &Path, content: &str) -> Result<()> {
    fs::write(path, content)?;
    Ok(())
}
