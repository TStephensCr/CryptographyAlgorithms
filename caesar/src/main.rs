use std::io;
use std::fs;

fn main() {
    println!("Simple CL Caesar Cypher");
    println!("Choose an option:");
    println!("1. Encrypt input string");
    println!("2. Encrypt text file");
    println!("3. Decrypt input string");
    println!("4. Decrypt text file");
    println!("5. Exit");

    // Read user input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Parse input to a number
    let choice: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

    let offset = input_offset();
    let mut plaintext = String::new();

    // Perform actions based on the user's choice
    match choice {
        1 | 3 => {
            plaintext = input_keyboard();
        }
        2 | 4 => {
            plaintext = match input_file() {
                Ok(value) => value,
                Err(e) => {
                    println!("Error occurred: {}", e); 
                    String::new()
                },
            };
        }
        5 => {
            println!("Goodbye!");
        }
        _ => {
            println!("Invalid choice. Please select 1 to 5.");
        }
    }

    let mut cyphertext = String::new();
    
    match choice {
        1 | 2 => cyphertext = String::from(apply_offset(offset,plaintext)),
        3 | 4 => cyphertext = String::from(apply_offset(-offset,plaintext)),
        _ => return,
    }

    println!("Output:");
    println!("{}",cyphertext);
}

fn input_offset() -> i32 {
    let mut input = String::new();

    println!("Insert offset:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let offset: i32 = input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    offset
}

fn input_keyboard() -> String {//measure times?
    let mut input = String::new();

    println!("Insert plain text:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let plaintext: String = input
        .to_string();
    
    plaintext
}

fn input_file() -> Result<String, std::io::Error> {
    let mut filename = String::new();

    println!("Insert text file name:");
    io::stdin()
        .read_line(&mut filename)
        .expect("Failed to read line");

    let filename = filename.trim();

    let contents = fs::read_to_string(filename)?;

    Ok(contents)
}

fn apply_offset(offset: i32, plaintext: String) -> String {
    let mut cyphertext = String::new();

    for c in plaintext.chars() {
        cyphertext.push((c as i32+offset) as u8 as char);
    }
    cyphertext
}
