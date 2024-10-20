use chrono::Local;
use log::debug;
use rss::Channel;
use rusqlite::{Connection, Result as SqliteResult};

// create database and table
pub fn create_db(dbfile: &str) -> SqliteResult<Connection> {
    debug!("Executing create_db");
    let conn = Connection::open(dbfile)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS RssFeed (
            post_date DATETIME,
            id INTEGER PRIMARY KEY,
            title TEXT UNIQUE NOT NULL,
            description TEXT,
            content TEXT,
            link TEXT,
            pub_date DATETIME,
            rss_link TEXT
        )",
        [],
    )?;
    Ok(conn)
}

// insert parsed feed items into database
pub fn insert_feed_items(conn: &Connection, feed: &Channel) -> SqliteResult<()> {
    let mut stmt = conn.prepare(
        "INSERT OR IGNORE INTO RssFeed \
        (post_date, title, description, content, link, pub_date, rss_link) \
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
    )?;
    for item in feed.items() {
        debug!("{}", item.title().unwrap());
        stmt.execute([
            // '&*' - reference dereference time
            &*Local::now().to_string(),
            item.title().unwrap_or(""),
            item.description().unwrap_or(""),
            item.content().unwrap_or(""),
            item.link().unwrap_or(""),
            item.pub_date().unwrap_or(""),
            feed.title(),
        ])?;
    }
    Ok(())
}

pub fn last(conn: &Connection) -> SqliteResult<()> {
    println!("10 latest posts:");
    let mut stmt =
        conn.prepare("SELECT post_date, title FROM RssFeed ORDER BY post_date DESC LIMIT 10")?;
    let mut row_num = 0;
    let rows = stmt.query_map([], |row| {
        row_num += 1;
        debug!("Processing row {}", row_num);
        let post_date: String = row.get(0)?;
        let title: String = row.get(1)?;
        Ok((post_date, title))
    })?;
    for row in rows {
        println!("{:?}", row); // No idea what ':?' is, the linter told me to do so
    }
    Ok(()) // Return a tuple wrapped in Ok()
}