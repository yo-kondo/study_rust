//! No.32 貯金箱の憂鬱
//! https://yukicoder.me/problems/no/32

use std::io::stdin;

/// 入力データ
#[derive(Debug)]
struct InputData {
    /// 100 円硬貨
    yen100: i32,
    /// 25 円硬貨
    yen25: i32,
    /// 1 円硬貨
    yen1: i32,
}

/// エントリポイント
fn main() {
    let input = input_data();
    println!("{}", exchange(input));
}

/// 両替を行い、硬貨の合計枚数を返します。
fn exchange(input: InputData) -> i32 {
    let mut yen100 = input.yen100;
    let mut yen25 = input.yen25;
    let mut yen1 = input.yen1;

    yen25 += yen1 / 25;
    yen1 %= 25;

    yen100 += yen25 / 4;
    yen25 %= 4;

    yen100 %= 10;

    yen100 + yen25 + yen1
}

/// 標準入力から文字列を取得します。
fn input_data() -> InputData {
    // 1行目
    let mut yen100 = String::new();
    stdin().read_line(&mut yen100).unwrap();
    let yen100: i32 = yen100.trim().parse().unwrap();

    // 2行目
    let mut yen25 = String::new();
    stdin().read_line(&mut yen25).unwrap();
    let yen25: i32 = yen25.trim().parse().unwrap();

    // 3行目
    let mut yen1 = String::new();
    stdin().read_line(&mut yen1).unwrap();
    let yen1: i32 = yen1.trim().parse().unwrap();

    InputData {
        yen100,
        yen25,
        yen1,
    }
}

#[cfg(test)]
mod tests {
    use crate::exchange;
    use crate::InputData;

    #[test]
    fn test_exchange1() {
        let input = InputData {
            yen100: 7,
            yen25: 20,
            yen1: 10,
        };
        assert_eq!(exchange(input), 12);
    }

    #[test]
    fn test_exchange2() {
        let input = InputData {
            yen100: 0,
            yen25: 0,
            yen1: 0,
        };
        assert_eq!(exchange(input), 0);
    }
}
