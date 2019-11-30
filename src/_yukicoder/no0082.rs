//! No.82 市松模様
//! https://yukicoder.me/problems/no/82

use std::io::stdin;

/// エントリポイント
fn main() {
    let input = read_lines();
    println!("{}", checkered_pattern(input));
}

/// 標準入力から文字列を取得します。
fn read_lines() -> String {
    // 1行目
    let mut str1 = String::new();
    stdin().read_line(&mut str1).unwrap();
    str1
}

/// 市松模様の文字列を返します。
fn checkered_pattern(input: String) -> String {
    let sp = input.trim().split_whitespace().collect::<Vec<&str>>();
    let width = sp[0].parse::<i32>().unwrap();
    let height = sp[1].parse::<i32>().unwrap();
    let mut color = sp[2];

    let mut pattern = String::new();
    for _i in 0..height {
        for _i in 0..width {
            if color == "B" {
                pattern += "B";
                color = "W";
            } else {
                pattern += "W";
                color = "B";
            }
        }
        pattern += "\n";

        if width % 2 == 0 {
            if color == "W" {
                color = "B"
            } else {
                color = "W"
            }
        }
    }
    pattern.trim().to_string()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn checkered_pattern_test1() {
        assert_eq!(
            "BWBW
WBWB
BWBW",
            checkered_pattern("4 3 B".to_string())
        );
    }

    #[test]
    fn checkered_pattern_test2() {
        assert_eq!(
            "WBW
BWB
WBW
BWB",
            checkered_pattern("3 4 W".to_string())
        );
    }

    #[test]
    fn checkered_pattern_test3() {
        assert_eq!("W", checkered_pattern("1 1 W".to_string()));
    }
}
