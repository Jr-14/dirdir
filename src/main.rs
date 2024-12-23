use rusqlite::{Connection, Result, params};
use clap::{Args, Parser, Subcommand};

#[derive(Args, Debug, Clone)]
#[group(required = true, multiple = false)]
struct PathOrName {
    #[arg(long)]
    name: Option<String>,

    #[arg(long)]
    path: Option<String>,
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
    },

    #[command(aliases = ["rm"])]
    Delete {
        #[command(flatten)]
        path_or_name: PathOrName,
    },
}

#[derive(Parser, Debug)]
#[command(name = "sd", author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands
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
    let args = Cli::parse();
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

    match &args.cmd {
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
            let new_dir = DirectoryEntry::new(&path, name.clone());
            conn.execute(
                "INSERT INTO directories (path, name) VALUES (?1, ?2)",
                params![&new_dir.path, &new_dir.name]
            )?;
        },
        Commands::Delete { path_or_name } => {
            if let Some(path) = &path_or_name.path {
                conn.execute("DELETE FROM directories WHERE path = ?;", [path])?;
            };

            if let Some(name) = &path_or_name.name {
                conn.execute("DELETE FROM directories WHERE name = ?;", [name])?;
            }
        },
    }

    Ok(())
}
