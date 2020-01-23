//! No.24 数当てゲーム
//! https://yukicoder.me/problems/no/24

use std::io::stdin;

/// 入力データ
#[derive(Debug)]
struct InputData {
    /// データの個数
    count: i32,
    /// 回答
    answers: Vec<([i32; 4], bool)>,
}

/// エントリポイント
fn main() {
    let input = input_data();
    println!("{}", correct(input));
}

/// 回答の列挙体
#[derive(PartialEq)]
enum YesNo {
    /// 不明
    Unknown,
    /// YES
    YES,
    /// NO
    NO,
}

/// 回答から正解を導出します。
fn correct(input: InputData) -> i32 {
    // 正解候補（0:不明、1:YES、2:NO）
    let mut candidates: [YesNo; 10] = [
        YesNo::Unknown,
        YesNo::Unknown,
        YesNo::Unknown,
        YesNo::Unknown,
        YesNo::Unknown,
        YesNo::Unknown,
        YesNo::Unknown,
        YesNo::Unknown,
        YesNo::Unknown,
        YesNo::Unknown,
    ];

    for ans in input.answers {
        if !ans.1 {
            // NO
            for n in ans.0.iter() {
                candidates[*n as usize] = YesNo::NO;
            }
        } else {
            // YES
            for i in 0..10 {
                let find = match ans.0.iter().find(|&&x| x == i as i32) {
                    Some(_) => true,
                    None => false,
                };

                candidates[i] = match candidates[i] {
                    YesNo::Unknown if find => YesNo::YES,
                    YesNo::Unknown => YesNo::Unknown,
                    // answersに数字があればYES、なければNO
                    // 1度YESになっても、つぎにYESにならなければNO
                    YesNo::YES if find => YesNo::YES,
                    YesNo::YES => YesNo::NO,
                    YesNo::NO => YesNo::NO,
                }
            }
        }
    }

    // 結果を探す
    for (i, v) in candidates.iter().enumerate() {
        if *v == YesNo::YES {
            return i as i32;
        }
    }

    for (i, v) in candidates.iter().enumerate() {
        if *v == YesNo::Unknown {
            return i as i32;
        }
    }

    // ここには来ない
    0
}

/// 標準入力から文字列を取得します。
fn input_data() -> InputData {
    // 1行目
    let mut s1 = String::new();
    stdin().read_line(&mut s1).unwrap();
    let count: i32 = s1.trim().parse().unwrap();

    // 2行目以降
    let mut answers = Vec::new();

    for _i in 0..count {
        let mut s2 = String::new();
        stdin().read_line(&mut s2).unwrap();

        // 空白で分割してVecに格納
        let sp: Vec<&str> = s2.trim().split_whitespace().collect();

        let array: [i32; 4] = [
            sp[0].parse().unwrap(),
            sp[1].parse().unwrap(),
            sp[2].parse().unwrap(),
            sp[3].parse().unwrap(),
        ];
        let yn = if sp[4] == "YES" { true } else { false };

        answers.push((array, yn));
    }

    InputData { count, answers }
}

#[cfg(test)]
mod tests {
    use crate::correct;
    use crate::InputData;

    #[test]
    fn test_correct1() {
        let ary1 = [1, 2, 4, 3];
        let ary2 = [8, 5, 6, 7];
        let ary3 = [0, 1, 2, 3];
        let input = InputData {
            count: 3,
            answers: vec![(ary1, false), (ary2, false), (ary3, false)],
        };

        assert_eq!(correct(input), 9);
    }

    #[test]
    fn test_correct2() {
        let ary1 = [1, 2, 3, 4];
        let ary2 = [4, 5, 6, 7];
        let input = InputData {
            count: 2,
            answers: vec![(ary1, true), (ary2, true)],
        };

        assert_eq!(correct(input), 4);
    }

    #[test]
    fn test_correct3() {
        let ary1 = [2, 6, 5, 3];
        let ary2 = [1, 0, 4, 7];
        let ary3 = [1, 7, 8, 4];
        let ary4 = [7, 1, 9, 8];

        let input = InputData {
            count: 4,
            answers: vec![(ary1, false), (ary2, true), (ary3, true), (ary4, false)],
        };

        assert_eq!(correct(input), 4);
    }

    #[test]
    fn test_correct4() {
        let ary1 = [6, 8, 1, 7];
        let ary2 = [4, 0, 3, 9];
        let ary3 = [4, 3, 0, 5];

        let input = InputData {
            count: 3,
            answers: vec![(ary1, false), (ary2, true), (ary3, false)],
        };

        assert_eq!(correct(input), 9);
    }

    #[test]
    fn test_correct5() {
        let ary1 = [6, 3, 0, 7];
        let ary2 = [6, 3, 7, 9];

        let input = InputData {
            count: 2,
            answers: vec![(ary1, false), (ary2, true)],
        };

        assert_eq!(correct(input), 9);
    }
}
