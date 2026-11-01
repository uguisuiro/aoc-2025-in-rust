use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

// 日付
const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

// テストケース
const TEST: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
fn main() -> Result<()> {
    start_day(DAY);

    //パート1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // パート1の実装
        let mut current: i32 = 50;
        let mut state: i32 = 50;
        let mut cnt_zero: usize = 0;
        for rotate in reader.lines() {
            let str = rotate?.to_string();
            if str.starts_with("L") {
                let current_rotate_circle = str.strip_prefix("L");
                match current_rotate_circle {
                    Some(v) => current = v.parse().unwrap(),
                    None => println!("none value"),
                };
                current = current % 100;
                state = state - current;

                if state < 0 {
                    state = state + 100;
                    state = state.abs();
                }
                if state == 0 {
                    cnt_zero = cnt_zero + 1;
                }
            } else {
                let current_rotate_circle = str.strip_prefix("R");
                match current_rotate_circle {
                    Some(v) => current = v.parse().unwrap(),
                    None => println!("none value"),
                };
                current = current % 100;
                state = state + current;
                if state >= 100 {
                    state = state % 100;
                }
                if state == 0 {
                    cnt_zero = cnt_zero + 1;
                }
            }
        }

        let answer: usize = cnt_zero;
        Ok(answer)
    }

    // テストケースの答えと一致するかを確認
    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    // パート1はここまで

    //パート2はここから
    println!("\n=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // パート2の実装
        let mut current: i32 = 50;
        let mut state: i32 = 50;
        let mut cnt_zero: i32 = 0;
        for rotate in reader.lines() {
            let str = rotate?.to_string();
            if str.starts_with("R") {
                let current_rotate_circle = str.strip_prefix("R");
                match current_rotate_circle {
                    Some(v) => current = v.parse().unwrap(),
                    None => println!("none value"),
                }
                state = state + current;
                if state < 0 {
                    cnt_zero = cnt_zero + 1;
                }
                while state.abs() >= 100 {
                    state = state - 100;
                    cnt_zero = cnt_zero + 1;
                }
                state = state.abs();
            } else {
                let current_rotate_circle = str.strip_prefix("L");
                match current_rotate_circle {
                    Some(v) => current = v.parse().unwrap(),
                    None => println!("none value"),
                };
                if current == 0 {
                    continue;
                }
                if state == 0 {
                    state = state - current;
                    if state < 0 {
                        state = state + 100;
                    }
                } else {
                    state = state - current;
                    if state < 0 {
                        cnt_zero = cnt_zero + 1;
                        state = state + 100;
                    }
                }
                while state < 0 {
                    state = state + 100;
                    cnt_zero = cnt_zero + 1;
                }
                state = state.abs();
                if state == 0 {
                    cnt_zero = cnt_zero + 1;
                }
            }
        }
        let answer: usize = cnt_zero as usize;
        Ok(answer)
    }
    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
