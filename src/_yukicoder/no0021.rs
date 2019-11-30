//! No.21 平均の差
//! https://yukicoder.me/problems/no/21

use std::io::stdin;

/// 入力データ
#[derive(Debug)]
struct InputData {
    /// 数字の個数
    num_count: i32,
    /// グループ
    group: i32,
    /// 数字
    nums: Vec<i32>,
}

/// エントリポイント
fn main() {
    let input = input_data();
    println!("{}", average(input));
}

/// 復号化します。
fn average(input: InputData) -> f32 {
    let mut sorted = input.nums;
    // 降順にソート
    sorted.sort_by(|a, b| b.cmp(a));

    // 最大の平均
    let max_val = sorted[0] as f32;
    // 最小の平均
    let min_val = sorted[sorted.len() - 1] as f32;

    // 計算結果の小数点切り上げ
    (max_val - min_val).ceil()
}

/// 標準入力から文字列を取得します。
fn input_data() -> InputData {
    // 1行目
    let mut s1 = String::new();
    stdin().read_line(&mut s1).unwrap();
    let num_count: i32 = s1.trim().parse().unwrap();

    // 2行目
    let mut s2 = String::new();
    stdin().read_line(&mut s2).unwrap();
    let group: i32 = s2.trim().parse().unwrap();

    // 3行目以降
    let mut nums = Vec::new();
    // 1行目で取得したデータ件数分ループ
    for _i in 0..num_count {
        let mut s3 = String::new();
        stdin().read_line(&mut s3).unwrap();
        nums.push(s3.trim().parse().unwrap());
    }

    InputData {
        num_count,
        group,
        nums,
    }
}

#[cfg(test)]
mod tests {
    use crate::average;
    use crate::InputData;

    #[test]
    fn test_average1() {
        let input = InputData {
            num_count: 5,
            group: 3,
            nums: vec![555, 20, 432, 301, 21],
        };

        assert_eq!(average(input), 535f32);
    }

    #[test]
    fn test_average2() {
        let input = InputData {
            num_count: 8,
            group: 4,
            nums: vec![329, 980, 656, 738, 739, 542, 873, 501],
        };

        assert_eq!(average(input), 651f32);
    }
}
