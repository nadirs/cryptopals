use std::iter::*;
use std::cmp::min;

static BASE64_MAP: &'static[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn base64_encode(x: &[u8]) -> String {
    let mut result = Vec::with_capacity((x.len() / 3) * 4);
    let chunks = x.chunks(3);

    let mut padding = 0;
    for chunk in chunks {
        let word = match chunk.len() {
            1 => {
                padding = 2;
                (chunk[0] as u32) << 16
            },
            2 => {
                padding = 1;
                (chunk[0] as u32) << 16 | (chunk[1] as u32) << 8
            },
            3 => (chunk[0] as u32) << 16 | (chunk[1] as u32) << 8 | (chunk[2] as u32),
            n => panic!("Found chunk with {} elements", n),
        };
        result.push(BASE64_MAP[((word >> 18) & 0b111111) as usize]);
        result.push(BASE64_MAP[((word >> 12) & 0b111111) as usize]);

        // Max padding is 2
        // if padding is less than 2, we can add 3rd byte to our result
        if padding < 2 {
            result.push(BASE64_MAP[((word >> 6) & 0b111111) as usize]);
        }
        // if padding is less than 1 (0), we can add 4th byte to our result
        if padding < 1 {
            result.push(BASE64_MAP[(word & 0b111111) as usize]);
        }
    }

    let mut encoded_str = String::from_utf8(result).unwrap();
    encoded_str.push_str(repeat("=").take(padding).collect::<String>().as_str());

    encoded_str
}

pub fn hex(bytes: &[u8]) -> String {
    let byte_str = bytes.iter()
        .flat_map(|b| {
            vec![
                hex_digit_to_char(*b >> 4),
                hex_digit_to_char(*b & 0b1111),
            ]
        })
        .collect();
    String::from_utf8(byte_str).unwrap()
}

pub fn unhex(s: &str) -> Vec<u8> {
    assert!(s.len() % 2 == 0);
    s.as_bytes().chunks(2).map(|b| {
        hex_char_to_digit(b[0]) << 4 | hex_char_to_digit(b[1])
    }).collect()
}

type HexChar = u8;
type HexNum = u8;
pub fn hex_char_to_digit(byte: HexChar) -> HexNum {
    match byte {
        b'a'...b'f' => 10 + byte - b'a',
        b'A'...b'F' => 10 + byte - b'A',
        b'0'...b'9' => byte - b'0',
        _ => panic!("invalid hex character")
    }
}

pub fn hex_digit_to_char(byte: HexNum) -> HexChar {
    match byte {
        n if n <= 9 => b'0' + n,
        n if n <= 15 => b'a' + n - 10,
        _ => panic!("invalid hex digit")
    }
}

pub fn fixed_xor(xs: &[u8], ys: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(min(xs.len(), ys.len()));
    for (x, y) in xs.iter().zip(ys) {
        result.push(x ^ y);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hex_char_to_digit() {
        assert_eq!(hex_char_to_digit(b'0'), 0);
        assert_eq!(hex_char_to_digit(b'1'), 1);
        assert_eq!(hex_char_to_digit(b'2'), 2);
        assert_eq!(hex_char_to_digit(b'3'), 3);
        assert_eq!(hex_char_to_digit(b'4'), 4);
        assert_eq!(hex_char_to_digit(b'5'), 5);
        assert_eq!(hex_char_to_digit(b'6'), 6);
        assert_eq!(hex_char_to_digit(b'7'), 7);
        assert_eq!(hex_char_to_digit(b'8'), 8);
        assert_eq!(hex_char_to_digit(b'9'), 9);

        assert_eq!(hex_char_to_digit(b'a'), 10);
        assert_eq!(hex_char_to_digit(b'b'), 11);
        assert_eq!(hex_char_to_digit(b'c'), 12);
        assert_eq!(hex_char_to_digit(b'd'), 13);
        assert_eq!(hex_char_to_digit(b'e'), 14);
        assert_eq!(hex_char_to_digit(b'f'), 15);

        assert_eq!(hex_char_to_digit(b'A'), 10);
        assert_eq!(hex_char_to_digit(b'B'), 11);
        assert_eq!(hex_char_to_digit(b'C'), 12);
        assert_eq!(hex_char_to_digit(b'D'), 13);
        assert_eq!(hex_char_to_digit(b'E'), 14);
        assert_eq!(hex_char_to_digit(b'F'), 15);
    }

    #[test]
    fn test_hex_digit_to_char() {
        assert_eq!(hex_digit_to_char(0), b'0');
        assert_eq!(hex_digit_to_char(1), b'1');
        assert_eq!(hex_digit_to_char(2), b'2');
        assert_eq!(hex_digit_to_char(3), b'3');
        assert_eq!(hex_digit_to_char(4), b'4');
        assert_eq!(hex_digit_to_char(5), b'5');
        assert_eq!(hex_digit_to_char(6), b'6');
        assert_eq!(hex_digit_to_char(7), b'7');
        assert_eq!(hex_digit_to_char(8), b'8');
        assert_eq!(hex_digit_to_char(9), b'9');
        assert_eq!(hex_digit_to_char(10), b'a');
        assert_eq!(hex_digit_to_char(11), b'b');
        assert_eq!(hex_digit_to_char(12), b'c');
        assert_eq!(hex_digit_to_char(13), b'd');
        assert_eq!(hex_digit_to_char(14), b'e');
        assert_eq!(hex_digit_to_char(15), b'f');
    }

    #[test]
    fn test_hex() {
        assert_eq!(hex(&vec![]), "");
        assert_eq!(hex(&vec![0]), "00");
        assert_eq!(hex(&vec![18]), "12");
        assert_eq!(hex(&vec![31, 130, 250]), "1f82fa");
    }

    #[test]
    fn test_unhex() {
        assert_eq!(unhex(""), vec![]);
        assert_eq!(unhex("00"), vec![0]);
        assert_eq!(unhex("12"), vec![18]);
        assert_eq!(unhex("1f82Fa"), vec![31, 130, 250]);
    }

    #[test]
    fn test_base64_encode() {
        assert_eq!(base64_encode("\0".as_bytes()), "AA==");
        assert_eq!(base64_encode("abcdef".as_bytes()), "YWJjZGVm");
        assert_eq!(base64_encode("abcdefgh".as_bytes()), "YWJjZGVmZ2g=");
    }

    #[test]
    fn test_hex_to_base64() {
        let x = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let y = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(base64_encode(&unhex(x)), y);
    }

    #[test]
    fn test_fixed_xor() {
        let a = "1c0111001f010100061a024b53535009181c";
        let b = "686974207468652062756c6c277320657965";
        let expected = "746865206b696420646f6e277420706c6179";
        assert_eq!(hex(&fixed_xor(&unhex(a), &unhex(b))), expected);
    }
}
