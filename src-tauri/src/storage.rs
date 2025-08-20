use rusqlite::Connection;

pub fn init_db() -> rusqlite::Result<Connection> {
    let conn = Connection::open("conversations.sqlite")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY,
            chat_id TEXT,
            role TEXT,
            text TEXT,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    Ok(conn)
}

// Add insert, query methods
