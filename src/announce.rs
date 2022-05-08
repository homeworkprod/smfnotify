/*
 * Copyright 2022 Jochen Kupperschmidt
 * License: MIT
 */

use crate::config::Config;
use anyhow::Result;
use feed_rs::model::Entry;

pub(crate) fn announce_new_entries(config: &Config, entries: &[Entry]) -> Result<()> {
    for entry in entries {
        let text = assemble_text(&config.webhook_text_template, entry);
        call_webhook(&config.webhook_url, &text)?;
    }
    Ok(())
}

fn assemble_text(template: &str, entry: &Entry) -> String {
    let author = get_entry_author(entry);
    let title = get_entry_title(entry);
    let url = get_entry_url(entry);

    template
        .replace("{author}", &author)
        .replace("{title}", &title)
        .replace("{url}", &url)
}

fn get_entry_author(entry: &Entry) -> String {
    let fallback = "Somebody".to_string();
    entry
        .authors
        .first()
        .map(|p| &p.name)
        .unwrap_or(&fallback)
        .to_string()
}

fn get_entry_title(entry: &Entry) -> String {
    let fallback = "?".to_string();
    entry
        .title
        .as_ref()
        .map(|t| &t.content)
        .unwrap_or(&fallback)
        .to_string()
}

fn get_entry_url(entry: &Entry) -> String {
    let fallback = "".to_string();
    entry
        .links
        .first()
        .map(|l| &l.href)
        .unwrap_or(&fallback)
        .to_string()
}

fn call_webhook(url: &str, text: &str) -> Result<()> {
    let request = ureq::post(url);
    request.send_json(ureq::json!({ "text": text }))?;
    Ok(())
}
