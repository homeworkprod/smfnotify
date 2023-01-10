/*
 * Copyright 2022-2023 Jochen Kupperschmidt
 * License: MIT
 */

use crate::files;
use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct IdKeeper {
    pub(crate) filename: PathBuf,
}

impl IdKeeper {
    pub(crate) fn read_last_processed_id(&self) -> Option<String> {
        files::read_text_file(&self.filename)
    }

    pub(crate) fn write_last_processed_id(&self, last_processed_id: &str) -> Result<()> {
        files::write_text_file(&self.filename, last_processed_id)?;
        Ok(())
    }
}
