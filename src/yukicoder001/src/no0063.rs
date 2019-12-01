//! No.63 ポッキーゲーム
//! https://yukicoder.me/problems/no/63

use std::io::stdin;

/// エントリポイント
fn main() {
    let in1 = read_line();
    println!("{}", pocky(in1));
}

/// ポッキーを何cmかじれるか判定します。
fn pocky(length_and_gnaw: String) -> i32 {
    let sp: Vec<&str> = length_and_gnaw.split_whitespace().collect();
    let length: i32 = sp[0].trim().parse().unwrap();
    let gnaw: i32 = sp[1].trim().parse().unwrap();

    let mut count = length / (gnaw * 2);
    let rem = length % (gnaw * 2);
    if rem == 0 {
        // ちょうど割り切れた場合はかじるのをやめるため、-1
        count -= 1;
    }
    count * gnaw
}

/// 標準入力から1行取得して返します。
fn read_line() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn pocky_test1() {
        assert_eq!(40, pocky("90 20".to_string()));
    }

    #[test]
    fn pocky_test2() {
        assert_eq!(0, pocky("100 50".to_string()));
    }

    #[test]
    fn pocky_test3() {
        assert_eq!(550, pocky("1111 11".to_string()));
    }
}
