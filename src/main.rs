use std::collections::HashMap;
use clap::{Parser, Subcommand};
use command::Commands;

#[path = "storage/storage.rs"] mod storage;
#[path = "cli/struct.rs"] mod command;


fn main() {
    
    let mut data_map: HashMap<String, String> = HashMap::new();

    let cli = command::Args::parse();
    
    match cli.cmd {
        Commands::GetAll => {
            storage::get_all(&mut data_map);
        },
        Commands::Put { key, value } => storage::put(&mut data_map, &key, &value)
    }
}
