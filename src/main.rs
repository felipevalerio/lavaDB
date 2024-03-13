use std::collections::HashMap;
use clap::{Parser, Subcommand};
use command::Commands;

#[path = "storage/storage.rs"] mod storage;
#[path = "cli/struct.rs"] mod command;


pub fn get_all(data_map: &mut HashMap<String, String>) {

	for (key, value) in data_map {
		println!("{} => {}", key, value);
	}

}

fn main() {
    
    let mut data_map: HashMap<String, String> = HashMap::new();

    let cli = command::Args::parse();
    
    match cli.cmd {
        Commands::Get => get_all(&mut data_map),
        Commands::Put { key, value } => storage::put(&mut data_map, &key, &value)
    }
}
