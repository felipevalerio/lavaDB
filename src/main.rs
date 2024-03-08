use std::collections::HashMap;
use clap::{Parser, Subcommand};

#[path = "storage/storage.rs"] mod storage;
#[path = "cli/struct.rs"] mod command;


fn main() {
    
    let mut data_map: HashMap<String, String> = HashMap::new();

    let cli = command::Args::parse();
    
    println!("{} => {}", cli.key, cli.value);
}
