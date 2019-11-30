//! No.79 過小評価ダメ・ゼッタイ
//! https://yukicoder.me/problems/no/79

use std::io::stdin;

/// エントリポイント
fn main() {
    let input = read_lines();
    println!("{}", majority_vote(input));
}

/// 標準入力から文字列を取得します。
fn read_lines() -> String {
    // 1行目
    let mut str1 = String::new();
    stdin().read_line(&mut str1).unwrap();

    // 2行目
    let mut str2 = String::new();
    stdin().read_line(&mut str2).unwrap();
    str2
}

/// 多数決で一番多いレベルを取得します。
/// 複数ある場合は、レベルの高い方を取得します。
fn majority_vote(survey: String) -> i32 {
    let list = survey
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        // Vec<&str> を Vec<i32> に変換
        .map(|&x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    // voteの0番目は使用しない
    let mut vote = [0; 7];
    for v in list {
        let i = v as usize;
        vote[i] = vote[i] + 1;
    }

    // 配列の中で一番大きい数値のインデックスを返す。
    let mut max_index = 0;
    let mut max_num = 0;
    for i in 0..7 {
        if vote[i] >= max_num {
            max_index = i;
            max_num = vote[i];
        }
    }
    max_index as i32
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn majority_vote_test1() {
        assert_eq!(2, majority_vote("2 1 2 3 2".to_string()));
    }

    #[test]
    fn majority_vote_test2() {
        assert_eq!(3, majority_vote("1 2 3 3 1".to_string()));
    }

    #[test]
    fn majority_vote_test3() {
        assert_eq!(6, majority_vote("5 3 5 6 3 6 2 3 5 6".to_string()));
    }
}
