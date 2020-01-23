//! No.104 国道
//! https://yukicoder.me/problems/no/104

use std::io::stdin;

/// エントリポイント
fn main() {
    let input = read_lines();
    println!("{}", national_highway(input));
}

/// 標準入力から文字列を取得します。
fn read_lines() -> String {
    let mut str1 = String::new();
    stdin().read_line(&mut str1).unwrap();
    str1
}

/// 国道何号線を歩いているかを返します。
fn national_highway(right_left: String) -> i32 {
    // 標準入力からの文字列は最後に改行コードが入るため、トリムする。
    let right_left = right_left.trim();
    let mut road = 1;
    for s in right_left.chars() {
        if s == 'L' {
            road = road * 2;
        } else {
            road = road * 2 + 1;
        }
    }
    road
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn national_highway_test1() {
        assert_eq!(5, national_highway("LR".to_string()));
    }

    #[test]
    fn national_highway_test2() {
        assert_eq!(12, national_highway("RLL".to_string()));
    }

    #[test]
    fn national_highway_test3() {
        assert_eq!(12986, national_highway("RLLRLRLRRRLRL".to_string()));
    }

    #[test]
    fn national_highway_test4() {
        assert_eq!(1, national_highway("".to_string()));
    }
}
