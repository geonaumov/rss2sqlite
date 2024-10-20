mod modules {
    pub mod sql;
    pub mod config;
}

use modules::sql::create_db;
use modules::sql::insert_feed_items;
use modules::sql::last;
use modules::config::Config;

use std::error::Error;
use clap::{Arg, Command};
use anyhow::{Context, Result};
use env_logger;
use env_logger::Builder;
use log::{debug, error, info, warn, LevelFilter};
use reqwest::blocking;
use rss::Channel;

fn fetch_rss_feed(url: &str) -> Result<Channel> {
    debug!("Fetching RSS feed {}", url);
    let response = blocking::get(url)?.text()?;
    let channel = Channel::read_from(response.as_bytes())?;
    Ok(channel)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Setup logger
    let mut builder = Builder::from_default_env();
    builder.filter_level(LevelFilter::Info);
    builder.init();

    // Setup command arguments
    let matches = Command::new("RSS to SQLite")
        .version("0.1.1")
        .author("Geo Naumov <george.naumov@outlook.com>")
        .about("A collector for RSS feeds written in Rust. Uses the SQLite database.")
        .subcommand(Command::new("update").about("Update the feeds"))
        .subcommand(Command::new("last").about("Print last news items"))
        .arg(
            Arg::new("debug")
                .help("Turn on debugging information")
                .short('d')
                .long("debug")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // Debug flag
    if matches.get_flag("debug") {
        println!("Debugging mode is on");
    }

    // Match subcommands
    match matches.subcommand() {
        Some(("update", _)) => {
            info!("Updating feeds...");
            // Load configuration
            debug!("Loading configuration");
            let config: Config = confy::load("RSS2SQLite", None)?;
            let conn = create_db(&config.dbfile);
            info!("Processing {} feeds", config.feeds.len());
            for feed in config.feeds.iter() {
                let parsed_feed = fetch_rss_feed(feed).context("Failed to parse feed");
                info!("Feed: {}", feed);
                match &conn {
                    Ok(ref conn) => {
                        insert_feed_items(&conn, &parsed_feed.unwrap()).context("Failed to insert record")?;
                    },
                    Err(..) => {
                        error!("Database error!");
                    }
                }
            }
        }
        Some(("last", _)) => {
            info!("Last news feeds...");
            let config: Config = confy::load("RSS2SQLite", None)?;
            let conn = create_db(&config.dbfile);
            let _ = last(&conn.unwrap()).context("Failed to query the database")?;
        }
        _ => warn!("Subcommand is required"),
    }
    info!("Finished!");
    Ok(())
}
