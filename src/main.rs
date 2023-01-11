/*
 * Copyright 2022-2023 Jochen Kupperschmidt
 * License: MIT
 */

#[macro_use]
extern crate log;

use announce::Announcer;
use anyhow::Result;
use config::Config;
use feed::FeedReader;
use idkeeper::IdKeeper;
use log::LevelFilter;
mod announce;
mod cli;
mod config;
mod feed;
mod files;
mod idkeeper;

fn main() -> Result<()> {
    let args = cli::parse_args();

    configure_logging(args.quiet);

    let config = config::load_config(&args.config_filename)?;

    let app = Application::new(config);

    app.run()?;

    Ok(())
}

fn configure_logging(quiet: bool) {
    let level_filter = if quiet {
        LevelFilter::Error
    } else {
        LevelFilter::Info
    };
    env_logger::builder().filter_level(level_filter).init();
}

struct Application {
    id_keeper: IdKeeper,
    feed_reader: FeedReader,
    announcer: Announcer,
}

impl Application {
    fn new(config: Config) -> Self {
        Self {
            id_keeper: IdKeeper {
                filename: config.last_processed_id_filename,
            },
            feed_reader: FeedReader {
                url: config.feed_url,
                cookie_value: config.feed_cookie_value,
            },
            announcer: Announcer {
                webhook_text_template: config.webhook_text_template,
                webhook_url: config.webhook_url,
            },
        }
    }

    fn run(&self) -> Result<()> {
        let last_processed_id = self.id_keeper.read_last_processed_id();

        let new_entries = &self.feed_reader.get_new_entries(last_processed_id)?;
        let new_entries_len = new_entries.len();

        if new_entries_len > 0 {
            info!("Found {new_entries_len} new entries.");

            self.announcer.announce_new_entries(new_entries)?;

            let new_last_processed_id = &new_entries[0].id;
            self.id_keeper
                .write_last_processed_id(new_last_processed_id)?;
        } else {
            info!("Found no new entries.");
        }

        Ok(())
    }
}
