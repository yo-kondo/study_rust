//! No.70 睡眠の重要性！
//! https://yukicoder.me/problems/no/70

use std::io::stdin;

/// エントリポイント
fn main() {
    let input = read_lines();
    println!("{}", sleep(input));
}

/// 標準入力から文字列を取得します。
fn read_lines() -> Vec<String> {
    // 1行目
    let mut str1 = String::new();
    stdin().read_line(&mut str1).unwrap();
    let num_count: i32 = str1.trim().parse().unwrap();

    // 2行目以降
    let mut sleep_time = Vec::new();
    // 1行目で取得したデータ件数分ループ
    for _i in 0..num_count {
        let mut str2 = String::new();
        stdin().read_line(&mut str2).unwrap();
        sleep_time.push(str2.trim().to_string());
    }
    sleep_time
}

/// 睡眠時間の合計を分で返します。
fn sleep(input: Vec<String>) -> i32 {
    // 1日の分
    const DAY_MINUTES: i32 = 24 * 60;

    let mut sleep_time = 0;

    for line in input {
        let sp: Vec<&str> = line.split_whitespace().collect();

        let start_time = get_minutes(sp[0]);
        let end_time = get_minutes(sp[1]);

        if start_time < end_time {
            // マイナス値になるので絶対値に変換
            sleep_time += (start_time - end_time).abs();
        } else {
            sleep_time += end_time + DAY_MINUTES - start_time;
        }
    }
    sleep_time
}

/// HH:mm 形式の文字列を分に変換して返します。
fn get_minutes(str_time: &str) -> i32 {
    let sp: Vec<&str> = str_time.split(":").collect();
    let hour: i32 = sp[0].parse().unwrap();
    let min: i32 = sp[1].parse().unwrap();
    hour * 60 + min
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn sleep_test1() {
        let input = vec![
            "22:24 5:9".to_string(),
            "22:46 5:37".to_string(),
            "1:11 7:11".to_string(),
        ];
        assert_eq!(1176, sleep(input));
    }

    #[test]
    fn sleep_test2() {
        let input = vec![
            "22:30 4:30".to_string(),
            "22:46 4:46".to_string(),
            "1:17 7:26".to_string(),
            "0:38 7:0".to_string(),
            "23:49 7:41".to_string(),
        ];
        assert_eq!(1943, sleep(input));
    }
}
