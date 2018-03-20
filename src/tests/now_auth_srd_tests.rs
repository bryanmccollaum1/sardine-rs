use Result;
use now_auth_srd::NowSrd;
use now_auth_srd_errors::NowAuthSrdError;

static TEST_CERT_DATA: &'static [u8] =
    b"\x30\x82\x02\xfa\x30\x82\x01\xe2\xa0\x03\x02\x01\x02\x02\x10\x16
	\xed\x2a\xa0\x49\x5f\x25\x9d\x4f\x5d\x99\xed\xad\xa5\x70\xd1\x30
	\x0d\x06\x09\x2a\x86\x48\x86\xf7\x0d\x01\x01\x05\x05\x00\x30\x26
	\x31\x24\x30\x22\x06\x03\x55\x04\x03\x13\x1b\x57\x49\x4e\x32\x4b
	\x38\x52\x32\x2e\x61\x77\x61\x6b\x65\x63\x6f\x64\x69\x6e\x67\x2e
	\x61\x74\x68\x2e\x63\x78\x30\x1e\x17\x0d\x31\x31\x30\x32\x31\x32
	\x32\x31\x33\x37\x33\x35\x5a\x17\x0d\x32\x31\x30\x32\x31\x32\x30
	\x30\x30\x30\x30\x30\x5a\x30\x26\x31\x24\x30\x22\x06\x03\x55\x04
	\x03\x13\x1b\x57\x49\x4e\x32\x4b\x38\x52\x32\x2e\x61\x77\x61\x6b
	\x65\x63\x6f\x64\x69\x6e\x67\x2e\x61\x74\x68\x2e\x63\x78\x30\x82
	\x01\x22\x30\x0d\x06\x09\x2a\x86\x48\x86\xf7\x0d\x01\x01\x01\x05
	\x00\x03\x82\x01\x0f\x00\x30\x82\x01\x0a\x02\x82\x01\x01\x00\xfb
	\x55\xc6\x84\xd0\xee\x3a\x06\xdd\xbb\xbe\x86\xb3\x3f\xfa\x84\x94
	\xd0\x47\xfe\x4e\x79\x4e\x57\x6e\x5f\x8a\x29\x81\x9b\xf7\xb2\xf4
	\x7b\xcd\x14\xd3\x38\x18\xbd\x54\xac\x62\xc9\xad\x3e\xc9\xcd\xe2
	\x3d\x28\x38\xdd\x8d\x40\x62\x03\x25\x97\xe0\x4f\x15\xd3\x35\x19
	\x01\x49\x8f\xbd\x67\xae\x1e\x23\xd0\x19\x73\x4a\xc5\x4c\x29\xa8
	\x5f\x88\x9c\xe1\x1b\x33\xf0\xdf\xb1\x97\xcf\x49\x75\x32\xb2\x1c
	\xee\x9d\x85\xd8\x1e\xe8\xd1\xee\xf5\xd4\x44\xc7\x84\x42\x50\x6b
	\x73\x10\x70\x4c\x33\xdb\x52\x87\xfc\xc9\xf0\xd7\xf0\xfc\xb8\xbe
	\xdf\x9e\x55\x42\x22\x80\x92\xe8\x76\x66\x5d\x33\x1f\x35\x3c\x6e
	\x4e\x3a\xbb\x5d\xd8\xd6\xaa\x59\xd2\x3d\x7b\xdb\x06\xba\x10\xff
	\x4d\x6e\x57\x5d\x9a\xdb\xb3\x77\x6f\xf5\xe0\xe1\xdf\x4e\xc2\x4f
	\xde\x66\x5f\x88\xbe\xe4\x0e\x73\x1a\x39\xa0\x20\x36\xd6\x07\xd1
	\x27\xbd\xae\x11\x81\xaa\x5b\x7b\x99\x71\x5b\xfd\xbb\xf5\x5f\x41
	\x49\xda\x45\x1d\x57\xe3\xa2\x05\x31\x1c\x75\xb6\x73\xdd\x04\xe7
	\x7b\xed\xc6\x3a\x00\xbd\xb7\x83\x73\x44\xf2\xfd\x3b\x2a\xe5\x8d
	\xb5\x19\x63\xe0\x2a\xa7\x15\xaa\x42\xe3\xa3\xd5\xde\xf1\x97\x02
	\x03\x01\x00\x01\xa3\x24\x30\x22\x30\x0b\x06\x03\x55\x1d\x0f\x04
	\x04\x03\x02\x04\x30\x30\x13\x06\x03\x55\x1d\x25\x04\x0c\x30\x0a
	\x06\x08\x2b\x06\x01\x05\x05\x07\x03\x01\x30\x0d\x06\x09\x2a\x86
	\x48\x86\xf7\x0d\x01\x01\x05\x05\x00\x03\x82\x01\x01\x00\x7c\x43
	\x0c\x5a\xbb\x0c\x72\x18\xc2\xbb\x38\x20\xa6\x8a\xf5\x99\x5b\x58
	\x8a\x44\x51\x60\xed\x40\x82\x23\x83\x82\xed\x51\x7c\x69\xb9\xca
	\xdc\x35\x8d\xc9\xd0\x79\x21\xcd\xbd\xf3\x4b\x3f\x80\x57\xc8\x35
	\x23\xff\x95\x2a\xd2\x01\xa8\xb6\x30\x50\x65\xc4\x6a\x55\x26\xd6
	\xd7\xa4\x18\x34\x76\x8d\x9d\x5c\x73\x99\xf4\xbc\x3f\xdf\xe3\x1d
	\xbd\x1b\x7f\x1c\x7d\xb0\x2f\x50\xb4\x48\x38\x4b\x0a\xe8\x35\x83
	\xaa\xe5\x30\x51\xcb\xc2\x61\x65\x87\xae\x85\x77\x47\xf5\xd1\xcc
	\xec\x89\x83\x85\x0e\x8b\xeb\x25\x2f\x25\xfa\x35\x35\x50\x9f\xbd
	\x36\x72\xa3\xf6\xee\xe6\xb3\xe8\x99\x36\xcc\x88\x44\x85\x78\x2f
	\xdc\xfd\xb9\xdd\x6b\x8e\x40\xe7\x58\x64\x23\xde\x42\x9e\xba\xf5
	\x00\x8d\xa5\x37\x55\xe6\x59\x3b\x0f\xad\x9b\x02\x13\xfc\x8e\x49
	\xae\xe9\x22\xb0\x9c\x76\x8e\x35\x4d\xe1\x2a\x14\xa0\x7e\x42\x9e
	\x01\x9a\xb8\xac\xd0\x92\xa7\xc7\x85\x8f\x43\xd8\x8c\xd1\x30\x8a
	\xa6\xf4\x1a\x06\x34\x84\x23\xe1\x92\x89\xa5\x36\xe1\x69\x75\x57
	\x43\x3f\xed\xc0\x75\x76\x19\x22\x59\xd1\xcd\x28\x75\xda\xf5\x02
	\x38\xd2\x5a\xc3\x23\x74\x2c\x40\xc7\xf1\xf1\xad\xdf\x6c";

const TEST_USERNAME: &'static str = "john.doe";
const TEST_PASSWORD: &'static str = "Dummy123";

#[test]
fn good_login() {
    let mut client: NowSrd = NowSrd::new(false).unwrap();
    let mut server: NowSrd = NowSrd::new(true).unwrap();

    let mut in_data: Vec<u8> = Vec::new();
    let mut out_data: Vec<u8> = Vec::new();

    client.set_cert_data(TEST_CERT_DATA.to_vec()).unwrap();
    server.set_cert_data(TEST_CERT_DATA.to_vec()).unwrap();

    client
        .set_credentials(TEST_USERNAME.to_string(), TEST_PASSWORD.to_string())
        .unwrap();

    let mut client_status: bool = false;
    let mut server_status: bool = false;

    while !client_status && !server_status {
        client_status = client.authenticate(&mut in_data, &mut out_data).unwrap();
        in_data = out_data;
        out_data = Vec::new();

        server_status = server.authenticate(&mut in_data, &mut out_data).unwrap();
        in_data = out_data;
        out_data = Vec::new()
    }

    assert!(client_status);
    assert!(server_status);

    assert_eq!(server.get_username(), TEST_USERNAME);
    assert_eq!(server.get_password(), TEST_PASSWORD);
}
