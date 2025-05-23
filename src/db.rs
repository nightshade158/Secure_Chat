use rusqlite::{params, Connection};

pub fn initialize_db() {
    let conn = Connection::open("database.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE,
            password TEXT
        )",
        [],
    ).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sender TEXT,
            receiver TEXT,
            iv TEXT,
            encrypted_message TEXT
        )",
        [],
    ).unwrap();
}

pub fn register_user(username: &str, password: &str) -> bool {
    let conn = Connection::open("database.db").unwrap();
    conn.execute(
        "INSERT INTO users (username, password) VALUES (?1, ?2)",
        params![username, password],
    ).is_ok()
}

pub fn authenticate_user(username: &str, password: &str) -> bool {
    let conn = Connection::open("database.db").unwrap();
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM users WHERE username = ?1 AND password = ?2").unwrap();
    let count: i32 = stmt.query_row(params![username, password], |row| row.get(0)).unwrap();
    count > 0
}

pub fn store_message(sender: &str, receiver: &str, iv: &str, encrypted_message: &str) -> bool {
    let conn = Connection::open("database.db").unwrap();
    conn.execute(
        "INSERT INTO messages (sender, receiver, iv, encrypted_message) VALUES (?1, ?2, ?3, ?4)",
        params![sender, receiver, iv, encrypted_message],
    ).is_ok()
}

pub fn fetch_unread_messages(receiver: &str) -> Vec<(i32, String, String, String)> {
    let conn = Connection::open("database.db").unwrap();
    let mut stmt = conn.prepare("SELECT id, sender, iv, encrypted_message FROM messages WHERE receiver = ?1").unwrap();
    let messages = stmt.query_map(params![receiver], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
    }).unwrap();
    messages.filter_map(Result::ok).collect()
}

pub fn delete_message(id: i32) {
    let conn = Connection::open("database.db").unwrap();
    conn.execute("DELETE FROM messages WHERE id = ?1", params![id]).unwrap();
}
