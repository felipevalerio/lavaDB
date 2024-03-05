use std::io;
use std::collections::HashMap;
#[path = "storage/storage.rs"] mod storage;

fn main() {
    
    let mut choice: i32 = 0;
    let mut key: String = String::new();
    let mut value: String = String::new();
    let mut input: String = String::new();
    let mut data_map: HashMap<String, String> = HashMap::new();

    loop {
        println!("1) Insert");
        println!("2) Update");
        println!("3) Delete");
        println!("4) Get all");
        println!("0) Exit"); 
        println!("Insert your option");
        input.clear();
        io::stdin().read_line(&mut input);

        choice = input.trim().parse().unwrap();

        match choice {
            1 => {
                println!("Key:");
                io::stdin().read_line(&mut key);

                println!("Value:");
                io::stdin().read_line(&mut value);

                storage::put(&mut data_map, &key, &value);
            }
                    
            //2 => update(),
            //3 => del(),
            4 => storage::get_all(&mut data_map),
            0 => break,
            _=> println!("This option does not exist")   
        }
    }
}
