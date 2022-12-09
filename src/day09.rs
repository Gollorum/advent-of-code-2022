use std::cmp::{max, min};
use std::collections::HashSet;
use crate::utils;
use crate::utils::{ErrorMsg, read_lines};

pub fn run_sample_1() {
    ErrorMsg::print(run(2, "input/day09_sample.txt"));
}

pub fn run_sample_2() {
    ErrorMsg::print(run(10, "input/day09_sample_2.txt"));
}

pub fn run_actual_1() {
    ErrorMsg::print(run(2, "input/day09.txt"));
}

pub fn run_actual_2() {
    ErrorMsg::print(run(10, "input/day09.txt"));
}

fn run(rope_len: usize, path: &str) -> Result<(), ErrorMsg> {
    let mut rope_positions = vec![(0i32, 0i32); rope_len];
    let mut tail_visits: HashSet<(i32, i32)> = HashSet::new();
    tail_visits.insert((0,0));
    for line in read_lines(path)? {
        let (x_diff, y_diff) = match line?.split_at(2) {
            ("R ", num) => Ok((num.parse::<i32>()?, 0)),
            ("L ", num) => Ok((-num.parse::<i32>()?, 0)),
            ("U ", num) => Ok((0, num.parse::<i32>()?)),
            ("D ", num) => Ok((0, -num.parse::<i32>()?)),
            (l, r) => Err(ErrorMsg{wrapped: format!("Failed to parse line {} {}", l, r)})
        }?;
        for _ in 0..(x_diff.abs()) {
            rope_positions[0].0 += x_diff.signum();
            for i in 1..rope_len {
                if (rope_positions[i - 1].0 - rope_positions[i].0).abs() >= 2 {
                    rope_positions[i].0 += (rope_positions[i - 1].0 - rope_positions[i].0) / 2;
                    rope_positions[i].1 += min(1, max(-1, (rope_positions[i - 1].1 - rope_positions[i].1)));
                } else if (rope_positions[i - 1].1 - rope_positions[i].1).abs() >= 2 {
                    rope_positions[i].1 += (rope_positions[i - 1].1 - rope_positions[i].1) / 2;
                    rope_positions[i].0 += min(1, max(-1, (rope_positions[i - 1].0 - rope_positions[i].0)));
                }
            }
            tail_visits.insert(rope_positions[rope_len - 1]);
        }
        for _ in 0..(y_diff.abs()) {
            rope_positions[0].1 += y_diff.signum();
            for i in 1..rope_len {
                if (rope_positions[i - 1].1 - rope_positions[i].1).abs() >= 2 {
                    rope_positions[i].1 += (rope_positions[i - 1].1 - rope_positions[i].1) / 2;
                    rope_positions[i].0 += min(1, max(-1, (rope_positions[i - 1].0 - rope_positions[i].0)));
                } else if (rope_positions[i - 1].0 - rope_positions[i].0).abs() >= 2 {
                    rope_positions[i].0 += (rope_positions[i - 1].0 - rope_positions[i].0) / 2;
                    rope_positions[i].1 += min(1, max(-1, (rope_positions[i - 1].1 - rope_positions[i].1)));
                }
            }
            tail_visits.insert(rope_positions[rope_len - 1]);
        }
    }
    Ok(println!("Visited {} positions", tail_visits.len()))
}