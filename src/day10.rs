use crate::utils;
use crate::utils::ErrorMsg;

pub fn run_sample() {
    ErrorMsg::print(run("input/day10_sample.txt"));
}

pub fn run_actual() {
    ErrorMsg::print(run("input/day10.txt"));
}

fn run(path: &str) -> Result<(), ErrorMsg> {
    const SCREEN_WIDTH: i32 = 40;
    let mut x_reg = 1;
    let mut total_signal_strength = 0;
    let mut cycle_num = 1;
    let mut resulting_image: Vec<char> = Vec::new();
    resulting_image.push('#');
    fn advance_cycle(x_reg: i32, total_signal_strength: i32, cycle_num: i32, image: &mut Vec<char>) -> (i32, i32) {
        let new_cycle_num = cycle_num + 1;
        let x = (new_cycle_num - 1) % SCREEN_WIDTH;
        if x == 0 { image.push('\n'); }
        image.push(if (x_reg - x).abs() <= 1 {'#'} else {' '});
        if (new_cycle_num - 20) % 40 == 0 {
            (total_signal_strength + new_cycle_num * x_reg, new_cycle_num)
        } else {
            (total_signal_strength, new_cycle_num)
        }
    };
    for line_r in utils::read_lines(path)? {
        let line = line_r?;
        if line == "noop" {
            (total_signal_strength, cycle_num) = advance_cycle(x_reg, total_signal_strength, cycle_num, &mut resulting_image);
        } else if line.starts_with("addx") {
            let mut new_pixel = ' ';
            (total_signal_strength, cycle_num) = advance_cycle(x_reg, total_signal_strength, cycle_num, &mut resulting_image);
            let num = line[5..].parse::<i32>()?;
            x_reg += num;
            (total_signal_strength, cycle_num) = advance_cycle(x_reg, total_signal_strength, cycle_num, &mut resulting_image);
        } else {
            Err(ErrorMsg{wrapped: format!("Failed to parse line {}", line)})?;
        }
    }
    Ok(println!("Total signal was {total_signal_strength}\nimage: \n{}", resulting_image.iter().collect::<String>()))
}