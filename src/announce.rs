/*
 * Copyright 2022-2023 Jochen Kupperschmidt
 * License: MIT
 */

use feed_rs::model::Entry;

#[derive(Debug)]
pub(crate) struct Announcer {
    pub(crate) webhook_text_template: String,
    pub(crate) webhook_url: String,
}

impl Announcer {
    pub(crate) fn announce_new_entries(&self, entries: &[Entry]) {
        for entry in entries {
            let text = assemble_text(&self.webhook_text_template, entry);
            call_webhook(&self.webhook_url, &text);
        }
    }
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

fn call_webhook(url: &str, text: &str) {
    let request = ureq::post(url);
    let result = request.send_json(ureq::json!({ "text": text }));
    match result {
        Err(e) => error!("Call to webhook failed: {}", e),
        Ok(_) => info!("Call to webhook succeeded"),
    };
}
