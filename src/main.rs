#![allow(unused)]


fn read_version(transaction_hex: &str)-> u32 {
    let transaction_bytes = hex::decode(transaction_hex).unwrap();

    //TryFrom/Arrays/Slices - here we are converting a slice of bytes (the first 4 bytes of the transaction) into an array of 4 bytes, which is required for the from_le_bytes function to work correctly.
    //The try_from function is used to convert a slice of bytes into an array of bytes. It returns a Result type, which can be either Ok (if the conversion is successful) or Err (if the conversion fails). In this case, we are unwrapping the Result, which means that if the conversion fails, the program will panic and terminate.
    //The from_le_bytes function takes an array of bytes and converts it into a u32 integer, interpreting the bytes in little-endian order. This is necessary because the version field in a Bitcoin transaction is stored in little-endian format.
    // let version_bytes = <[u8; 4]>::try_from(&transaction_bytes[0..4]).unwrap();

    //TryInto/Arrays/Slices- here we are using the try_into method to convert a slice of bytes (the first 4 bytes of the transaction) into an array of 4 bytes. The try_into method is a more concise way to perform this conversion compared to using the try_from function. It also returns a Result type, which we unwrap to get the array of bytes.
    //TryInto is a trait that provides a way to convert between types. In this case, we are using it to convert a slice of bytes into an array of bytes. The try_into method is called on the slice of bytes, and it attempts to convert it into an array of 4 bytes. If the conversion is successful, it returns Ok with the array; if it fails, it returns Err. We unwrap the Result to get the array of bytes directly.
    let version_bytes: [u8; 4] = (&transaction_bytes[0..4]).try_into().unwrap();

   u32::from_le_bytes(version_bytes)
}

fn main() {
    let version = read_version("000000000001c47896df6c74aa8351f371feef54d0b9a90516d74ebe4d0d828");
    println!("version: {}",version);
   
}
