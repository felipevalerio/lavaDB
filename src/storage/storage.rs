use std::fs::File;
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

	let file_path = Path::new("./db_file").join("db.txt");
	let mut file = File::create(file_path).expect("Erro");
	let encoded: Vec<u8> = bincode::serialize(&data_map).unwrap();
	file.write_all(&encoded)?;
	Ok(())
}


pub fn get_all(data_map: &mut HashMap<String, String>) {

	println!("{}", data_map.len());
	
	for (key, value) in data_map {
		println!("teste");
		println!("{} => {}", key, value);
	}

}
