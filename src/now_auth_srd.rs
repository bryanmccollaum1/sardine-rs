use std;
use std::io::{Read, Write, Error};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

/* https://tools.ietf.org/html/rfc3526 */

pub const NOW_AUTH_SRD_NEGOTIATE_ID: u8 = 1;
pub const NOW_AUTH_SRD_CHALLENGE_ID: u8 = 2;
pub const NOW_AUTH_SRD_RESPONSE_ID: u8 = 3;
pub const NOW_AUTH_SRD_CONFIRM_ID: u8 = 4;
pub const NOW_AUTH_SRD_DELEGATE_ID: u8 = 5;
pub const NOW_AUTH_SRD_RESULT_ID: u8 = 6;

static SRD_DH_PARAMS: [SrdDhParams; 4] =
[
    SrdDhParams{
        p_data:
        b"\xAC\x6B\xDB\x41\x32\x4A\x9A\x9B\xF1\x66\xDE\x5E\x13\x89\x58\x2F\
		\xAF\x72\xB6\x65\x19\x87\xEE\x07\xFCl\x31\x92\x94\x3D\xB5\x60\x50\
		\xA3\x73\x29\xCB\xB4\xA0\x99\xED\x81\x93\xE0\x75\x77\x67\xA1\x3D\
		\xD5\x23\x12\xAB\x4B\x03\x31\x0D\xCD\x7F\x48\xA9\xDA\x04\xFD\x50\
		\xE8\x08\x39\x69\xED\xB7\x67\xB0\xCF\x60\x95\x17\x9A\x16\x3A\xB3\
		\x66\x1A\x05\xFB\xD5\xFA\xAA\xE8\x29\x18\xA9\x96\x2F\x0B\x93\xB8\
		\x55\xF9\x79\x93\xEC\x97\x5E\xEA\xA8\x0D\x74\x0A\xDB\xF4\xFF\x74\
		\x73\x59\xD0\x41\xD5\xC3\x3E\xA7\x1D\x28\x1E\x44\x6B\x14\x77\x3B\
		\xCA\x97\xB4\x3A\x23\xFB\x80\x16\x76\xBD\x20\x7A\x43\x6C\x64\x81\
		\xF1\xD2\xB9\x07\x87\x17\x46\x1A\x5B\x9D\x32\xE6\x88\xF8\x77\x48\
		\x54\x45\x23\xB5\x24\xB0\xD5\x7D\x5E\xA7\x7A\x27\x75\xD2\xEC\xFA\
		\x03\x2C\xFB\xDB\xF5\x2F\xB3\x78\x61\x60\x27\x90\x04\xE5\x7A\xE6\
		\xAF\x87\x4E\x73\x03\xCE\x53\x29\x9C\xCC\x04\x1C\x7B\xC3\x08\xD8\
		\x2A\x56\x98\xF3\xA8\xD0\xC3\x82\x71\xAE\x35\xF8\xE9\xDB\xFB\xB6\
		\x94\xB5\xC8\x03\xD8\x9F\x7A\xE4\x35\xDE\x23\x6D\x52\x5F\x54\x75\
		\x9B\x65\xE3\x72\xFC\xD6\x8E\xF2\x0F\xA7\x11\x1F\x9E\x4A\xFF\x73",
        g_data: b"\x00\x02"
    },
    SrdDhParams{
        p_data:
        b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC9\x0F\xDA\xA2\x21\x68\xC2\x34\
		\xC4\xC6\x62\x8B\x80\xDC\x1C\xD1\x29\x02\x4E\x08\x8A\x67\xCC\x74\
		\x02\x0B\xBE\xA6\x3B\x13\x9B\x22\x51\x4A\x08\x79\x8E\x34\x04\xDD\
		\xEF\x95\x19\xB3\xCD\x3A\x43\x1B\x30\x2B\x0A\x6D\xF2\x5F\x14\x37\
		\x4F\xE1\x35\x6D\x6D\x51\xC2\x45\xE4\x85\xB5\x76\x62\x5E\x7E\xC6\
		\xF4\x4C\x42\xE9\xA6\x37\xED\x6B\x0B\xFF\x5C\xB6\xF4\x06\xB7\xED\
		\xEE\x38\x6B\xFB\x5A\x89\x9F\xA5\xAE\x9F\x24\x11\x7C\x4B\x1F\xE6\
		\x49\x28\x66\x51\xEC\xE4\x5B\x3D\xC2\x00\x7C\xB8\xA1\x63\xBF\x05\
		\x98\xDA\x48\x36\x1C\x55\xD3\x9A\x69\x16\x3F\xA8\xFD\x24\xCF\x5F\
		\x83\x65\x5D\x23\xDC\xA3\xAD\x96\x1C\x62\xF3\x56\x20\x85\x52\xBB\
		\x9E\xD5\x29\x07\x70\x96\x96\x6D\x67\x0C\x35\x4E\x4A\xBC\x98\x04\
		\xF1\x74\x6C\x08\xCA\x18\x21\x7C\x32\x90\x5E\x46\x2E\x36\xCE\x3B\
		\xE3\x9E\x77\x2C\x18\x0E\x86\x03\x9B\x27\x83\xA2\xEC\x07\xA2\x8F\
		\xB5\xC5\x5D\xF0\x6F\x4C\x52\xC9\xDE\x2B\xCB\xF6\x95\x58\x17\x18\
		\x39\x95\x49\x7C\xEA\x95\x6A\xE5\x15\xD2\x26\x18\x98\xFA\x05\x10\
		\x15\x72\x8E\x5A\x8A\xAA\xC4\x2D\xAD\x33\x17\x0D\x04\x50\x7A\x33\
		\xA8\x55\x21\xAB\xDF\x1C\xBA\x64\xEC\xFB\x85\x04\x58\xDB\xEF\x0A\
		\x8A\xEA\x71\x57\x5D\x06\x0C\x7D\xB3\x97\x0F\x85\xA6\xE1\xE4\xC7\
		\xAB\xF5\xAE\x8C\xDB\x09\x33\xD7\x1E\x8C\x94\xE0\x4A\x25\x61\x9D\
		\xCE\xE3\xD2\x26\x1A\xD2\xEE\x6B\xF1\x2F\xFA\x06\xD9\x8A\x08\x64\
		\xD8\x76\x02\x73\x3E\xC8\x6A\x64\x52\x1F\x2B\x18\x17\x7B\x20\x0C\
		\xBB\xE1\x17\x57\x7A\x61\x5D\x6C\x77\x09\x88\xC0\xBA\xD9\x46\xE2\
		\x08\xE2\x4F\xA0\x74\xE5\xAB\x31\x43\xDB\x5B\xFC\xE0\xFD\x10\x8E\
		\x4B\x82\xD1\x20\xA9\x21\x08\x01\x1A\x72\x3C\x12\xA7\x87\xE6\xD7\
		\x88\x71\x9A\x10\xBD\xBA\x5B\x26\x99\xC3\x27\x18\x6A\xF4\xE2\x3C\
		\x1A\x94\x68\x34\xB6\x15\x0B\xDA\x25\x83\xE9\xCA\x2A\xD4\x4C\xE8\
		\xDB\xBB\xC2\xDB\x04\xDE\x8E\xF9\x2E\x8E\xFC\x14\x1F\xBE\xCA\xA6\
		\x28\x7C\x59\x47\x4E\x6B\xC0\x5D\x99\xB2\x96\x4F\xA0\x90\xC3\xA2\
		\x23\x3B\xA1\x86\x51\x5B\xE7\xED\x1F\x61\x29\x70\xCE\xE2\xD7\xAF\
		\xB8\x1B\xDD\x76\x21\x70\x48\x1C\xD0\x06\x91\x27\xD5\xB0\x5A\xA9\
		\x93\xB4\xEA\x98\x8D\x8F\xDD\xC1\x86\xFF\xB7\xDC\x90\xA6\xC0\x8F\
		\x4D\xF4\x35\xC9\x34\x06\x31\x99\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF",
		g_data: b"\x00\x05"
    },
    SrdDhParams{
        p_data:
        b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC9\x0F\xDA\xA2\x21\x68\xC2\x34\
		\xC4\xC6\x62\x8B\x80\xDC\x1C\xD1\x29\x02\x4E\x08\x8A\x67\xCC\x74\
		\x02\x0B\xBE\xA6\x3B\x13\x9B\x22\x51\x4A\x08\x79\x8E\x34\x04\xDD\
		\xEF\x95\x19\xB3\xCD\x3A\x43\x1B\x30\x2B\x0A\x6D\xF2\x5F\x14\x37\
		\x4F\xE1\x35\x6D\x6D\x51\xC2\x45\xE4\x85\xB5\x76\x62\x5E\x7E\xC6\
		\xF4\x4C\x42\xE9\xA6\x37\xED\x6B\x0B\xFF\x5C\xB6\xF4\x06\xB7\xED\
		\xEE\x38\x6B\xFB\x5A\x89\x9F\xA5\xAE\x9F\x24\x11\x7C\x4B\x1F\xE6\
		\x49\x28\x66\x51\xEC\xE4\x5B\x3D\xC2\x00\x7C\xB8\xA1\x63\xBF\x05\
		\x98\xDA\x48\x36\x1C\x55\xD3\x9A\x69\x16\x3F\xA8\xFD\x24\xCF\x5F\
		\x83\x65\x5D\x23\xDC\xA3\xAD\x96\x1C\x62\xF3\x56\x20\x85\x52\xBB\
		\x9E\xD5\x29\x07\x70\x96\x96\x6D\x67\x0C\x35\x4E\x4A\xBC\x98\x04\
		\xF1\x74\x6C\x08\xCA\x18\x21\x7C\x32\x90\x5E\x46\x2E\x36\xCE\x3B\
		\xE3\x9E\x77\x2C\x18\x0E\x86\x03\x9B\x27\x83\xA2\xEC\x07\xA2\x8F\
		\xB5\xC5\x5D\xF0\x6F\x4C\x52\xC9\xDE\x2B\xCB\xF6\x95\x58\x17\x18\
		\x39\x95\x49\x7C\xEA\x95\x6A\xE5\x15\xD2\x26\x18\x98\xFA\x05\x10\
		\x15\x72\x8E\x5A\x8A\xAA\xC4\x2D\xAD\x33\x17\x0D\x04\x50\x7A\x33\
		\xA8\x55\x21\xAB\xDF\x1C\xBA\x64\xEC\xFB\x85\x04\x58\xDB\xEF\x0A\
		\x8A\xEA\x71\x57\x5D\x06\x0C\x7D\xB3\x97\x0F\x85\xA6\xE1\xE4\xC7\
		\xAB\xF5\xAE\x8C\xDB\x09\x33\xD7\x1E\x8C\x94\xE0\x4A\x25\x61\x9D\
		\xCE\xE3\xD2\x26\x1A\xD2\xEE\x6B\xF1\x2F\xFA\x06\xD9\x8A\x08\x64\
		\xD8\x76\x02\x73\x3E\xC8\x6A\x64\x52\x1F\x2B\x18\x17\x7B\x20\x0C\
		\xBB\xE1\x17\x57\x7A\x61\x5D\x6C\x77\x09\x88\xC0\xBA\xD9\x46\xE2\
		\x08\xE2\x4F\xA0\x74\xE5\xAB\x31\x43\xDB\x5B\xFC\xE0\xFD\x10\x8E\
		\x4B\x82\xD1\x20\xA9\x21\x08\x01\x1A\x72\x3C\x12\xA7\x87\xE6\xD7\
		\x88\x71\x9A\x10\xBD\xBA\x5B\x26\x99\xC3\x27\x18\x6A\xF4\xE2\x3C\
		\x1A\x94\x68\x34\xB6\x15\x0B\xDA\x25\x83\xE9\xCA\x2A\xD4\x4C\xE8\
		\xDB\xBB\xC2\xDB\x04\xDE\x8E\xF9\x2E\x8E\xFC\x14\x1F\xBE\xCA\xA6\
		\x28\x7C\x59\x47\x4E\x6B\xC0\x5D\x99\xB2\x96\x4F\xA0\x90\xC3\xA2\
		\x23\x3B\xA1\x86\x51\x5B\xE7\xED\x1F\x61\x29\x70\xCE\xE2\xD7\xAF\
		\xB8\x1B\xDD\x76\x21\x70\x48\x1C\xD0\x06\x91\x27\xD5\xB0\x5A\xA9\
		\x93\xB4\xEA\x98\x8D\x8F\xDD\xC1\x86\xFF\xB7\xDC\x90\xA6\xC0\x8F\
		\x4D\xF4\x35\xC9\x34\x02\x84\x92\x36\xC3\xFA\xB4\xD2\x7C\x70\x26\
		\xC1\xD4\xDC\xB2\x60\x26\x46\xDE\xC9\x75\x1E\x76\x3D\xBA\x37\xBD\
		\xF8\xFF\x94\x06\xAD\x9E\x53\x0E\xE5\xDB\x38\x2F\x41\x30\x01\xAE\
		\xB0\x6A\x53\xED\x90\x27\xD8\x31\x17\x97\x27\xB0\x86\x5A\x89\x18\
		\xDA\x3E\xDB\xEB\xCF\x9B\x14\xED\x44\xCE\x6C\xBA\xCE\xD4\xBB\x1B\
		\xDB\x7F\x14\x47\xE6\xCC\x25\x4B\x33\x20\x51\x51\x2B\xD7\xAF\x42\
		\x6F\xB8\xF4\x01\x37\x8C\xD2\xBF\x59\x83\xCA\x01\xC6\x4B\x92\xEC\
		\xF0\x32\xEA\x15\xD1\x72\x1D\x03\xF4\x82\xD7\xCE\x6E\x74\xFE\xF6\
		\xD5\x5E\x70\x2F\x46\x98\x0C\x82\xB5\xA8\x40\x31\x90\x0B\x1C\x9E\
		\x59\xE7\xC9\x7F\xBE\xC7\xE8\xF3\x23\xA9\x7A\x7E\x36\xCC\x88\xBE\
		\x0F\x1D\x45\xB7\xFF\x58\x5A\xC5\x4B\xD4\x07\xB2\x2B\x41\x54\xAA\
		\xCC\x8F\x6D\x7E\xBF\x48\xE1\xD8\x14\xCC\x5E\xD2\x0F\x80\x37\xE0\
		\xA7\x97\x15\xEE\xF2\x9B\xE3\x28\x06\xA1\xD5\x8B\xB7\xC5\xDA\x76\
		\xF5\x50\xAA\x3D\x8A\x1F\xBF\xF0\xEB\x19\xCC\xB1\xA3\x13\xD5\x5C\
		\xDA\x56\xC9\xEC\x2E\xF2\x96\x32\x38\x7F\xE8\xD7\x6E\x3C\x04\x68\
		\x04\x3E\x8F\x66\x3F\x48\x60\xEE\x12\xBF\x2D\x5B\x0B\x74\x74\xD6\
		\xE6\x94\xF9\x1E\x6D\xBE\x11\x59\x74\xA3\x92\x6F\x12\xFE\xE5\xE4\
		\x38\x77\x7C\xB6\xA9\x32\xDF\x8C\xD8\xBE\xC4\xD0\x73\xB9\x31\xBA\
		\x3B\xC8\x32\xB6\x8D\x9D\xD3\x00\x74\x1F\xA7\xBF\x8A\xFC\x47\xED\
		\x25\x76\xF6\x93\x6B\xA4\x24\x66\x3A\xAB\x63\x9C\x5A\xE4\xF5\x68\
		\x34\x23\xB4\x74\x2B\xF1\xC9\x78\x23\x8F\x16\xCB\xE3\x9D\x65\x2D\
		\xE3\xFD\xB8\xBE\xFC\x84\x8A\xD9\x22\x22\x2E\x04\xA4\x03\x7C\x07\
		\x13\xEB\x57\xA8\x1A\x23\xF0\xC7\x34\x73\xFC\x64\x6C\xEA\x30\x6B\
		\x4B\xCB\xC8\x86\x2F\x83\x85\xDD\xFA\x9D\x4B\x7F\xA2\xC0\x87\xE8\
		\x79\x68\x33\x03\xED\x5B\xDD\x3A\x06\x2B\x3C\xF5\xB3\xA2\x78\xA6\
		\x6D\x2A\x13\xF8\x3F\x44\xF8\x2D\xDF\x31\x0E\xE0\x74\xAB\x6A\x36\
		\x45\x97\xE8\x99\xA0\x25\x5D\xC1\x64\xF3\x1C\xC5\x08\x46\x85\x1D\
		\xF9\xAB\x48\x19\x5D\xED\x7E\xA1\xB1\xD5\x10\xBD\x7E\xE7\x4D\x73\
		\xFA\xF3\x6B\xC3\x1E\xCF\xA2\x68\x35\x90\x46\xF4\xEB\x87\x9F\x92\
		\x40\x09\x43\x8B\x48\x1C\x6C\xD7\x88\x9A\x00\x2E\xD5\xEE\x38\x2B\
		\xC9\x19\x0D\xA6\xFC\x02\x6E\x47\x95\x58\xE4\x47\x56\x77\xE9\xAA\
		\x9E\x30\x50\xE2\x76\x56\x94\xDF\xC8\x1F\x56\xE8\x80\xB9\x6E\x71\
		\x60\xC9\x80\xDD\x98\xED\xD3\xDF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF",
        g_data: b"\x00\x13"
    },
    SrdDhParams{
        p_data: b"",
        g_data: b""
    }
];

