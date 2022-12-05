use regex::Regex;
use crate::utils;
use crate::utils::ErrorMsg;

pub fn run_sample() {
    ErrorMsg::print(run("input/day04_sample.txt"));
}

pub fn run_actual() {
    ErrorMsg::print(run("input/day04.txt"));
}

fn run(path: &str) -> Result<(), ErrorMsg> {
    let lines = utils::read_lines(path)?;
    let mut num_contained = 0;
    let mut num_overlaps = 0;
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)")?;
    for line_r in lines {
        let line = line_r?;
        let captures = re.captures(line.as_str()).ok_or(ErrorMsg{wrapped: "Regex failed".to_string()})?;
        let left = (captures[1].parse::<u32>()?, captures[2].parse::<u32>()?);
        let right = (captures[3].parse::<u32>()?, captures[4].parse::<u32>()?);
        if (left.0 <= right.0 && left.1 >= right.1) || (left.0 >= right.0 && left.1 <= right.1) {
            num_contained += 1;
        }
        if !(left.1 < right.0 || left.0 > right.1) {
            num_overlaps += 1;
        }
    }
    Ok(println!("{} pairs contained each other, {} pairs overlapped", num_contained, num_overlaps))
}