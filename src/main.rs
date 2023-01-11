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

    configure_logging(args.quiet);

    let interval = args.interval.map(|secs| Duration::from_secs(secs));

    let config = config::load_config(&args.config_filename)?;

    let app = Application::new(config, interval);

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
    interval: Option<Duration>,
}

impl Application {
    fn new(config: Config, interval: Option<Duration>) -> Self {
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
            interval,
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
