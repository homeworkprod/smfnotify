/*
 * Copyright 2022-2023 Jochen Kupperschmidt
 * License: MIT
 */

#[macro_use]
extern crate log;

use crate::announce::Announcer;
use crate::config::Config;
use crate::feed::FeedReader;
use crate::idkeeper::IdKeeper;
use anyhow::Result;
use log::Level;
use std::thread;
use std::time::Duration;
mod announce;
mod cli;
mod config;
mod feed;
mod files;
mod idkeeper;

fn main() -> Result<()> {
    let args = cli::parse_args();

    let config = config::load_config(&args.config_filename)?;

    configure_logging(config.log_level);

    let app = Application::new(config);

    app.run()?;

    Ok(())
}

fn configure_logging(level: Level) {
    simple_logger::init_with_level(level).unwrap()
}

struct Application {
    id_keeper: IdKeeper,
    feed_reader: FeedReader,
    announcer: Announcer,
    interval: Option<Duration>,
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
            interval: config.interval,
        }
    }

    fn run(&self) -> Result<()> {
        match self.interval {
            Some(duration) => self.run_looped(duration)?,
            None => self.run_once()?,
        }
        Ok(())
    }

    fn run_once(&self) -> Result<()> {
        let last_processed_id = self.id_keeper.read_last_processed_id();

        let new_entries = &self.feed_reader.get_new_entries(last_processed_id)?;
        let new_entries_len = new_entries.len();
        info!("New entries found: {new_entries_len}");

        if new_entries_len > 0 {
            self.announcer.announce_new_entries(new_entries)?;

            let new_last_processed_id = &new_entries[0].id;
            self.id_keeper
                .write_last_processed_id(new_last_processed_id)?;
        }

        Ok(())
    }

    fn run_looped(&self, interval: Duration) -> Result<()> {
        info!("Interval: {} seconds", interval.as_secs());

        loop {
            self.run_once()?;
            thread::sleep(interval);
        }
    }
}
