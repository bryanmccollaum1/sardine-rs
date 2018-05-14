use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std;
use std::io::Read;
use std::io::Write;

use Result;
use message_types::SrdMessage;

use srd::getrandom;
use rand::{EntropyRng, RngCore};

mod basic_blob;
mod logon_blob;
pub use self::basic_blob::BasicBlob;
pub use self::logon_blob::LogonBlob;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct SrdBlob {
    blob_type: String,
    data: Vec<u8>,
}

#[cfg(feature="wasm")]
#[wasm_bindgen]
impl SrdBlob {
    pub fn new_logon(username: &str, password: &str) -> SrdBlob {
        let logon = LogonBlob::new(username, password);
        let mut data = Vec::new();
        logon.write_to(&mut data).unwrap();
        SrdBlob {
            blob_type: "Logon".to_string(),
            data
        }
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl SrdBlob {
    pub fn new(blob_type: &str, data: &[u8]) -> SrdBlob {
        SrdBlob {
            blob_type: blob_type.to_string(),
            data: Vec::from(data),
        }
    }

    pub fn blob_type_copy(&self) -> String {
        self.blob_type.clone()
    }

    pub fn data_copy(&self) -> Vec<u8> {
        self.data.clone()
    }
}

impl SrdBlob {
    pub fn blob_type(&self) -> &str {
        &self.blob_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl SrdMessage for SrdBlob {
    fn read_from(buffer: &mut std::io::Cursor<&[u8]>) -> Result<Self>
    where
        Self: Sized,
    {
        let type_size = buffer.read_u16::<LittleEndian>()?;
        let type_padding = buffer.read_u16::<LittleEndian>()?;
        let data_size = buffer.read_u16::<LittleEndian>()?;
        let data_padding = buffer.read_u16::<LittleEndian>()?;

        let length = type_size - 1;
        let mut string = vec![0u8; length as usize];
        buffer.read_exact(&mut string)?;
        buffer.read_u8()?; // null terminator
        let mut padding = vec![0u8; type_padding as usize];
        buffer.read_exact(&mut padding)?;
        let blob_type: String = string.iter().map(|c| *c as char).collect();

        let mut data = vec![0u8; data_size as usize];
        buffer.read_exact(&mut data)?;
        let mut padding = vec![0u8; data_padding as usize];
        buffer.read_exact(&mut padding)?;

        Ok(SrdBlob { blob_type, data })
    }

    fn write_to(&self, buffer: &mut Vec<u8>) -> Result<()> {
        let mut rng = EntropyRng::new();

        let type_size = self.blob_type.len() + 1;
        let type_padding = 16 - (type_size + 8) % 16;
        let data_size = self.data.len();
        let data_padding = 16 - (data_size % 16);

        buffer.write_u16::<LittleEndian>(type_size as u16)?;
        buffer.write_u16::<LittleEndian>(type_padding as u16)?;
        buffer.write_u16::<LittleEndian>(data_size as u16)?;
        buffer.write_u16::<LittleEndian>(data_padding as u16)?;

        buffer.write_all(&self.blob_type.chars().map(|c| c as u8).collect::<Vec<u8>>())?;
        buffer.write_u8(0u8)?;

        let mut padding = vec![0u8; type_padding];
        if cfg!(feature = "wasm") {
            padding = getrandom(padding);
        } else {
            rng.try_fill_bytes(&mut padding)?;
        }
        buffer.write_all(&padding)?;

        buffer.write_all(&self.data)?;

        let mut padding = vec![0u8; data_padding];
        if cfg!(feature = "wasm") {
            padding = getrandom(padding);
        } else {
            rng.try_fill_bytes(&mut padding)?;
        }
        buffer.write_all(&padding)?;

        Ok(())
    }
}

pub trait Blob: SrdMessage {
    fn blob_type() -> &'static str;
}

#[cfg(test)]
mod test {
    use message_types::SrdMessage;
    use srd_blob::SrdBlob;
    use std;

    #[test]
    fn blob_encoding() {
        let srd_blob = SrdBlob::new("Basic", &vec![0, 1, 2, 3]);

        let mut buffer: Vec<u8> = Vec::new();
        match srd_blob.write_to(&mut buffer) {
            Ok(_) => (),
            Err(_) => assert!(false),
        };

        let mut cursor = std::io::Cursor::new(buffer.as_slice());
        match SrdBlob::read_from(&mut cursor) {
            Ok(blob) => {
                assert_eq!(blob, srd_blob);
            }
            Err(_) => assert!(false),
        };
    }
}
