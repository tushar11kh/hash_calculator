use std::{io::{self, Write}};

mod hashing;
fn main() {
    
    println!("Hello there! this tool gives you hashed values of your input.");
    println!("At Present there are 3 methods in this program to do it by.\n");
    print!("Write in your input here: ");
    io::stdout().flush().expect("Failed to flush");

    let mut x:String = String::new();

    io::stdin().read_line(&mut x).expect("Failed to get your input"); 

    println!("Select which Hashiang you want:");
    
    let output = hashing::hashing_sha256::calculate(&mut x);
    let output_2 = hashing::hashing_blake3::calculate(&mut x);
    let output_3 = hashing::hashing_keccak::calculate(&mut x);

    println!("Output: {output}");
    println!("Output: {output_2}");
    println!("Output: {output_3}");

}
