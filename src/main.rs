use core::fmt;
use std::io::Read;
use serde::{Serialize};


#[allow(unused)]

&[derive(Debug,Serialize)]

struct Input{
    txid: [u8; 32],
    output_index: u32,
    script_sig: Vec<u8>,
    sequence: u32,
}


//this was replaced by the #[derive(Debug)] above, which automatically generates an implementation of the Debug trait for the Input struct. The Debug trait allows us to format the struct in a way that is useful for debugging purposes, such as printing its fields and values in a readable format. By using #[derive(Debug)], we can easily print instances of the Input struct without having to manually implement the fmt::Debug trait ourselves.

// impl fmt::Debug for Input {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Input")
//             .field("txid", &self.txid)
//             .field("output_index", &self.output_index)
//             .field("script_sig",&self.script_sig)
//             .field("sequence", &self.sequence)
//             .finish()
//     }
// }


fn read_compact_size(transaction_bytes: &mut &[u8])->u64{
    let mut compact_size = [0_u8; 1];
    transaction_bytes.read(&mut compact_size).unwrap();

    match compact_size[0] {
        0..=252 => compact_size[0] as u64,
        253 => {
            let mut buffer = [0; 2];
            transaction_bytes.read(&mut buffer).unwrap();
            u16::from_le_bytes(buffer) as u64
        },
        254 => {
            let mut buffer = [0; 4];
            transaction_bytes.read(&mut buffer).unwrap();
            u32::from_le_bytes(buffer) as u64
        },
        255 => {
            let mut buffer = [0; 8];
            transaction_bytes.read(&mut buffer).unwrap();
            u64::from_le_bytes(buffer)
        },
    }
}


fn read_u32(transaction_bytes: &mut &[u8])-> u32 {
    // let transaction_bytes = hex::decode(transaction_hex).unwrap();

    //TryFrom/Arrays/Slices - here we are converting a slice of bytes (the first 4 bytes of the transaction) into an array of 4 bytes, which is required for the from_le_bytes function to work correctly.
    //The try_from function is used to convert a slice of bytes into an array of bytes. It returns a Result type, which can be either Ok (if the conversion is successful) or Err (if the conversion fails). In this case, we are unwrapping the Result, which means that if the conversion fails, the program will panic and terminate.
    //The from_le_bytes function takes an array of bytes and converts it into a u32 integer, interpreting the bytes in little-endian order. This is necessary because the version field in a Bitcoin transaction is stored in little-endian format.
    // let version_bytes = <[u8; 4]>::try_from(&transaction_bytes[0..4]).unwrap();

    //TryInto/Arrays/Slices- here we are using the try_into method to convert a slice of bytes (the first 4 bytes of the transaction) into an array of 4 bytes. The try_into method is a more concise way to perform this conversion compared to using the try_from function. It also returns a Result type, which we unwrap to get the array of bytes.
    //TryInto is a trait that provides a way to convert between types. In this case, we are using it to convert a slice of bytes into an array of bytes. The try_into method is called on the slice of bytes, and it attempts to convert it into an array of 4 bytes. If the conversion is successful, it returns Ok with the array; if it fails, it returns Err. We unwrap the Result to get the array of bytes directly.
    // let version_bytes: [u8; 4] = (&transaction_bytes[0..4]).try_into().unwrap();

    // let transaction_bytes = hex::decode(transaction_hex).unwrap();

    // let mut bytes_slice = transaction_bytes.as_slice();
    let mut buffer=[0; 4];
    transaction_bytes.read(&mut buffer).unwrap();
    u32::from_le_bytes(buffer)
}

fn read_txid(transaction_bytes: &mut &[u8])-> [u8; 32] {
    let mut buffer = [0; 32];
    transaction_bytes.read(&mut buffer).unwrap();
    buffer
}

fn read_script(transaction_bytes: &mut &[u8])->Vec<u8> {
    let script_size = read_compact_size(transaction_bytes) as usize;
    let mut buffer = vec![0_u8; script_size];
    transaction_bytes.read(&mut buffer).unwrap();
    buffer
}
fn main() {
    let transaction_hex = "010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf72587";
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let mut bytes_slice = transaction_bytes.as_slice();
    let version = read_u32(&mut bytes_slice);
    let input_count = read_compact_size(&mut bytes_slice);
    let mut inputs = vec![];

    for _ in 0..input_count {
        let txid=read_txid(&mut bytes_slice);
        let output_index = read_u32(&mut bytes_slice);
        let script_sig=read_script(&mut bytes_slice);
        let sequence=read_u32(&mut bytes_slice);

        inputs.push(Input{
            txid: txid,
            output_index,
            script_sig,
            sequence,
        });
    }
    let json_inputs = serde_json::to_string_pretty(&inputs).unwrap();
    println!("bytes slice first element: {:?}", bytes_slice.first());
    println!("version: {}", version);
    println!("input count: {}", input_count);
}

#[cfg(test)]

mod test {
    use super::read_compact_size;

    #[test]
    fn test_read_compact_size(){
        let mut bytes = [1_u8].as_slice();
        let count=read_compact_size(&mut bytes);
        assert_eq!(count, 1_u64);

        let mut bytes =[253_u8, 0, 1].as_slice();
        let count=read_compact_size(&mut bytes);
        assert_eq!(count, 256_u64);

        let mut bytes =[254_u8, 0, 0, 0, 1].as_slice();
        let count=read_compact_size(&mut bytes);
        assert_eq!(count, 256_u64.pow(3));

        let mut bytes =[255_u8, 0, 0, 0, 0, 0, 0, 0, 1].as_slice();
        let count=read_compact_size(&mut bytes);
        assert_eq!(count, 256_u64.pow(7));

        let hex = "fd204e";
        let decode = hex::decode(hex).unwrap();
        let mut bytes = decode.as_slice();
        let count=read_compact_size(&mut bytes);
        let expected_count = 20_000_u64;
        assert_eq!(count, expected_count);
    }
}
