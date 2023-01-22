
#[cfg(test)]
mod tests {
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
            let data = encode_header(test.timestamp, test.key_size, test.value_size);
            let (timestamp, key_size, value_size) = decode_header(data);
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
            size: i32,
        }

        let tests = [
            Test{timestamp: 10, key: "hello".to_string(), value: "world".to_string(), size: header_size + 10},
            Test{timestamp: 0, key: "".to_string(), value: "".to_string(), size: header_size},
            Test{timestamp: 100, key: "🔑".to_string(), value: "".to_string(), size: header_size + 4},
        ];

        for test in &tests {
            let (size, data) = encode_kv(test.timestamp, test.key, test.value);
            let (timestamp, key, value) = decode_kv(data);
            assert_eq!(timestamp, test.timestamp);
            assert_eq!(key, test.key);
            assert_eq!(value, test.value);
            assert_eq!(size, test.size);
        }
    }
}
