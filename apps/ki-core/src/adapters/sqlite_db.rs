use rusqlite::{params, Connection};

pub struct SqliteDb {
    pub conn: Connection,
}

impl SqliteDb {
    pub fn new(db_path: &str) -> Result<Self, String> {
        let conn = Connection::open(db_path).map_err(|e| e.to_string())?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS food_logs (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                protein REAL NOT NULL,
                carbs REAL NOT NULL,
                fat REAL NOT NULL,
                calories INTEGER NOT NULL,
                logged_at TEXT NOT NULL
            )",
            [],
        ).map_err(|e| e.to_string())?;

        Ok(Self { conn })
    }
}

