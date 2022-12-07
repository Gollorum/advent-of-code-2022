use crate::utils;
use crate::utils::ErrorMsg;

pub fn run_sample(marker_len: usize) {
    ErrorMsg::print(run(marker_len, "input/day06_sample.txt"));
}

pub fn run_actual(marker_len: usize) {
    ErrorMsg::print(run(marker_len, "input/day06.txt"));
}

fn are_distinct(init: &Vec<char>, last: char) -> bool {
    let mut accum: u32 = 1 << (last as u32 - 'a' as u32);
    for &c in init {
        let curr = 1 << (c as u32 - 'a' as u32);
        if (accum & curr) != 0 { return false; }
        accum |= curr;
    }
    return true;
}

fn run(marker_len: usize, path: &str) -> Result<(), ErrorMsg> {
    let line = utils::read_lines(path)?.next()
        .ok_or(ErrorMsg{wrapped: "No lines read".to_string()})??;
    let mut queue: Vec<char> = Vec::new();
    for (i, c) in line.chars().enumerate() {
        if i >= marker_len - 1 {
            if are_distinct(&queue, c) {
                return Ok(println!("{}", i + 1));
            }
            queue.remove(0);
        }
        queue.push(c);
    }
    Err(ErrorMsg{wrapped: "Iterated whole signal without finding marker".to_string()})
}