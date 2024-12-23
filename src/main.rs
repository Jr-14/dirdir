use rusqlite::{Connection, Result, params};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "sd", author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[command(aliases = ["ls", "ll"])]
    List,

    #[command(aliases = ["a"])]
    Add {
        /// Path to add
        path: String,

        /// Name of the directory
        name: Option<String>,
    }

}

#[derive(Debug)]
struct DirectoryEntry {
    id: Option<i32>,
    pub path: String,
    pub name: Option<String>
}

impl DirectoryEntry {
    pub fn new(path: &str, name: Option<String>) -> Self {
        Self {
            id: None,
            path: String::from(path),
            name
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("{:?}", args);

    let conn = Connection::open("./db/testdb.sqlite")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS directories (
            id    INTEGER PRIMARY KEY,
            path  TEXT NOT NULL,
            name  TEXT
        )",
        (), // empty list of parameters.
    )?;

    match args.cmd {
        Commands::List => {
            println!("Hello from List");
            let mut stmt = conn.prepare("SELECT id, path, name FROM directories;")?;
            let directories_iter = stmt.query_map([], |row| {
                Ok(DirectoryEntry {
                    id: row.get(0)?,
                    path: row.get(1)?,
                    name: row.get(2)?,
                })
            })?;
            for dir in directories_iter {
                println!("{:?}", dir.unwrap());
            }
        },
        Commands::Add { path, name } => {
            let new_dir = DirectoryEntry::new(&path, name);
            conn.execute(
                "INSERT INTO directories (path, name) VALUES (?1, ?2)",
                params![&new_dir.path, &new_dir.name]
            )?;
        }
    }

    Ok(())
}
