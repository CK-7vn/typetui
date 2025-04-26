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
            "CREATE TABLE if not exists tests (
        id integer primary key,
        username text not null ,
        wpm integer not null ,
        raw_wpm integer,
        accuracy integer not null,
        word_count integer not null,
        time integer not null)",
            [],
        );
        Ok(db)
    }
    pub fn add_test(
        &mut self,
        username: String,
        wpm: i32,
        raw_wpm: i32,
        accuracy: i32,
        word_count: i32,
        time: i32,
    ) {
        let _ = self.conn.execute(
            "INSERT INTO tests (username, wpm, raw_wpm, accuracy, word_count, time) values (?1, ?2, ?3, ?4, ?5, ?6)",
            params![username, wpm,raw_wpm, accuracy, word_count, time],
        );
    }
    pub fn get_all_tests(&self) -> Result<Vec<(String, i32, i32, i32, i32, i32)>> {
        let mut stmt = self.conn.prepare(
            "SELECT username, wpm, raw_wpm, accuracy, word_count, time
           FROM tests
          ORDER BY wpm DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i32>(1)?,
                row.get::<_, i32>(2)?,
                row.get::<_, i32>(3)?,
                row.get::<_, i32>(4)?,
                row.get::<_, i32>(5)?,
            ))
        })?;
        let mut results = Vec::new();
        for r in rows {
            results.push(r?);
        }
        Ok(results)
    }
}
