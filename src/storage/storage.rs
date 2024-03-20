use std::collections::HashMap;
use bincode::{Decode, Encode};

pub fn put(data_map: &mut HashMap<String, String>, key: &String, value: &String) {
	
	data_map.insert(key.to_string(), value.to_string());

	let encoded: Vec<u8> = bincode::encode_to_vec(&data_map).unwrap();
}


pub fn get_all(data_map: &mut HashMap<String, String>) {

	println!("{}", data_map.len());
	
	for (key, value) in data_map {
		println!("teste");
		println!("{} => {}", key, value);
	}

}
