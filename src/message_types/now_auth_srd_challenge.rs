use std;
use std::io::Read;
use std::io::Write;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use message_types::NowAuthSrdMessage;
use message_types::expand_start;
use message_types::now_auth_srd_id::NOW_AUTH_SRD_CHALLENGE_ID;
use Result;

pub struct NowAuthSrdChallenge {
    pub packet_type: u16,
    pub flags: u16,
    pub key_size: u16,
    pub generator: Vec<u8>,
    pub prime: Vec<u8>,
    pub public_key: Vec<u8>,
    pub nonce: [u8; 32],
}

impl NowAuthSrdMessage for NowAuthSrdChallenge {
    fn read_from(buffer: &mut std::io::Cursor<Vec<u8>>) -> Result<Self>
    where
        Self: Sized,
    {
        let packet_type = buffer.read_u16::<LittleEndian>()?;
        let flags = buffer.read_u16::<LittleEndian>()?;
        let key_size = buffer.read_u16::<LittleEndian>()?;

        let mut generator = vec![0u8; 2];
        let mut prime = vec![0u8; key_size as usize];
        let mut public_key = vec![0u8; key_size as usize];
        buffer.read_exact(&mut generator)?;
        buffer.read_exact(&mut prime)?;
        buffer.read_exact(&mut public_key)?;

        let mut nonce = [0u8; 32];
        buffer.read_exact(&mut nonce)?;

        Ok(NowAuthSrdChallenge {
            packet_type,
            flags,
            key_size,
            generator,
            prime,
            public_key,
            nonce,
        })
    }

    fn write_to(&self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.write_u16::<LittleEndian>(self.packet_type)?;
        buffer.write_u16::<LittleEndian>(self.flags)?;
        buffer.write_u16::<LittleEndian>(self.key_size)?;
        buffer.write_all(&self.generator)?;
        buffer.write_all(&self.prime)?;
        buffer.write_all(&self.public_key)?;
        buffer.write_all(&self.nonce)?;

        Ok(())
    }

    fn get_size(&self) -> usize {
        40usize + self.key_size as usize * 2
    }

    fn get_id(&self) -> u16 {
        NOW_AUTH_SRD_CHALLENGE_ID
    }
}

impl NowAuthSrdChallenge {
    pub fn new(
        key_size: u16,
        mut generator: Vec<u8>,
        mut prime: Vec<u8>,
        mut public_key: Vec<u8>,
        nonce: [u8; 32],
    ) -> NowAuthSrdChallenge {
        expand_start(&mut generator, 2);
        expand_start(&mut prime, (key_size / 8) as usize);
        expand_start(&mut public_key, (key_size / 8) as usize);

        NowAuthSrdChallenge {
            packet_type: NOW_AUTH_SRD_CHALLENGE_ID,
            flags: 0,
            key_size,
            generator,
            prime,
            public_key,
            nonce,
        }
    }
}
