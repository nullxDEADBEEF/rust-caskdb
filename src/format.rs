// To encode our data we need to have a fixed-size header, this header
// is the addition of timestamp + key_size + value_size, each 4 bytes, meaning 
// our header is 12 bytes
const FIXED_HEADER_SIZE: u32 = 12;


struct Codec;

impl Codec {
    fn encode_header(timestamp: u32, key_size: u32, value_size: u32) -> Vec<u8> {
        let mut header = vec![0; FIXED_HEADER_SIZE as usize];
        header[0..4].copy_from_slice(&timestamp.to_ne_bytes());
        header[4..8].copy_from_slice(&key_size.to_ne_bytes());
        header[8..12].copy_from_slice(&value_size.to_ne_bytes());

        println!("HEADER: {:?}", header);
        header
    }

    fn decode_header(header: &[u8]) -> (u32, u32, u32) {
        let timestamp = u32::from_ne_bytes(header[0..4].try_into().unwrap());
        let key_size = u32::from_ne_bytes(header[4..8].try_into().unwrap());
        let value_size = u32::from_ne_bytes(header[8..12].try_into().unwrap());

        (timestamp, key_size, value_size)
    }

    fn encode_kv(timestamp: u32, key: &str, value: &str) -> (u32, Vec<u8>) {
        let mut header = Codec::encode_header(timestamp, key.len() as u32, value.len() as u32);
        let data: Vec<_> = key.as_bytes().iter().chain(value.as_bytes().iter()).collect();
        let data_length = data.len();
        header.extend(data);

        (FIXED_HEADER_SIZE as u32 + data_length as u32, header)
    }

    fn decode_kv(data: &[u8]) -> (u32, String, String) {
        let (timestamp, key_size, value_size) = Codec::decode_header(&data[..FIXED_HEADER_SIZE as usize]);
        let key = std::str::from_utf8(&data[FIXED_HEADER_SIZE as usize..(FIXED_HEADER_SIZE+key_size) as usize]).unwrap();
        let value = std::str::from_utf8(&data[(FIXED_HEADER_SIZE + key_size) as usize..(FIXED_HEADER_SIZE + key_size + value_size) as usize]).unwrap();

        (timestamp, key.to_owned(), value.to_owned())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_header() {
        struct Test {
            timestamp: u32,
            key_size: u32,
            value_size: u32
        }

        let tests = [
            Test{timestamp: 10, key_size: 0, value_size: 10000},
            Test{timestamp: 10, key_size: 0, value_size: 10000},
            Test{timestamp: 10, key_size: 0, value_size: 10000},
        ];

        for test in &tests {
            let data = Codec::encode_header(test.timestamp, test.key_size, test.value_size);
            let (timestamp, key_size, value_size) = Codec::decode_header(&data);
            assert_eq!(timestamp, test.timestamp);
            assert_eq!(key_size, test.key_size);
            assert_eq!(value_size, test.value_size);
        }
    }

    #[test]
    fn test_encode_kv() {
        struct Test {
            timestamp: u32,
            key: String,
            value: String,
            size: u32,
        }

        let tests = [
            Test{timestamp: 10, key: "hello".to_string(), value: "world".to_string(), size: FIXED_HEADER_SIZE as u32 + 10},
            Test{timestamp: 0, key: "".to_string(), value: "".to_string(), size: FIXED_HEADER_SIZE as u32 },
            Test{timestamp: 100, key: "🔑".to_string(), value: "".to_string(), size: FIXED_HEADER_SIZE as u32 + 4},
        ];

        for test in &tests {
            let (size, data) = Codec::encode_kv(test.timestamp, &test.key, &test.value);
            let (timestamp, key, value) = Codec::decode_kv(&data);
            assert_eq!(timestamp, test.timestamp);
            assert_eq!(key, test.key);
            assert_eq!(value, test.value);
            assert_eq!(size, test.size);
        }
    }
}