struct SrdDhParams
{
	p_data: &'static [u8],
	g_data: &'static [u8]
}

pub struct NowSrd<'a>
{
    is_server: bool,
	//NowSrdCallbacks cbs;

	keys: &'a [u8],
    key_size: u16,
	seq_num: u32,
	username: &'a str,
	password: &'a str,

	cert_data: &'a [u8],
	cert_size: usize,
	cbt_level: u32,

	buffers: [&'a[u8];6],

	client_nonce: [u8; 32],
	server_nonce: [u8; 32],
	delegation_key: [u8; 32],
	integrity_key: [u8; 32],
	iv: [u8; 32],

	generator: [u8; 2],

    status: u8
//	NowCCBigNumRef bnGenerator;

//	uint8_t* prime;
//	uint8_t* peerKey;
//	uint8_t* publicKey;
//	uint8_t* privateKey;
//	uint8_t* secretKey;
//
//	NowCCBigNumRef bnPrime;
//	NowCCBigNumRef bnPeerKey;
//	NowCCBigNumRef bnPublicKey;
//	NowCCBigNumRef bnPrivateKey;
//	NowCCBigNumRef bnSecretKey;
}

impl<'a> NowSrd<'a> {
    pub fn new(is_server: bool) -> NowSrd<'a> {
        NowSrd {
            is_server,
            keys: &[0; 32],
            key_size: 0,
            seq_num: 0,
            username: "hello",
            password: "world!",

            cert_data: &[0; 32],
            cert_size: 0,
            cbt_level: 0,

            buffers: [&[0; 32], &[0; 32], &[0; 32], &[0; 32], &[0; 32], &[0; 32]],

            client_nonce: [0; 32],
            server_nonce: [0; 32],
            delegation_key: [0; 32],
            integrity_key: [0; 32],
            iv: [0; 32],

            generator: [0; 2],
            status: 1
        }
    }

