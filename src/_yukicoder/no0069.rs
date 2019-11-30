//! No.69 文字を自由に並び替え
//! https://yukicoder.me/problems/no/69

use std::io::stdin;

/// エントリポイント
fn main() {
    let in1 = read_line();
    let in2 = read_line();
    println!("{}", same(in1, in2));
}

/// 並び替えた文字列が同じかどうか判定します。
fn same(str1: String, str2: String) -> String {
    let str1 = sort_string(str1);
    let str2 = sort_string(str2);
    if str1 == str2 {
        String::from("YES")
    } else {
        String::from("NO")
    }
}

/// 文字列をソートします。
fn sort_string(str: String) -> String {
    // Stringをchar配列に変換
    let mut chars: Vec<char> = str.chars().collect();
    chars.sort();
    // ソートしたcharをStringに戻す
    let result: String = chars.into_iter().collect();
    result
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
    fn same_test1() {
        assert_eq!("YES", same("dog".to_string(), "god".to_string()));
    }

    #[test]
    fn same_test2() {
        assert_eq!("NO", same("cat".to_string(), "tea".to_string()));
    }

    #[test]
    fn same_test3() {
        assert_eq!("YES", same("silence".to_string(), "license".to_string()));
    }

    #[test]
    fn same_test4() {
        assert_eq!(
            "YES",
            same("_yukicoder".to_string(), "_yukicoder".to_string())
        );
    }

    #[test]
    fn same_test5() {
        assert_eq!(
            "NO",
            same("hurjztkyua".to_string(), "urjukzthua".to_string())
        );
    }
}
