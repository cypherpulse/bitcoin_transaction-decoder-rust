#![allow(unused)]


fn read_version(transaction_hex: &str)-> u32 {
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let version_bytes = &transaction_bytes[0..4];
    println!("version bytes: {:?}", version_bytes);

    1
}

fn main() {
    let version = read_version("0000000000001c47896df6c74aa8351f371feef54d0b9a90516d74ebe4d0d828");
    println!("version: {}",version);
   
}
