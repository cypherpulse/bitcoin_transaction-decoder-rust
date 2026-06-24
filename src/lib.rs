use clap::Parser;
use core::fmt;
use std::error::Error;
use std::io::{Error as ioError, Read};
use transaction::{Amount, BitcoinValue, Input, Output, Transaction, Txid};
mod transaction;
use sha2::{Digest, Sha256};


fn read_compact_size(transaction_bytes: &mut &[u8]) -> Result<u64, ioError> {
    let mut compact_size = [0_u8; 1];
    transaction_bytes.read(&mut compact_size)?;

    match compact_size[0] {
        0..=252 => Ok(compact_size[0] as u64),
        253 => {
            let mut buffer = [0; 2];
            transaction_bytes.read(&mut buffer)?;
            Ok(u16::from_le_bytes(buffer) as u64)
        }
        254 => {
            let mut buffer = [0; 4];
            transaction_bytes.read(&mut buffer)?;
            Ok(u32::from_le_bytes(buffer) as u64)
        }
        255 => {
            let mut buffer = [0; 8];
            transaction_bytes.read(&mut buffer)?;
            Ok(u64::from_le_bytes(buffer))
        }
    }
}

fn read_u32(transaction_bytes: &mut &[u8]) -> Result<u32, ioError> {
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
    let mut buffer = [0; 4];
    transaction_bytes.read(&mut buffer)?;
    Ok(u32::from_le_bytes(buffer))
}

fn read_amount(transaction_bytes: &mut &[u8]) -> Result<Amount, ioError> {
    let mut buffer = [0; 8];
    transaction_bytes.read(&mut buffer)?;
    Ok(Amount::from_sat(u64::from_le_bytes(buffer)))
}

fn read_txid(transaction_bytes: &mut &[u8]) -> Result<Txid, ioError> {
    let mut buffer = [0; 32];
    transaction_bytes.read(&mut buffer)?;
    Ok(Txid::from_bytes(buffer))
}

fn read_script(transaction_bytes: &mut &[u8]) -> Result<String, ioError> {
    let script_size = read_compact_size(transaction_bytes)? as usize;
    let mut buffer = vec![0_u8; script_size];
    transaction_bytes.read(&mut buffer)?;
    Ok(hex::encode(buffer))
}

fn hash_raw_transaction(raw_transaction: &[u8]) -> Txid {
    let mut hasher = Sha256::new();
    hasher.update(&raw_transaction);
    let hash1 = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(&hash1);
    let hash2 = hasher.finalize();

    Txid::from_bytes(hash2.into())
}

pub fn decode(transaction_hex: String) -> Result<String, Box<dyn Error>> {
    let transaction_bytes =
        hex::decode(transaction_hex).map_err(|e| format!("Hex decode error: {}", e))?;
    let mut bytes_slice = transaction_bytes.as_slice();
    let version = read_u32(&mut bytes_slice)?;
    let input_count = read_compact_size(&mut bytes_slice)?;
    let mut inputs = vec![];

    for _ in 0..input_count {
        let txid = read_txid(&mut bytes_slice)?;
        let output_index = read_u32(&mut bytes_slice)?;
        let script_sig = read_script(&mut bytes_slice)?;
        let sequence = read_u32(&mut bytes_slice)?;

        inputs.push(Input {
            txid: txid,
            output_index,
            script_sig,
            sequence,
        });
    }

    let output_count = read_compact_size(&mut bytes_slice)?;
    let mut outputs = vec![];

    for _ in 0..output_count {
        let amount = read_amount(&mut bytes_slice)?;
        let script_pubkey = read_script(&mut bytes_slice)?;

        outputs.push(Output {
            amount,
            script_pubkey,
        });
    }

    let lock_time = read_u32(&mut bytes_slice)?;
    let transaction_id = hash_raw_transaction(&transaction_bytes);

    let transaction = Transaction {
        transaction_id,
        version,
        inputs,
        outputs,
        lock_time,
    };

    Ok(serde_json::to_string_pretty(&transaction)?)
}


#[cfg(test)]

mod test {
    use super::read_compact_size;
    use super::Error;

    #[test]
    fn test_read_compact_size() -> Result<(), Box<dyn Error>> {
        let mut bytes = [1_u8].as_slice();
        let count = read_compact_size(&mut bytes)?;
        assert_eq!(count, 1_u64);

        let mut bytes = [253_u8, 0, 1].as_slice();
        let count = read_compact_size(&mut bytes)?;
        assert_eq!(count, 256_u64);

        let mut bytes = [254_u8, 0, 0, 0, 1].as_slice();
        let count = read_compact_size(&mut bytes)?;
        assert_eq!(count, 256_u64.pow(3));

        let mut bytes = [255_u8, 0, 0, 0, 0, 0, 0, 0, 1].as_slice();
        let count = read_compact_size(&mut bytes)?;
        assert_eq!(count, 256_u64.pow(7));

        let hex = "fd204e";
        let decode = hex::decode(hex)?;
        let mut bytes = decode.as_slice();
        let count = read_compact_size(&mut bytes)?;
        let expected_count = 20_000_u64;
        assert_eq!(count, expected_count);

    Ok(())
    }
}
