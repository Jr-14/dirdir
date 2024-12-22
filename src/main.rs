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
    // Get(String),
    Get,
    Set,
    List,
    // Add(String),
    Add {
        /// Interesting name
        path: String,

        #[arg(short, long)]
        list: bool,
    }

}

#[derive(Debug)]
struct DirectoryEntry {
    id: Option<i32>,
    pub path: String,
}

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

#[derive(Debug)]
struct Directory {
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

    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;


    conn.execute(
        "CREATE TABLE IF NOT EXISTS directories (
            id      INTEGER PRIMARY KEY,
            path    TEXT NOT NULL
        )",
        (), // empty list of parameters.
    )?;
    let dir = DirectoryEntry::new("hello world");
    conn.execute(
        "INSERT INTO directories (path, something) VALUES (?1, ?2)",
        (&dir.path, "something"),
    )?;

    match args.cmd {
        Commands::Get => { println!("Hello from command GET") },
        Commands::Set => { println!("Hello from command SET") },
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
        Commands::Add { list, path } => {
            let new_dir = DirectoryEntry::new(&path);
            conn.execute(
                "INSERT INTO directories (path) VALUES (?1)",
                (&new_dir.path),
            )?;
            if list {
                println!("Hello from command Add: {:?}, name is: {:?}", list, name);
            } else {
                println!("Hello from command Add: {:?}, name is: {:?} - ohh nooo", list, name);
            }
        }
    }
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
    Ok(())
}
