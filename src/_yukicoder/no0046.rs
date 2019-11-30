//! No.46 はじめのn歩
//! https://yukicoder.me/problems/no/46

use std::io::stdin;

/// エントリポイント
fn main() {
    let input = input_data();
    println!("{}", walking(input));
}

/// 何歩で到達できるか計算します。
fn walking(input: String) -> i32 {
    let sp: Vec<&str> = input.trim().split_whitespace().collect();

    // 一歩で歩ける距離
    let walk: f64 = sp[0].parse().unwrap();
    // 歩く距離
    let distance: f64 = sp[1].parse().unwrap();

    (distance / walk).ceil() as i32
}

/// 標準入力から文字列を取得します。
fn input_data() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    input
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn walking_test() {
        assert_eq!(3, walking("2 5".to_string()));
        assert_eq!(10, walking("10 100".to_string()));
        assert_eq!(9, walking("123456789 987654321".to_string()));
    }
}
