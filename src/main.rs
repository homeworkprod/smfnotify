/*
 * Copyright 2022 Jochen Kupperschmidt
 * License: MIT
 */

use announce::Announcer;
use anyhow::Result;
use feed::FeedReader;
use std::path::Path;
mod announce;
mod cli;
mod config;
mod feed;
mod files;

fn main() -> Result<()> {
    let args = cli::parse_args();

    let config = config::load_config(&args.config_filename)?;

    let announcer = Announcer {
        webhook_text_template: config.webhook_text_template,
        webhook_url: config.webhook_url,
    };

    let feed_reader = FeedReader {
        url: config.feed_url,
        cookie_value: config.feed_cookie_value,
    };

    let last_processed_id = read_last_processed_id(&config.last_processed_id_filename);

    let new_entries = feed_reader.get_new_entries(last_processed_id)?;
    let new_entries_len = new_entries.len();

    if new_entries_len > 0 {
        if !args.quiet {
            println!("Found {new_entries_len} new entries.");
        }

        announcer.announce_new_entries(&new_entries)?;

        let new_last_processed_id = &new_entries[0].id;
        write_last_processed_id(&config.last_processed_id_filename, new_last_processed_id)?;
    } else {
        if !args.quiet {
            println!("Found no new entries.");
        }
    }

    Ok(())
}

fn read_last_processed_id(path: &Path) -> Option<String> {
    files::read_text_file(path)
}

fn write_last_processed_id(path: &Path, last_processed_id: &str) -> Result<()> {
    files::write_text_file(path, last_processed_id)?;
    Ok(())
}
