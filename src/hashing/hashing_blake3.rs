pub fn calculate(x: &String){


    let y =blake3::hash(x.as_bytes()).to_string();
    println!("Blake3 of {x}: {y}")

}