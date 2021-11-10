use std::io;
use std::fs;

fn read(f: String) {
    println!("Reading the file {}...", f);
    let contents = fs::read_to_string(f)
        .expect("Error while reading the file");
    
    println!("File content: \n {}", contents);
}

fn main() {
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).expect("Error while reading filename");
    filename = filename.trim().to_string();
    read(filename);
}
