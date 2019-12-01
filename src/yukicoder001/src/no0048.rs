//! No.48 ロボットの操縦
//! https://yukicoder.me/problems/no/48

use std::io::stdin;

/// エントリポイント
fn main() {
    let in1 = read_line();
    let in2 = read_line();
    let in3 = read_line();
    println!("{}", count_order(in1, in2, in3));
}

/// 移動先に到達するまでに必要な最小の命令回数を計算します。
///
/// ## arguments
/// * `east_west` - 目的地の東西方向
/// * `north_south` - 目的地の南北方向
/// * `max_distance` - ロボットが1命令につき前進することができる最大の距離
fn count_order(east_west: String, north_south: String, max_distance: String) -> i32 {
    let mut num = 0;
    let ew: f64 = east_west.trim().parse().unwrap();
    let ns: f64 = north_south.trim().parse().unwrap();
    let md: f64 = max_distance.trim().parse().unwrap();

    // 北
    if ns > 0.0 {
        num += (ns / md).abs().ceil() as i32;
    }

    // 東西
    if ew != 0.0 {
        num += 1;
        num += (ew / md).abs().ceil() as i32;
    }

    // 北から南へ向く場合、いったん東西へ向く
    if ew == 0.0 && ns < 0.0 {
        num += 1;
    }

    // 南
    if ns < 0.0 {
        num += 1;
        num += (ns / md).abs().ceil() as i32;
    }

    num
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
    fn count_order_test() {
        assert_eq!(
            2,
            count_order("0".to_string(), "2".to_string(), "1".to_string())
        );
        assert_eq!(
            5,
            count_order("-7".to_string(), "15".to_string(), "7".to_string())
        );
    }
}
