use std::collections::HashMap;
use clap::Parser;
use command::Commands;

#[path = "storage/storage.rs"] mod storage;
#[path = "cli/struct.rs"] mod command;


fn main() {
    
    let cli = command::Args::parse();
    let mut data_map: HashMap<String, String> = HashMap::new();
    
    match cli.cmd {
        Commands::Put { key, value } => storage::put(&mut data_map, &key, &value),
        Commands::Get => storage::get_all(&mut data_map)
    }
}
