use std::fs;
use std::io::Write;
use std::io::Result;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub fn put(data_map: &mut HashMap<String, String>, key: &String, value: &String) {
	
	data_map.insert(key.to_string(), value.to_string());
	save_to_disk(data_map);
}


fn save_to_disk(data_map: &mut HashMap<String, String>) -> Result<T> {
	let encoded: Vec<u8> = bincode::serialize(&data_map).unwrap();
	let mut file = fs::OpenOptions::new()
								.write(true)
								.open("src/db.bin");
	
	file.write_all(&encoded);
}


pub fn get_all(data_map: &mut HashMap<String, String>) {

	println!("{}", data_map.len());
	
	for (key, value) in data_map {
		println!("teste");
		println!("{} => {}", key, value);
	}

}
