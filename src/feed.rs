/*
 * Copyright 2022 Jochen Kupperschmidt
 * License: MIT
 */

use anyhow::Result;
use feed_rs::model::Entry;
use feed_rs::parser;
use std::io::Read;

#[derive(Debug)]
pub(crate) struct FeedReader {
    pub(crate) url: String,
    pub(crate) cookie_value: String,
}

impl FeedReader {
    pub(crate) fn get_new_entries(&self, last_processed_id: Option<String>) -> Result<Vec<Entry>> {
        let reader = self.fetch_feed()?;
        let feed = parser::parse(reader)?;
        let entries = feed.entries;

        let new_entries = match last_processed_id {
            Some(id) => select_new_entries(&entries, &id),
            None => entries,
        };

        Ok(new_entries)
    }

    fn fetch_feed(&self) -> Result<impl Read> {
        let cookie = format!("SMFCookie10={}", self.cookie_value);
        let request = ureq::get(&self.url).set("Cookie", &cookie);
        let response = request.call()?;
        let reader = response.into_reader();
        Ok(reader)
    }
}

fn select_new_entries(entries: &[Entry], last_processed_id: &str) -> Vec<Entry> {
    entries
        .iter()
        .take_while(|e| e.id != last_processed_id)
        .cloned()
        .collect()
}
