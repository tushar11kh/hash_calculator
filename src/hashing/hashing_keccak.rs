use keccak_hash::{keccak};

pub fn calculate(x: &String){
    let hex_code= keccak(x);
    println!("keccak of {x}: {}",hex::encode(hex_code));
}   