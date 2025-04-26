use std::io::{Cursor, Read};

use crate::core::result::{Result, SARError};
use ages_prs::ModernPrsDecoder;
use blowfish::{
    cipher::{generic_array::GenericArray, BlockDecrypt, KeyInit},
    BlowfishLE,
};

const KEY: &[u8] = &[0x09, 0x07, 0xc1, 0x2b];

pub fn decrypt(bytes: &mut [u8]) {
    // It's safe to unwrap because the key is hardcoded and known
    let cipher = BlowfishLE::new_from_slice(KEY).unwrap();
    // decrypt the maximum multiple of 8 bytes
    for block in bytes.chunks_exact_mut(8) {
        let block = GenericArray::from_mut_slice(block);
        cipher.decrypt_block(block);
    }
}

pub fn decompress(bytes: &mut [u8]) -> Result<Box<[u8]>> {
    // XOR every byte in the buffer with 0x95
    bytes.iter_mut().for_each(|b| *b ^= 0x95);
    // decompress the PRS
    let mut decoder = ModernPrsDecoder::new(Cursor::new(&bytes[..]));
    let mut result = Vec::new();
    decoder.read_to_end(&mut result)?;

    Ok(Box::from(result))
}

#[derive(Debug, PartialEq)]
pub enum Compression {
    None,
    Compressed,
}

pub fn validate_format(bytes: &[u8]) -> Result<Compression> {
    if bytes[0..3] != [b's', b'a', b'r'] {
        return Err(SARError::InvalidFileHeader);
    }
    match bytes[3] {
        0x84 => Ok(Compression::Compressed),
        0x04 => Ok(Compression::None),
        _ => Err(SARError::InvalidFileHeader),
    }
}

#[cfg(test)]
mod tests {
    use crate::test::RAW_FILE;

    use super::*;

    #[test]
    fn test_validate_format() {
        let compression = validate_format(RAW_FILE).unwrap();
        assert_eq!(compression, Compression::Compressed);
    }
}
