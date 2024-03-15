use std::collections::HashMap;

pub fn put(data_map: &mut HashMap<String, String>, key: &String, value: &String) {
	
	data_map.insert(key.to_string(), value.to_string());

}


pub fn get_all(data_map: &mut HashMap<String, String>) {

	println!("{}", data_map.len());
	
	for (key, value) in data_map {
		println!("{} => {}", key, value);
	}

}
