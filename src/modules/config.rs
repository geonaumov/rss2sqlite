use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub dbfile: String,
    pub feeds: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            dbfile: "rss_feeds.db".into(),
            feeds: vec![
                "https://rss.slashdot.org/Slashdot/slashdotMain".into(),
                "https://www.engadget.com/rss.xml".into(),
            ],
        }
    }
}