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
    List,
    Add {
        /// Path to add
        path: String,
    }

}

#[derive(Debug)]
struct DirectoryEntry {
    id: Option<i32>,
    pub path: String,
}

impl DirectoryEntry {
    pub fn new(path: &str) -> Self {
        Self {
            id: None,
            path: String::from(path),
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("{:?}", args);

    let conn = Connection::open("./testdb.sqlite")?;

    match args.cmd {
        Commands::List => {
            println!("Hello from List");
            let mut stmt = conn.prepare("SELECT id, path FROM directories")?;
            let directories_iter = stmt.query_map([], |row| {
                Ok(DirectoryEntry {
                    id: row.get(0)?,
                    path: row.get(1)?,
                })
            })?;
            for dir in directories_iter {
                println!("{:?}", dir.unwrap());
            }
        },
        Commands::Add { path } => {
            let new_dir = DirectoryEntry::new(&path);
            conn.execute(
                "INSERT INTO directories (path) VALUES (?1)",
                [&new_dir.path],
            )?;
        }
    }

    Ok(())
}
