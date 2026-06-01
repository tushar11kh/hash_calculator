use sha256::digest;

pub fn calculate(x:&mut String)->String{

    return digest(x.as_str());

}