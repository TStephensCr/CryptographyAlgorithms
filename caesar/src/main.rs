use std::io;

fn main() {
    println!("Simple CL Caesar Cypher");

    let (offset,plaintext) = input_function();
    
    let cyphertext = String::from(apply_offset(offset,plaintext));
    println!("{}",cyphertext);
}

fn input_function() -> (i32, String) {
    let mut input = String::new();

    println!("Insert offset:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let offset: i32 = input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    input.clear();

    println!("Insert plain text:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let plaintext: String = input
        .to_string();
    
    (offset,plaintext)
}

fn apply_offset(offset: i32, plaintext: String) -> String {
    let mut cyphertext = String::new();

    for c in plaintext.chars() {
        cyphertext.push((c as i32+offset) as u8 as char);
    }
    cyphertext
}