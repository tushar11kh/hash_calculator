use std::io::{self, Write};

mod hashing;
mod hash_method;

fn main() {
    
    println!("\nHello there! this tool gives you hashed values of your input.");
    println!("At Present there are 3 methods in this program to do it by.\n");
    print!("Write in your input here: ");
    io::stdout().flush().expect("Failed to flush");

    let mut x:String = String::new();

    io::stdin().read_line(&mut x).expect("Failed to get your input"); 

    println!("Select which Hashing you want:");
    println!("1) Sha256");
    println!("2) Blake3");
    println!("3) Keccak");
    print!("Write your choice here: ");
    io::stdout().flush().expect("Flush fialed");

    let mut choice = String::new();

    io::stdin().read_line(&mut choice).expect("Failed to get your choice");

    println!();

    let method = match choice.trim() {
        "1"=> hash_method::HashMethod::Sha256,
        "2"=> hash_method::HashMethod::Blake3,
        "3"=>hash_method::HashMethod::Keccak256,
        _ =>{
            println!("Invalid choice");
            return;
        }
    };

    match method {
        hash_method::HashMethod::Sha256=>hashing::hashing_sha256::calculate(&x),
        hash_method::HashMethod::Blake3=>hashing::hashing_blake3::calculate(&x),
        hash_method::HashMethod::Keccak256=>hashing::hashing_keccak::calculate(&x),
    }

}
