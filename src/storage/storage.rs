use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::io::Result;
use std::collections::HashMap;


pub fn put(data_map: &mut HashMap<String, String>, key: &String, value: &String) {
	
	data_map.insert(key.to_string(), value.to_string());


	let save_to_disk = save_to_disk(data_map);
	println!("{:?}", save_to_disk)
}


fn save_to_disk(data_map: &mut HashMap<String, String>) -> Result<()> {

	let exists: bool;
	let file_path = Path::new("./db_file").join("db.txt");
	exists = Path::new("./db_file/db.txt").exists();

	if exists == true {
		
		let mut file = OpenOptions::new()
					.append(true)
					.open(file_path)
					.expect("cannot open this file");
		
		let encoded: Vec<u8> = bincode::serialize(&data_map).unwrap();
		file.write_all(&encoded)?;
		Ok(())
	}
	else {

		let mut file = File::create(file_path).expect("Erro");
		let encoded: Vec<u8> = bincode::serialize(&data_map).unwrap();
		file.write_all(&encoded)?;
		Ok(())
	}

}


pub fn get_all(data_map: &mut HashMap<String, String>) {

	println!("{}", data_map.len());
	
	for (key, value) in data_map {
		println!("teste");
		println!("{} => {}", key, value);
	}

}
