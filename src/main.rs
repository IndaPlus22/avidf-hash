mod dope;
use dope::{HashTable, data::Data};

use clap::{Parser, Subcommand};
use anyhow::{Context, Result};

/// Simple DBMS (Data Base Managment System) storing data in a .csv file.  
#[derive(Parser)]
struct Cli {
    /// The path to a .csv file with database information.
    /// Will create a file if it doesn't exist.
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
    /// The command to run on the database
    #[clap(subcommand)]
    command: Command,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let mut table = read(&args.path)
        .with_context(|| format!("Failed to create table"))?;

    match args.command {
        Command::Insert{ key, value } => {
            let data = Data { key, value };
            table.insert(data)
                .with_context(|| format!("Failed to insert data into table"))?;
        },
        Command::Delete { key } => {
            table.delete(key)
                .with_context(|| "Failed to remove data corresponding to key")?;
        }
        Command::Get{ key } => {
            println!("{}", table.get(key)
                .with_context(|| format!("Failed to get value corresponding to key"))?);
        },
        Command::Print => {
            table.print()
                .with_context(|| format!("Failed to print table"))?;
        }
    };

    write(table, &args.path)
        .with_context(|| format!("Failed to write to file"))?;
    
    Ok(())
}

#[derive(Subcommand, Clone)]
enum Command {
    /// Insert a given value at a given key
    Insert {
        /// The key to insert the value at
        key: String,
        /// The value to insert
        value: String
    },
    /// Delete a value by a given key
    Delete {
        /// The key to remove the value at
        key: String
    },
    /// Get a value by a given key
    Get {
        /// The key to get the value at
        key: String,
    },
    /// Prints table
    Print,
}

fn read(path: &std::path::PathBuf) -> Result<HashTable<String, String>> {
    let rdr = std::fs::File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .with_context(|| format!("Failed to open or create file for path {:?}", path))?;
    let mut reader = csv::Reader::from_reader(rdr);

    let mut table = HashTable::<String, String>::new(13);

    for result in reader.records() {
        let record = result?;
        let (key, value) = (record[0].to_string(), record[1].to_string());
        let data = Data { key, value };
        table.insert(data)
            .with_context(|| "Failed to insert data into table")?;
    }
    Ok(table)
}

fn write(mut table: HashTable<String, String>, path: &std::path::PathBuf) -> Result<()> {
    let mut writer = csv::Writer::from_path(path)
        .with_context(|| format!("Failed to create writer for path {:?}", path))?;
    writer.write_record(["key", "valiue"])
        .with_context(|| format!("Failed to write record"))?;
    for i in 0..table.capacity as usize {
        match &mut table.table[i] {
            Some(_vec) => {
                for j in 0.._vec.len() {
                    let data = _vec[j].clone();
                    writer.write_record([data.key, data.value])
                        .with_context(|| format!("Failed to write record"))?;
                }
            },
            None => ()
        }
    }
    writer.flush()
        .with_context(|| format!("Failed to flush file"))?;
    Ok(())
}

/*Hope you ready for the next episode
Hey, hey, hey, hey
Smoke weed every day */