pub fn calculate(x: &mut String)->String{


    blake3::hash(x.as_bytes()).to_string()

}