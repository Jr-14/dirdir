use rusqlite::{Connection, Result};

#[derive(Debug)]
struct DirectoryEntry {
    id: Option<i32>,
    pub path: String,
    pub name: Option<String>,
}

#[derive(Debug)]
struct Directory {
}

impl DirectoryEntry {
    pub fn new(path: &str) -> Self {
        Self {
            id: None,
            path: String::from(path),
            name: None
        }
    }
}

fn main() -> Result<()> {
    let conn = Connection::open("./db/testdb.sqlite")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS directory (
            id    INTEGER PRIMARY KEY,
            path  TEXT NOT NULL,
            name  TEXT
        )",
        (), // empty list of parameters.
    )?;

    let dir = DirectoryEntry::new("./hello/world");

    conn.execute(
        "INSERT INTO directory (name, path) VALUES (?1, ?2)",
        (&dir.name, &dir.path)
    )?;

    let mut stmt = conn.prepare("SELECT * FROM directory")?;
    let dir_iter = stmt.query_map([], |row| {
        Ok(DirectoryEntry {
            id: row.get(0)?,
            path: row.get(1)?,
            name: row.get(2)?,
        })
    })?;

    for dir in dir_iter {
        println!("Found directory {:?}", dir.unwrap());
    }

    Ok(())
}
