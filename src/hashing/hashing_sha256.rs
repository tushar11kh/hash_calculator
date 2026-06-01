use sha256::digest;

pub fn calculate(x:&String){

    let y = digest(x.as_str());

    println!("Sha256 of {x}: {y}")

}