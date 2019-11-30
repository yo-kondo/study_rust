//! No.116 門松列(1)
//! https://yukicoder.me/problems/no/116

use std::io::stdin;

/// エントリポイント
fn main() {
    let input = read_lines();
    println!("{}", pine_decoration(input));
}

/// 標準入力から文字列を取得します。
fn read_lines() -> String {
    // 1行目は不要
    let mut str1 = String::new();
    stdin().read_line(&mut str1).unwrap();

    let mut str2 = String::new();
    stdin().read_line(&mut str2).unwrap();
    str2
}

/// 竹の高さが門松の列になっている個数を返します。
fn pine_decoration(bamboo_heights: String) -> i32 {
    let h = bamboo_heights
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        // Vec<&str> を Vec<i32> に変換
        .map(|&x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    // 門松の判定
    let mut count = 0;
    for i in 1..(h.len() - 1) {
        // いずれかが同じ高さ
        if h[i - 1] == h[i] || h[i] == h[i + 1] || h[i - 1] == h[i + 1] {
            continue;
        }

        // 中央が2番目の高さではない
        if (h[i - 1] > h[i] && h[i] > h[i + 1]) || (h[i - 1] < h[i] && h[i] < h[i + 1]) {
            continue;
        }
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn pine_decoration_test1() {
        assert_eq!(2, pine_decoration("1 3 4 1 2".to_string()));
    }

    #[test]
    fn pine_decoration_test2() {
        assert_eq!(2, pine_decoration("1 4 2 4 1".to_string()));
    }

    #[test]
    fn pine_decoration_test3() {
        assert_eq!(2, pine_decoration("1 4 1 5 2".to_string()));
    }
}
