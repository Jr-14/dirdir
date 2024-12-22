use rusqlite::{Connection, Result};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Get,
    Set,
    Add
}

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

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    // let conn = Connection::open("./testdb.sqlite")?;
    //
    // conn.execute(
    //     "CREATE TABLE IF NOT EXISTS person (
    //         id    INTEGER PRIMARY KEY,
    //         name  TEXT NOT NULL,
    //         data  BLOB
    //     )",
    //     (), // empty list of parameters.
    // )?;
    // let me = Person {
    //     id: 0,
    //     name: "Steven".to_string(),
    //     data: None,
    // };
    // conn.execute(
    //     "INSERT INTO person (name, data) VALUES (?1, ?2)",
    //     (&me.name, &me.data),
    // )?;
    //
    // let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    // let person_iter = stmt.query_map([], |row| {
    //     Ok(Person {
    //         id: row.get(0)?,
    //         name: row.get(1)?,
    //         data: row.get(2)?,
    //     })
    // })?;
    //
    // for person in person_iter {
    //     println!("Found person {:?}", person.unwrap());
    // }
    // Ok(())
}
