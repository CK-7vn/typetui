use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct DB {
    pub conn: rusqlite::Connection,
}

impl DB {
    pub fn new() -> Result<DB> {
        let db_con = match Connection::open("typetui.db") {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error opening or creating database: {}", e);
                return Err(e);
            }
        };
        let db = DB { conn: db_con };
        let _ = db.conn.execute(
            "CREATE TABLE if not exists users (
        id integer primary key,
        username text not null unique,
        total_tests integer,
        top_wpm integer)",
            [],
        );

        let _ = db.conn.execute(
            "CREATE TABLE if not exists  tests (
        id integer primary key,
        username text not null ,
        wpm integer not null )",
            [],
        );
        Ok(db)
    }
    pub fn add_test(&mut self, username: String, wpm: i32) {
        let _ = self.conn.execute(
            "INSERT INTO tests (username, wpm) values (?1, ?2)",
            params![username, wpm],
        );
    }

    pub fn get_all_tests(&self) -> Result<Vec<(String, i32)>> {
        let mut stmt = self
            .conn
            .prepare("SELECT username, wpm FROM tests ORDER BY wpm DESC")?;
        let rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;
        let mut results = Vec::new();
        for r in rows {
            results.push(r?);
        }
        Ok(results)
    }
}
