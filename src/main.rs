// RSS to sqlite database collector

mod modules {
    pub mod sql;
}

use modules::sql::create_db;
use modules::sql::insert_feed_items;
use modules::sql::last;

use anyhow::{Context, Result};
use env_logger::Builder;
use log::{debug, info, warn, LevelFilter};
use reqwest::blocking;
use rss::Channel;
use serde::Deserialize;
// use std::error::Error;
use std::fs;

#[derive(Deserialize)]
struct FeedConfig {
    dbfile: String,
    feeds: Vec<String>,
}

fn load_config() -> Result<FeedConfig> {
    debug!("Loading configuration");
    let config_data =
        fs::read_to_string("config.json").context("Failed to read configuration file")?;
    debug!("Parsing configuration");
    let config: FeedConfig =
        serde_json::from_str(&config_data).context("Failed to parse configuration file")?;
    Ok(config)
}

fn fetch_rss_feed(url: &str) -> Result<Channel> {
    debug!("Fetching RSS feed {}", url);
    let response = blocking::get(url)?.text()?;
    let channel = Channel::read_from(response.as_bytes())?;
    Ok(channel)
}

fn main() -> Result<()> {
    let mut builder = Builder::from_default_env();
    builder.filter_level(LevelFilter::Info);
    builder.init();
    // let's go
    info!("RSS feed collector");
    warn!("Work in progress");
    let config = load_config()?;
    let conn = create_db(&config.dbfile)?;
    info!("Processing {} feeds", config.feeds.len());
    for feed in config.feeds.iter() {
        let parsed_feed = fetch_rss_feed(feed).context("Failed to parse feed")?;
        info!("{} items from {}", parsed_feed.items().len(), feed);
        insert_feed_items(&conn, &parsed_feed).context("Failed to insert record")?;
    }
    let _ = last(&conn).context("Failed to query the database")?;
    info!("Finished");
    Ok(())
}
