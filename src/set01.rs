use std::iter::*;

static BASE64_MAP: &'static[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn base64_encode(x: &[u8]) -> String {
    let mut result = Vec::with_capacity((x.len() / 3) * 4);
    let chunks = x.chunks(3);

    for chunk in chunks {
        let word = match chunk.len() {
            1 => (chunk[0] as u32) << 16,
            2 => (chunk[0] as u32) << 16 | (chunk[1] as u32) << 8,
            3 => (chunk[0] as u32) << 16 | (chunk[1] as u32) << 8 | (chunk[2] as u32),
            _ => unimplemented!(),
        };
        result.push(BASE64_MAP[((word >> 18) & 0b111111) as usize]);
        result.push(BASE64_MAP[((word >> 12) & 0b111111) as usize]);
        result.push(BASE64_MAP[((word >>  6) & 0b111111) as usize]);
        result.push(BASE64_MAP[( word        & 0b111111) as usize]);
    }

    String::from_utf8(result).unwrap()
}

pub fn unhex(s: &str) -> Vec<u8> {
    assert!(s.len() % 2 == 0);
    s.as_bytes().chunks(2).map(|b| {
        convert(b[0]) << 4 | convert(b[1])
    }).collect()
}

type HexChar = u8;
type HexNum = u8;
pub fn convert(byte: HexChar) -> HexNum {
    match byte {
        b'a'...b'f' => 10 + byte - b'a',
        b'A'...b'F' => 10 + byte - b'A',
        b'0'...b'9' => byte - b'0',
        _ => panic!("invalid hex character")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(convert(b'0'), 0);
        assert_eq!(convert(b'1'), 1);
        assert_eq!(convert(b'2'), 2);
        assert_eq!(convert(b'3'), 3);
        assert_eq!(convert(b'4'), 4);
        assert_eq!(convert(b'5'), 5);
        assert_eq!(convert(b'6'), 6);
        assert_eq!(convert(b'7'), 7);
        assert_eq!(convert(b'8'), 8);
        assert_eq!(convert(b'9'), 9);

        assert_eq!(convert(b'a'), 10);
        assert_eq!(convert(b'b'), 11);
        assert_eq!(convert(b'c'), 12);
        assert_eq!(convert(b'd'), 13);
        assert_eq!(convert(b'e'), 14);
        assert_eq!(convert(b'f'), 15);

        assert_eq!(convert(b'A'), 10);
        assert_eq!(convert(b'B'), 11);
        assert_eq!(convert(b'C'), 12);
        assert_eq!(convert(b'D'), 13);
        assert_eq!(convert(b'E'), 14);
        assert_eq!(convert(b'F'), 15);
    }

    #[test]
    fn test_unhex() {
        assert_eq!(unhex(""), vec![]);
        assert_eq!(unhex("00"), vec![0]);
        assert_eq!(unhex("12"), vec![18]);
        assert_eq!(unhex("1f82Fa"), vec![31, 130, 250]);
    }

    #[test]
    fn test_c01() {
        let x = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let y = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(base64_encode(&unhex(x)), y);
    }
}
