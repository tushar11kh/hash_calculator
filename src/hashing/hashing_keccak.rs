use keccak_hash::{keccak};

pub fn calculate(x: &mut String)->String {
    let hex_code= keccak(x);
    hex::encode(hex_code)
}   