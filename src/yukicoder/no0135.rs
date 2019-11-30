//! No.135 とりあえず1次元の問題
//! https://yukicoder.me/problems/no/135

use std::io::stdin;

/// エントリポイント
fn main() {
    let input = read_lines();
    println!("{}", minimum_distance(input));
}

/// 標準入力から文字列を取得します。
fn read_lines() -> String {
    // 1行目は不要
    let mut str1 = String::new();
    stdin().read_line(&mut str1).unwrap();

    let mut str2 = String::new();
    stdin().read_line(&mut str2).unwrap();
    str2 = str2.trim().to_string();
    str2
}

/// 座標間の最小距離を返します。
fn minimum_distance(distance: String) -> i32 {
    let mut nums = distance
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|&x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    // すべて0の場合
    if nums.iter().all(|&x| x == 0) {
        return 0;
    }

    nums.sort();

    // 同じ値があると0になってしまうため、重複を排除する。
    let mut distinct = Vec::new();
    let mut before = std::i32::MAX;
    for i in nums {
        if i == before {
            continue;
        }
        distinct.push(i);
        before = i;
    }

    let mut min = std::i32::MAX;
    for i in 0..(distinct.len() - 1) {
        let m = distinct[i + 1] - distinct[i];
        if min > m {
            min = m;
        }
    }
    min
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn minimum_distance_test1() {
        assert_eq!(49, minimum_distance("0 51 100".to_string()));
    }

    #[test]
    fn minimum_distance_test2() {
        assert_eq!(1, minimum_distance("0 1 1 0".to_string()));
    }
}
