use regex::Regex;
use crate::utils;
use crate::utils::ErrorMsg;

pub fn run_sample() {
    ErrorMsg::print(run("input/day05_sample.txt"));
}

pub fn run_actual() {
    ErrorMsg::print(run("input/day05.txt"));
}

struct Instruction {
    amount: u16,
    from: usize,
    to: usize
}

fn run(path: &str) -> Result<(), ErrorMsg> {
    let mut lines = utils::read_lines(path)?;
    let mut initial_state: Vec<Vec<char>> = Vec::new();
    while let Some(stack_line_r) = lines.next() {
        let stack_line = stack_line_r?;
        if stack_line == "" {break;}
        for i in 0..((stack_line.len() + 1) / 4) {
            if initial_state.len() <= i { initial_state.push(Vec::new()) }
            let char = stack_line.chars().nth(i * 4 + 1).ok_or(ErrorMsg{wrapped: "Instruction ended unexpectedly".to_string()})?;
            if char != ' ' { initial_state[i].insert(0, char) }
        }
    }

    let mut instructions: Vec<Instruction> = Vec::new();
    let instruction_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)")?;
    while let Some(formatted_instruction_r) = lines.next() {
        let formatted_instruction = formatted_instruction_r?;
        let captures = instruction_regex.captures(formatted_instruction.as_str()).ok_or(ErrorMsg{wrapped: format!("Regex failed on '{}'", formatted_instruction)})?;
        instructions.push(Instruction{
            amount: captures[1].parse::<u16>()?,
            from: captures[2].parse::<usize>()? - 1,
            to: captures[3].parse::<usize>()? - 1
        })
    }

    let mut state_v1 = initial_state.clone();
    let mut state_v2 = initial_state.clone();
    for instruction in instructions {
        for _ in 0..instruction.amount {
            let moved_box = state_v1[instruction.from].pop().ok_or(ErrorMsg{wrapped: "Failed to remove top element from stack: Nothing left".to_string()})?;
            state_v1[instruction.to].push(moved_box)
        }

        let v2_index = state_v2[instruction.to].len();
        for _ in 0..instruction.amount {
            let moved_box = state_v2[instruction.from].pop().ok_or(ErrorMsg{wrapped: "Failed to remove top element from stack: Nothing left".to_string()})?;
            state_v2[instruction.to].insert(v2_index, moved_box)
        }
    }
    let format_top: fn(&Vec<Vec<char>>) -> String = |state_vec| state_vec.iter().map(|stack| stack.last().unwrap_or(&' ')).collect::<String>();
    Ok(println!("{} | {}", format_top(&state_v1), format_top(&state_v2)))
}