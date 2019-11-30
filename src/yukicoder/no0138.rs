//! No.138 化石のバージョン
//! https://yukicoder.me/problems/no/138

use std::io::stdin;

/// エントリポイント
fn main() {
    let input = read_lines();
    println!("{}", version_check(input.0, input.1));
}

/// 標準入力から文字列を取得します。
fn read_lines() -> (String, String) {
    let mut str1 = String::new();
    stdin().read_line(&mut str1).unwrap();
    str1 = str1.trim().to_string();

    let mut str2 = String::new();
    stdin().read_line(&mut str2).unwrap();
    str2 = str2.trim().to_string();
    (str1, str2)
}

///対象のバージョンか化石のバージョンかどうかを返します。
/// ## arguments
/// * `fossil` - 化石のバージョン
/// * `target_version` - 対象のバージョン
fn version_check(fossil: String, target_version: String) -> String {
    let old_version = fossil
        .split(".")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    let new_version = target_version
        .split(".")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    for i in 0..(old_version.len()) {
        if old_version[i] < new_version[i] {
            return "NO".to_string();
        }
        if old_version[i] > new_version[i] {
            return "YES".to_string();
        }
    }
    "YES".to_string()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn version_check_test1() {
        assert_eq!(
            "YES",
            version_check("4.8.1".to_string(), "4.8.0".to_string())
        );
    }

    #[test]
    fn version_check_test2() {
        assert_eq!(
            "NO",
            version_check("0.0.0".to_string(), "1.1.1".to_string())
        );
    }

    #[test]
    fn version_check_test3() {
        assert_eq!(
            "NO",
            version_check("1.2.3".to_string(), "3.2.1".to_string())
        );
    }

    #[test]
    fn version_check_test4() {
        assert_eq!(
            "YES",
            version_check("22.84.57".to_string(), "3.95.45".to_string())
        );
    }

    #[test]
    fn version_check_test5() {
        assert_eq!(
            "NO",
            version_check("50.50.50".to_string(), "50.50.51".to_string())
        );
    }
}
