/*
 * Copyright 2022 Jochen Kupperschmidt
 * License: MIT
 */

use anyhow::Result;
use feed_rs::model::Entry;
use feed_rs::parser;
use std::io::Read;

pub(crate) fn get_new_entries(
    url: &str,
    cookie_value: &str,
    last_processed_id: Option<String>,
) -> Result<Vec<Entry>> {
    let reader = fetch_feed(url, cookie_value)?;
    let feed = parser::parse(reader)?;
    let entries = feed.entries;

    let new_entries = match last_processed_id {
        Some(id) => select_new_entries(&entries, &id),
        None => entries,
    };

    Ok(new_entries)
}

fn fetch_feed(url: &str, cookie_value: &str) -> Result<impl Read> {
    let cookie = format!("SMFCookie10={cookie_value}");
    let request = ureq::get(url).set("Cookie", &cookie);
    let response = request.call()?;
    let reader = response.into_reader();
    Ok(reader)
}

fn select_new_entries(entries: &[Entry], last_processed_id: &str) -> Vec<Entry> {
    entries
        .iter()
        .take_while(|e| e.id != last_processed_id)
        .cloned()
        .collect()
}