    pub fn now_srd_read_msg(&self, msg: &NowAuthSrdMessage, packet_type: u8) -> i32
    {
        let nstatus: u32 = 0;
        let header: &NowAuthSrdHeader = &msg.header;

        // Returns an error if the type is not expected
        if header.packet_type != packet_type as u16 {
            return -1;
        }

        match header.packet_type as u8 {
            NOW_AUTH_SRD_NEGOTIATE_ID => {},
            NOW_AUTH_SRD_CHALLENGE_ID => {},
            _ => {
                // Returns if the type is unknown
                return -1;
            }
        }
        10
    }
}

pub trait Message{
    fn read_from<R: Read>(reader: &mut R) -> Result<Self, std::io::Error>
    where
        Self: Sized;
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error>;
    fn get_size(&self) -> u32;
}

impl Message for Vec<u8> {
    fn read_from<R: Read>(reader: &mut R) -> Result<Vec<u8>, std::io::Error> {
        let length = reader.read_u16::<LittleEndian>()?;
        let mut buffer = vec![0; length as usize];
        reader.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        let length = self.len() as u16; // TODO: check?
        writer.write_u16::<LittleEndian>(length)?;
        writer.write_all(&self)?;
        Ok(())
    }
    fn get_size(&self) -> u32 {
        // u16 for the length + the vector itself
        (2 + self.len()) as u32
    }
}

