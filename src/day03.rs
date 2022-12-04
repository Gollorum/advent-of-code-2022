use std::error::Error;
use std::fmt::Display;
use crate::utils;
use crate::utils::ErrorMsg;

pub fn run_sample() {
    ErrorMsg::print(run("input/day03_sample.txt"));
}

pub fn run_actual() {
    ErrorMsg::print(run("input/day03.txt"));
}


fn priority_from(c: char) -> Result<u32, char> {
    match c {
        'A'..='Z' => Ok(c as u32 - 'A' as u32 + 27),
        'a'..='z' => Ok(c as u32 - 'a' as u32 + 1),
        _ => Err(c)
    }
}

impl From<char> for ErrorMsg {
    fn from(err: char) -> Self {
        ErrorMsg { wrapped: format!("Invalid char: {}", err.to_string()) }
    }
}

fn run(path: &str) -> Result<(), ErrorMsg> {
    let lines = utils::read_lines(path)?;
    let mut priority_sum: u32 = 0;
    let mut badge_priority_sum: u32 = 0;
    let mut badge_index = 0;
    let mut badge_priority: u64 = !0;
    for line_r in lines {
        let line = line_r?;
        let (left_half, right_half) = line.split_at(line.len() / 2);
        let mut left_acc: u64 = 0;
        for c in left_half.chars() {
            left_acc |= 1 << priority_from(c)?;
        }
        let mut right_acc: u64 = 0;
        for c in right_half.chars() {
            right_acc |= 1 << priority_from(c)?;
        }
        let shared = left_acc & right_acc;
        priority_sum += shared.trailing_zeros();

        badge_priority &= (left_acc | right_acc);
        if badge_index == 2 {
            badge_index = 0;
            badge_priority_sum += badge_priority.trailing_zeros();
            badge_priority = !0;
        } else { badge_index += 1; }
    }
    Ok(println!("Sum is {}, badge sum is {}", priority_sum, badge_priority_sum))
}