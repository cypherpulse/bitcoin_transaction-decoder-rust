#![allow(unused)]


fn read_version(transaction_hex: &str)-> u32 {
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let version_bytes =<[u8; 4]>::try_from( &transaction_bytes[0..4]);

    let version = u32::from_le_bytes(version_bytes);
    println!("version bytes: {:?}", version_bytes);
    1
}

fn main() {
    let version = read_version("0000000000001c47896df6c74aa8351f371feef54d0b9a90516d74ebe4d0d828");
    println!("version: {}",version);
   
}
