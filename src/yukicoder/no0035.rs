//! No.35 タイパー高橋
//! https://yukicoder.me/problems/no/35

use std::io::stdin;

/// 入力データ
struct InputData {
    /// ゲーム数
    _count: i32,
    /// 問題
    questions: Vec<Question>,
}

/// 問題
struct Question {
    /// 制限時間
    limit: i32,
    /// 入力すべき文字列
    type_str: String,
}

/// エントリポイント
fn main() {
    let input = input_data();
    let result = typing(input);
    println!("{} {}", result.0, result.1);
}

/// タイピングゲームの結果を返します。
/// # 戻り値
/// 以下の値を格納したタプル
/// 0: 正しくタイプできる文字数
/// 1: タイプできずに逃してしまう文字数
fn typing(input: InputData) -> (i32, i32) {
    let mut ok_type = 0;
    let mut ng_type = 0;

    for qu in input.questions {
        let type_num = 12 * qu.limit / 1000;

        let str_len = qu.type_str.len() as i32;

        if type_num >= str_len {
            ok_type += str_len;
        } else {
            ok_type += type_num;
            ng_type += str_len - type_num;
        }
    }
    (ok_type, ng_type)
}

/// 標準入力から文字列を取得します。
fn input_data() -> InputData {
    // 1行目
    let mut count = String::new();
    stdin().read_line(&mut count).unwrap();
    let count: i32 = count.trim().parse().unwrap();

    // 2行目以降
    let mut questions = Vec::new();
    for _i in 0..count {
        let mut q = String::new();
        stdin().read_line(&mut q).unwrap();

        let sp: Vec<&str> = q.trim().split_whitespace().collect();
        questions.push(Question {
            limit: sp[0].trim().parse().unwrap(),
            type_str: sp[1].to_string(),
        });
    }
    InputData {
        _count: count,
        questions,
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn typing_test1() {
        let input = InputData {
            _count: 2,
            questions: vec![
                Question {
                    limit: 750,
                    type_str: "yukicoder".to_string(),
                },
                Question {
                    limit: 749,
                    type_str: "yukicoder".to_string(),
                },
            ],
        };
        assert_eq!((17, 1), typing(input));
    }

    #[test]
    fn typing_test2() {
        let input = InputData {
            _count: 4,
            questions: vec![
                Question {
                    limit: 83,
                    type_str: "topcoder".to_string(),
                },
                Question {
                    limit: 417,
                    type_str: "atcoder".to_string(),
                },
                Question {
                    limit: 29183,
                    type_str: "yukicoder".to_string(),
                },
                Question {
                    limit: 1,
                    type_str: "codeforces".to_string(),
                },
            ],
        };
        assert_eq!((14, 20), typing(input));
    }
}
