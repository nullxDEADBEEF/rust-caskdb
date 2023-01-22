// To encode our data we need to have a fixed-size header, this header
// is the addition of timestamp + key_size + value_size, each 4 bytes, meaning 
// our header is 12 bytes
const FIXED_HEADER_SIZE: u8 = 12;


struct Header {
    timestamp: u32,
    key_size: u32,
    value_size: u32,
}

impl Header {
    fn encode(timestamp: u32, key_size: u32, value_size: u32) -> [u8; FIXED_HEADER_SIZE as usize] {
        let mut header: [u8; FIXED_HEADER_SIZE as usize] = [0; FIXED_HEADER_SIZE as usize];
        header[0..4].copy_from_slice(&timestamp.to_ne_bytes());
        header[4..8].copy_from_slice(&key_size.to_ne_bytes());
        header[8..12].copy_from_slice(&value_size.to_ne_bytes());

        println!("HEADER: {:?}", header);
        header
    }

    fn decode(header: [u8; FIXED_HEADER_SIZE as usize]) -> (u32, u32, u32) {
        let timestamp = u32::from_ne_bytes(header[0..4].try_into().unwrap());
        let key_size = u32::from_ne_bytes(header[4..8].try_into().unwrap());
        let value_size = u32::from_ne_bytes(header[8..12].try_into().unwrap());

        (timestamp, key_size, value_size)
    }

}

struct KeyDir {
    timestamp: u32,
    key: String,
    value: String,
    size: u32,
}

impl KeyDir {
    fn new(timestamp: u32, key: &str, value: &str) -> Self {
        Self {
            timestamp,
            key: key.to_owned(),
            value: value.to_owned(),
            size: 0,
        }
    }

    // TODO: return correct data, should probably be moved somewhere else
    // should return (size, data)
    fn encode_kv(timestamp: u32, key: &str, value: &str) -> (u32, u32) {
        let key_dir = KeyDir::new(timestamp, key, value);

        (key_dir.size, key_dir.timestamp)
    }

    // TODO: return correct data, should probably be moved somewhere else
    // should return (timestamp, key, value)
    fn decode_kv() -> (u32, String, String) {
        (0, "".to_owned(), "".to_owned())
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
            let data = Header::encode(test.timestamp, test.key_size, test.value_size);
            let (timestamp, key_size, value_size) = Header::decode(data);
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
            let (size, data) = KeyDir::encode_kv(test.timestamp, &test.key, &test.value);
            let (timestamp, key, value) = KeyDir::decode_kv();
            assert_eq!(timestamp, test.timestamp);
            assert_eq!(key, test.key);
            assert_eq!(value, test.value);
            assert_eq!(size, test.size);
        }
    }
}
