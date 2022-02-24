use std::io::*;


fn get_stdin() -> String {
    print!("Enter a string: ");
    stdout().flush().unwrap();
    
    let mut input_str: String = String::new();
    stdin().read_line(&mut input_str);

    input_str.to_lowercase()
}


fn main() {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    let  original_str: String = get_stdin();
    println!("{}", original_str);
}
