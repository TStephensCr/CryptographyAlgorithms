use num_bigint::{BigUint};
use num_primes::{Generator};
use num_traits::{One, Zero};
use std::io::{self, Write};
use std::time::Instant;
use rand::thread_rng;
use std::fs;

fn main() {

    println!("RSA implementation");
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

    let mut plaintext = String::new();
    let mut cyphertext = BigUint::default();

    match choice {
        1 => {
            let (e, d, n, _p) = generate_keys(2048);
            plaintext = input_keyboard();
            println!("ENCRYPTING \n Starting timer...");
            let start_time = Instant::now();
            cyphertext = encrypt(&plaintext, &e, &n);
            let end_time = start_time.elapsed();//Creating second variable saves 0.00000025 seconds, as opposed to printing start_time.elapsed() after the cyphertext
            println!("Encrypted message: {} \n Elapsed time: {:?}",cyphertext, end_time);
        }
        2 => {
            let (e, d, n, _p) = generate_keys(2048);
            plaintext = match input_file() {
                Ok(value) => value,
                Err(e) => {
                    println!("Error occurred: {}", e); 
                    String::new()
                },
            };
            println!("ENCRYPTING \n Starting timer...");
            let start_time = Instant::now();
            cyphertext = encrypt(&plaintext, &e, &n);
            let end_time = start_time.elapsed();
            println!("Encrypted message: {} \n Elapsed time: {:?}",cyphertext, end_time);
        }
        3 => {
            println!("Insert key(e or d, and n):");
            let key: BigUint = input_biguint_value();
            let n: BigUint = input_biguint_value();
            cyphertext = input_biguint_value();
            println!("DECRYPTING \n Starting timer...");
            let start_time = Instant::now();
            plaintext = decrypt(&cyphertext, &key, &n);
            let end_time = start_time.elapsed();
            println!("Decrypted message: {} \n Elapsed time: {:?}",plaintext, end_time);
        }
        4 => {
            println!("Insert key(e or d, and n):");
            let key: BigUint = input_biguint_value();
            let n: BigUint = input_biguint_value();
            let mut cleaned_value = String::new();
            cyphertext = match input_file() {
                Ok(value) => {
                    cleaned_value = value.trim().replace("\n", "").replace("\r", "");
                    match cleaned_value.parse::<BigUint>() {
                    Ok(big_uint) => big_uint,
                    Err(e) => {
                        println!("Failed to convert file to cyphertext: {}", e);
                        BigUint::default()
                    }
                    }
                }
                Err(e) => {
                    println!("Error occurred: {}", e);
                    BigUint::default()
                }
            };
            println!("DECRYPTING \n Starting timer...");
            let start_time = Instant::now();
            plaintext = decrypt(&cyphertext, &key, &n);
            let end_time = start_time.elapsed();
            println!("Decrypted message: {} \n Elapsed time: {:?}",plaintext, end_time);
        }
        5 => {
            println!("Goodbye!");
        }
        _ => {
            println!("Invalid choice. Please select 1 to 5.");
        }
    }


    
    
}

fn input_keyboard() -> String {//measure times?
    let mut input = String::new();

    println!("Insert plain text:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let plaintext: String = input
        .trim()
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

fn input_biguint_value() -> BigUint {//measure times?
    let mut input = String::new();

    println!("Insert large number:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read number");

    let value: BigUint = input.trim().parse::<BigUint>().expect("Invalid number");
    
    value
}


fn generate_keys(bits: usize) -> (BigUint,BigUint,BigUint,BigUint) {
    println!("GENERATING KEYS \n Starting timer...");
    let start_time = Instant::now();

    let mut rng = thread_rng();

    let p = BigUint::from_bytes_be(&(Generator::new_prime(bits / 2)).to_bytes_be());

    let q = BigUint::from_bytes_be(&(Generator::new_prime(bits / 2)).to_bytes_be());

    let n = &p * &q;
    
    let totient = (&p - 1u32) * (&q - 1u32);

    let e = BigUint::from(65537u32);

    let d = e.modinv(&totient).expect("Failed to compute modular inverse.");

    let end_time = start_time.elapsed();

    println!("---Generated values---");
    println!("p    {}",p);
    println!("q    {}",q);
    println!("e    {}",e);
    println!("d    {}",d);
    println!("n    {}",n);
    println!("Elapsed time: {:?}", end_time);

    (e, d, n, p)
}

fn encrypt(plaintext: &str, key: &BigUint, n: &BigUint) -> BigUint{
    let m = BigUint::from_bytes_be(plaintext.as_bytes());
    m.modpow(key, n)
}

fn decrypt(cyphertext: &BigUint, key: &BigUint, n: &BigUint) -> String{
    let m = cyphertext.modpow(key, n);
    String::from_utf8(m.to_bytes_be()).expect("Failed to convert to string")
}

