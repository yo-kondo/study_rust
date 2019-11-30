//! No.5 数字のブロック
//! https://yukicoder.me/problems/no/5

use std::io;

/// 入力データ
#[derive(Debug)]
struct InputData {
    /// 大きな箱の幅
    box_width: i32,
    /// ブロックの数
    block_num: i32,
    /// 各ブロックの幅
    block_width: Vec<i32>,
}

/// エントリポイント
fn main() {
    let input = input_data();
    println!("{}", calc_in_boxes(input));
}

/// 箱に入るブロックの数を計算して返します。
fn calc_in_boxes(input: InputData) -> i32 {
    // block_widthを昇順にソート
    let mut blocks = input.block_width;
    blocks.sort();

    //
    let mut num = 0;
    let mut remaining_width = input.box_width;

    // 小さいブロックから数えて、箱の幅を超えたら終了
    for b in &blocks {
        if remaining_width >= *b {
            remaining_width -= *b;
            num += 1;
        } else {
            break;
        }
    }

    num
}

/// 標準入力から以下の形式で取得した情報を構造体に設定します。
/// ```
/// 16
/// 3
/// 10 5 7
/// ```
fn input_data() -> InputData {
    // 1行目
    let mut box_width = String::new();
    io::stdin().read_line(&mut box_width).unwrap();
    let box_width: i32 = box_width.trim().parse().unwrap();

    // 2行目
    let mut block_num = String::new();
    io::stdin().read_line(&mut block_num).unwrap();
    let block_num: i32 = block_num.trim().parse().unwrap();

    // 3行目
    let mut block_width = String::new();
    io::stdin().read_line(&mut block_width).unwrap();
    let block_width = block_width
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();

    InputData {
        box_width,
        block_num,
        block_width,
    }
}

#[cfg(test)]
mod tests {
    use crate::calc_in_boxes;
    use crate::InputData;

    #[test]
    fn test_calc_in_boxes1() {
        let input = InputData {
            box_width: 16,
            block_num: 3,
            block_width: vec![10, 5, 7],
        };

        assert_eq!(calc_in_boxes(input), 2);
    }

    #[test]
    fn test_calc_in_boxes2() {
        let input = InputData {
            box_width: 100,
            block_num: 10,
            block_width: vec![14, 85, 77, 26, 50, 45, 66, 79, 10, 3],
        };

        assert_eq!(calc_in_boxes(input), 5);
    }

    #[test]
    fn test_calc_in_boxes3() {
        let input = InputData {
            box_width: 1,
            block_num: 1,
            block_width: vec![1],
        };

        assert_eq!(calc_in_boxes(input), 1);
    }
}
