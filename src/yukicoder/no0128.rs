//! No.128 お年玉(1)
//! https://yukicoder.me/problems/no/128

use std::io::stdin;

/// エントリポイント
fn main() {
    let input = read_lines();
    println!("{}", lottery(input.0, input.1));
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

/// お年玉を均等に渡せる金額を返します。
fn lottery(budget: String, people: String) -> i64 {
    let budget = budget.parse::<i64>().unwrap();
    let people = people.parse::<i64>().unwrap();
    budget / 1000 / people * 1000
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn lottery_test1() {
        assert_eq!(2000, lottery("10000".to_string(), "5".to_string()));
    }

    #[test]
    fn lottery_test2() {
        assert_eq!(2000, lottery("24000".to_string(), "9".to_string()));
    }

    #[test]
    fn lottery_test3() {
        assert_eq!(0, lottery("1000".to_string(), "6".to_string()));
    }
}
