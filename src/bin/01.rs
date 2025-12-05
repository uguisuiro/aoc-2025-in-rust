use anyhow::*;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

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
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut current:i32 =50;
        let mut state:i32=50;
        let mut cnt_zero:usize=0;
        for rotate in reader.lines() {
            //println!("{}", rotate?);
            let str=rotate?.to_string();
            if str.starts_with("L") {
                let current_rotate_circle=str.strip_prefix("L");
                match current_rotate_circle {
                    Some(v) => current=v.parse().unwrap(),
                    None => println!("none value"),
                };
                current=current%100;
                state=state-current;

                if state<0 {
                    state=state+100;
                    state=state.abs();
                }
                if state==0 {
                    cnt_zero=cnt_zero+1;
                }
            }else{
                let current_rotate_circle=str.strip_prefix("R");
                match current_rotate_circle {
                    Some(v) => current=v.parse().unwrap(),
                    None => println!("none value"),
                };
                current=current%100;
                state=state+current;
                if state>=100 {
                    state=state%100;
                }
                if state==0 {
                    cnt_zero=cnt_zero+1;
                }
            }
        }


        let answer:usize= cnt_zero;
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
   assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
