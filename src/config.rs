extern crate dirs;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::panic;



// Set value to config file
pub fn set(key: &str, value: &str){
     // Create a path to the desired file
    let home = dirs::home_dir().unwrap();
    let file_path = &home.join(".kctlconf");
    let path = Path::new(file_path);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let content: &str = &[key, "=", value].concat();

    // Write 
    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("Sucessfully set namespace {}", value),
    }
}

// Get value to config file
pub fn get() -> String {
     // Create a path to the desired file
    let home = dirs::home_dir().unwrap();
    let file_path = &home.join(".kctlconf");
    let path = Path::new(file_path);
    let display = path.display();

    if !path.exists(){
        return String::new();
    }

    // Open the path in read-only mode
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string
    let mut s = String::new();
    file.read_to_string(&mut s).expect("Couldn't read value from config!");
    let val: Vec<&str> = s.split("=").collect();
    String::from(val[1])
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_set(){
        set("KCTL_NAMESPACE", "food");
        assert_eq!(get(), "food");
    }
}