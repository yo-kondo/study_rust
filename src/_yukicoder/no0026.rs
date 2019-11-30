//! No.26 シャッフルゲーム
//! https://yukicoder.me/problems/no/26

use std::io;

/// 入力データ
#[derive(Debug)]
struct InputData {
    /// 最初に○印が付いているカップの位置
    first_position: i32,
    /// シャッフル回数
    shuffle_count: i32,
    /// シャッフルした位置
    shuffle_indices: Vec<[i32; 2]>,
}

/// エントリポイント
fn main() {
    let input = input_data();
    println!("{}", position_number(input));
}

/// ○印がついているカップの位置を返します。
fn position_number(input: InputData) -> i32 {
    // カップの位置情報（true:○、false:×）
    let mut correct_answer: [bool; 3] = [false, false, false];

    // 初期値
    correct_answer[(input.first_position - 1) as usize] = true;

    for index in input.shuffle_indices {
        let i = (index[0] - 1) as usize;
        let j = (index[1] - 1) as usize;

        let temp = correct_answer[i];
        correct_answer[i] = correct_answer[j];
        correct_answer[j] = temp;

        // 配列のstd::mem::swapは&mutでムーブされてしまうため、エラーが発生する。
        // error[E0499]: cannot borrow `correct_answer[..]` as mutable more than once at a time
        // std::mem::swap(&mut correct_answer[i], &mut correct_answer[j]);
    }

    // 結果を探す
    for (i, ans) in correct_answer.iter().enumerate() {
        if *ans {
            return i as i32 + 1i32;
        }
    }

    // ここには来ない
    0
}

/// 標準入力から文字列を取得します。
fn input_data() -> InputData {
    // 1行目
    let mut s1 = String::new();
    io::stdin().read_line(&mut s1).unwrap();
    let first_position: i32 = s1.trim().parse().unwrap();

    // 2行目
    let mut s2 = String::new();
    io::stdin().read_line(&mut s2).unwrap();
    let shuffle_count: i32 = s2.trim().parse().unwrap();

    // 3行目以降
    let mut shuffle_indices = Vec::new();
    for _i in 0..shuffle_count {
        let mut s3 = String::new();
        io::stdin().read_line(&mut s3).unwrap();

        let sp: Vec<&str> = s3.trim().split_whitespace().collect();
        let array: [i32; 2] = [sp[0].parse().unwrap(), sp[1].parse().unwrap()];
        shuffle_indices.push(array);
    }

    InputData {
        first_position,
        shuffle_count,
        shuffle_indices,
    }
}

#[cfg(test)]
mod tests {
    use crate::position_number;
    use crate::InputData;

    #[test]
    fn test_position_number1() {
        let input = InputData {
            first_position: 1,
            shuffle_count: 1,
            shuffle_indices: vec![[1, 3]],
        };

        assert_eq!(position_number(input), 3);
    }

    #[test]
    fn test_position_number2() {
        let input = InputData {
            first_position: 1,
            shuffle_count: 3,
            shuffle_indices: vec![[2, 3], [3, 2], [2, 3]],
        };

        assert_eq!(position_number(input), 1);
    }
}
