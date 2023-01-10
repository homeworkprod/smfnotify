/*
 * Copyright 2022-2023 Jochen Kupperschmidt
 * License: MIT
 */

use announce::Announcer;
use anyhow::Result;
use config::Config;
use feed::FeedReader;
use idkeeper::IdKeeper;
mod announce;
mod cli;
mod config;
mod feed;
mod files;
mod idkeeper;

fn main() -> Result<()> {
    let args = cli::parse_args();

    let config = config::load_config(&args.config_filename)?;

    let app = Application::new(config, args.quiet);

    app.run()?;

    Ok(())
}

struct Application {
    id_keeper: IdKeeper,
    feed_reader: FeedReader,
    announcer: Announcer,
    quiet: bool,
}

impl Application {
    fn new(config: Config, quiet: bool) -> Self {
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
            quiet,
        }
    }

    fn run(&self) -> Result<()> {
        let last_processed_id = self.id_keeper.read_last_processed_id();

        let new_entries = &self.feed_reader.get_new_entries(last_processed_id)?;
        let new_entries_len = new_entries.len();

        if new_entries_len > 0 {
            if !self.quiet {
                println!("Found {new_entries_len} new entries.");
            }

            self.announcer.announce_new_entries(new_entries)?;

            let new_last_processed_id = &new_entries[0].id;
            self.id_keeper
                .write_last_processed_id(new_last_processed_id)?;
        } else if !self.quiet {
            println!("Found no new entries.");
        }

        Ok(())
    }
}
