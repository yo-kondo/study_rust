//! No.29 パワーアップ
//! https://yukicoder.me/problems/no/29

use std::collections::HashMap;
use std::io;

/// 入力データ
#[derive(Debug)]
struct InputData {
    /// 敵を倒す回数
    count: i32,
    /// 敵を倒した時のもらえる３つのアイテムの番号
    items: Vec<[i32; 3]>,
}

/// エントリポイント
fn main() {
    let input = input_data();
    println!("{}", power_up(input));
}

/// パワーアップする最大の回数を求めます。
fn power_up(input: InputData) -> i32 {
    // アイテムを1つのVecに変換
    let mut array = Vec::new();
    for i in input.items {
        array.push(i[0]);
        array.push(i[1]);
        array.push(i[2]);
    }

    // グルーピング
    let mut map = HashMap::new();
    for v in array {
        *map.entry(v).or_insert(0) += 1;
    }

    let mut level = 0;

    // 2つのアイテムでレベルアップしたあとの、残りのアイテム
    let mut items = Vec::new();

    for (key, val) in map {
        // 「同じアイテム」を2つ
        level += val / 2;

        if val % 2 != 0 {
            items.push(key);
        }
    }

    // 「任意のアイテム」を4つ
    level += (items.len() / 4) as i32;

    level
}

/// 標準入力から文字列を取得します。
fn input_data() -> InputData {
    // 1行目
    let mut count = String::new();
    io::stdin().read_line(&mut count).unwrap();
    let count: i32 = count.trim().parse().unwrap();

    // 2行目以降
    let mut items = Vec::new();
    for _i in 0..count {
        let mut item = String::new();
        io::stdin().read_line(&mut item).unwrap();

        let sp: Vec<&str> = item.trim().split_whitespace().collect();
        let array: [i32; 3] = [
            sp[0].parse().unwrap(),
            sp[1].parse().unwrap(),
            sp[2].parse().unwrap(),
        ];
        items.push(array);
    }

    InputData { count, items }
}

#[cfg(test)]
mod tests {
    use crate::power_up;
    use crate::InputData;

    #[test]
    fn test_power_up1() {
        let input = InputData {
            count: 5,
            items: vec![[1, 2, 3], [4, 5, 6], [7, 8, 9], [10, 1, 2], [3, 4, 5]],
        };

        assert_eq!(power_up(input), 6);
    }

    #[test]
    fn test_power_up2() {
        let input = InputData {
            count: 3,
            items: vec![[1, 1, 1], [1, 1, 1], [1, 1, 1]],
        };

        assert_eq!(power_up(input), 4);
    }

    #[test]
    fn test_power_up3() {
        let input = InputData {
            count: 3,
            items: vec![[1, 2, 3], [5, 4, 1], [1, 9, 2]],
        };

        assert_eq!(power_up(input), 3);
    }
}
