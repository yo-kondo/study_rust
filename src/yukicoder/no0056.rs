//! No.56 消費税
//! https://yukicoder.me/problems/no/56

use std::io::stdin;

/// エントリポイント
fn main() {
    let in1 = read_line();
    println!("{}", tax(in1));
}

/// 消費税を加算した金額を返します。
fn tax(price_and_rate: String) -> i32 {
    let sp: Vec<&str> = price_and_rate.split_whitespace().collect();
    let price: f64 = sp[0].trim().parse().unwrap();
    let tax_rate: f64 = sp[1].trim().parse().unwrap();

    let tax_rate = (price * tax_rate / 100f64).floor() as i32;
    let price = price as i32;
    price + tax_rate
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
    fn tax_test1() {
        assert_eq!(108, tax("100 8".to_string()));
    }

    #[test]
    fn tax_test2() {
        assert_eq!(10, tax("10 8".to_string()));
    }

    #[test]
    fn tax_test3() {
        assert_eq!(135, tax("123 10".to_string()));
    }

    #[test]
    fn tax_test4() {
        assert_eq!(29, tax("25 16".to_string()));
    }

    #[test]
    fn tax_test5() {
        assert_eq!(9556788, tax("6048600 58".to_string()));
    }
}