impl Message for NowAuthSrdHeader{
    fn read_from<R: Read>(reader: &mut R) -> Result<NowAuthSrdHeader, std::io::Error> {
        Ok(NowAuthSrdHeader {
            packet_type: reader.read_u16:: < LittleEndian>()?,
            flags: reader.read_u16::<LittleEndian>()?
        })
    }

    fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        writer.write_u16::<LittleEndian>(self.packet_type)?;
        writer.write_u16::<LittleEndian>(self.flags)?;
        Ok(())
    }

    fn get_size(&self) -> u32 {
        4u32
    }
}


struct NowStream<'a>{
    pub buffer: &'a [u8],
    pub pointer: &'a u8,
    //capacity
}

pub struct NowAuthSrdHeader{
    pub packet_type: u16,
    pub flags: u16
}

pub struct NowAuthSrdNegotiate{
    pub key_size: u16,
    pub reserved: u16
}

pub struct NowAuthSrdChallenge<'a>{
    key_size: u16,
    generator: [u8; 2],
    prime: &'a [u8],
    public_key: &'a [u8],
    nonce: &'a [u8],
    cbt: [u8; 32],
    mac: [u8; 32]
}

pub enum NowAuthSrdPayload<'a>{
    NowAuthSrdNegotiate(NowAuthSrdNegotiate),
    NowAuthSrdChallenge(NowAuthSrdChallenge<'a>)
}

pub struct NowAuthSrdMessage<'a>{
    pub header: NowAuthSrdHeader,
    pub payload: NowAuthSrdPayload<'a>
}


