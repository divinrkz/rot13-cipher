use std::io::*;
mod lib;
use lib::rot13;

fn get_stdin() -> String {
    print!("Enter a string: ");
    stdout().flush().unwrap();
    
    let mut input_str: String = String::new();
    stdin().read_line(&mut input_str)
            .ok()
            .expect("Failed to read input");

    input_str
}


fn main() {

    let original_str: String = get_stdin();

    println!("ROT13 Cipher: {}", rot13(original_str));

}
