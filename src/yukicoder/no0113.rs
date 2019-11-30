//! No.113 宝探し
//! https://yukicoder.me/problems/no/113

use std::io::stdin;

/// エントリポイント
fn main() {
    let input = read_lines();
    println!("{}", treasure_hunt(input));
}

/// 標準入力から文字列を取得します。
fn read_lines() -> String {
    let mut str1 = String::new();
    stdin().read_line(&mut str1).unwrap();
    str1
}

/// 宝の位置までの最短の距離を返します。
fn treasure_hunt(direction: String) -> f64 {
    // 0:NS, 1:EW
    let mut dir_count = [0f64; 2];
    for d in direction.trim().chars() {
        match d {
            'N' => dir_count[0] += 1f64,
            'S' => dir_count[0] -= 1f64,
            'E' => dir_count[1] += 1f64,
            'W' => dir_count[1] -= 1f64,
            _ => panic!(),
        }
    }
    // 南東（NE）
    let ne = dir_count[0].powf(2.0);
    // 北西（SW）
    let sw = dir_count[1].powf(2.0);
    (ne + sw).sqrt()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn treasure_hunt_test1() {
        let expected = "1.41421".to_string();
        let actual = format!("{:.5}", treasure_hunt("NE".to_string()));
        assert_eq!(expected, actual);
    }

    #[test]
    fn treasure_hunt_test2() {
        let expected = "1.41421".to_string();
        let actual = format!("{:.5}", treasure_hunt("EN".to_string()));
        assert_eq!(expected, actual);
    }

    #[test]
    fn treasure_hunt_test3() {
        let expected = "5.00000".to_string();
        let actual = format!("{:.5}", treasure_hunt("SSSSWWW".to_string()));
        assert_eq!(expected, actual);
    }

    #[test]
    fn treasure_hunt_test4() {
        let expected = "4.47214".to_string();
        let actual = format!("{:.5}", treasure_hunt("ESSSSEWENWNNEWWEESSS".to_string()));
        assert_eq!(expected, actual);
    }
}
