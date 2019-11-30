//! No.18 うーさー暗号
//! https://yukicoder.me/problems/no/18

use std::char;
use std::io;

/// エントリポイント
fn main() {
    let input = input_data();
    println!("{}", decoding(input));
}

/// 復号化します。
fn decoding(input: String) -> String {
    let mut dec = String::new();

    for (i, v) in input.char_indices() {
        // char → u32(ascii)
        let ascii = v as i32;

        // 復号文字
        // インデックスは0から始まるため、+1
        let mut sub = ascii - (i as i32 + 1);

        // A-Z範囲外の調整
        loop {
            sub = match sub {
                s if s <= 64 => sub + 26,
                s if s >= 91 => sub - 26,
                _ => {
                    break;
                }
            };
        }

        dec = format!("{}{}", dec, (sub as u8) as char);
    }

    dec
}

/// 標準入力から文字列を取得します。
fn input_data() -> String {
    let mut encryption = String::new();
    io::stdin().read_line(&mut encryption).unwrap();
    // 改行を除去するためにtrim()する。
    encryption.trim().to_string()
}

#[cfg(test)]
mod tests {
    use crate::decoding;

    #[test]
    fn test_decoding1() {
        assert_eq!(decoding(String::from("BCD")), String::from("AAA"));
    }

    #[test]
    fn test_decoding2() {
        assert_eq!(
            decoding(String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ")),
            String::from("ZZZZZZZZZZZZZZZZZZZZZZZZZZ")
        );
    }

    #[test]
    fn test_decoding3() {
        assert_eq!(decoding(
            String::from("ZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ")),
                   String::from("YXWVUTSRQPONMLKJIHGFEDCBAZYXWVUTSRQPONMLKJIHGFEDCBAZYXWVUTSRQPONMLKJIHGFEDCBAZYXWVUTSRQPONMLKJIHGFEDCBAZ"));
    }
}
