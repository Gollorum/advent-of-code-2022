use std::cmp::{max, min};
use std::collections::HashMap;
use std::str::FromStr;
use crate::utils;
use crate::utils::ErrorMsg;

pub fn run_sample() {
    ErrorMsg::print(run("input/day17_sample.txt"));
}

pub fn run_actual() {
    ErrorMsg::print(run("input/day17.txt"));
}

fn run(path: &str) -> Result<(), ErrorMsg> {
    let rocks = vec![
        (vec![
            0b1111u8
        ], 4u8),(vec![
            0b010u8,
            0b111u8,
            0b010u8
        ], 3u8),(vec![
            0b111u8,
            0b001u8,
            0b001u8
        ], 3u8),(vec![
            0b1u8,
            0b1u8,
            0b1u8,
            0b1u8
        ], 1u8),(vec![
            0b11u8,
            0b11u8
        ], 2u8)
    ];
    let mut chamber: Vec<u8> = Vec::new();
    let wind_dirs = utils::read_lines(path)?.nth(0)
        .ok_or(ErrorMsg::new("Input had no lines"))??
        .chars().map(|c| match c {
            '<' => Ok(-1),
            '>' => Ok(1),
            other => Err(ErrorMsg{wrapped:format!("No wind dir: {other}")})
        }).collect::<Result<Vec<i16>, ErrorMsg>>()?;

    fn overlaps(chamber: &Vec<u8>, rock: &(Vec<u8>, u8), pos: (i16,i64)) -> bool {
        if pos.0 < 0 || (rock.1 + pos.0 as u8) > 7u8 || pos.1 < 0 { return true; }
        if pos.1 >= chamber.len() as i64 {return false;}
        for y in 0..min(rock.0.len(), chamber.len() - pos.1 as usize) {
            if rock.0[y] << (7 - pos.0 as u8 - rock.1) & chamber[y + pos.1 as usize] != 0 {return true;}
        }
        return false;
    }
    let mut height_offset = 0;
    let mut current_rock_index = 0usize;
    let mut current_wind_index = 0usize;
    let mut skip_cache: HashMap<(usize, usize), (u64, usize)> = HashMap::new();
    // let total_iteration_count = 10u64;
    // let total_iteration_count = 2022u64;
    let total_iteration_count = 1000000000000u64;
    let mut i = 0;
    while i < total_iteration_count {
        if chamber.len() > 0 && *chamber.last().unwrap() == 0b1111111u8 {
            println!("Cleanung up at i={i}, height={}, rock is {}, wind index is {current_wind_index}", chamber.len(), current_rock_index);
            height_offset += chamber.len();
            chamber.clear();
            if let Some((last_i, last_height)) = skip_cache.get(&(current_rock_index, current_wind_index)) {
                println!("Last skip cache was {current_rock_index}, {current_wind_index}, {last_i} {last_height}");
                // let repetitions = 2;
                let diff = height_offset - last_height;
                let repetitions = (total_iteration_count - i) / (i - last_i);
                println!("Was at {i}, {height_offset}, skip {repetitions} iterations to {} with {}", i + repetitions * (i - last_i), height_offset + diff * repetitions as usize);
                height_offset += diff * repetitions as usize;
                i = i + repetitions * (i - last_i);
                // last_skip_cache = Some((current_rock_index, current_wind_index, i));
                continue;
            }
            skip_cache.insert((current_rock_index, current_wind_index), (i, height_offset));
            // println!("New skip cache is {current_rock_index}, {current_wind_index}, {new_offset}, {i}");
        }
        if i > 0 && i & (i - 1) == 0 { println!("Loop {i}")}
        let current_rock = &rocks[current_rock_index];
        current_rock_index = (current_rock_index+1) % rocks.len();
        let mut rock_pos: (i16,i64) = (2, chamber.len() as i64+3);
        // rock_pos.0 +=
        //     wind_dirs[current_wind_index] +
        //     wind_dirs[(current_wind_index+1) % wind_dirs.len()] +
        //     wind_dirs[(current_wind_index+2) % wind_dirs.len()];
        // current_wind_index = (current_wind_index+3) % wind_dirs.len();
        // if rock_pos.0 < 0 { rock_pos.0 = 0;}
        // else if rock_pos.0 + current_rock.1 as i16 > 7 { rock_pos.0 = 7 - current_rock.1 as i16; }
        loop {
            let new_pos = (rock_pos.0 + wind_dirs[current_wind_index], rock_pos.1);
            current_wind_index = (current_wind_index+1) % wind_dirs.len();
            if !overlaps(&chamber, &current_rock, new_pos) {
                rock_pos = new_pos;
            }
            let new_pos = (rock_pos.0, rock_pos.1 - 1);
            if overlaps(&chamber, &current_rock, new_pos) {
                for _ in (chamber.len() as i64)..(rock_pos.1 + current_rock.0.len() as i64) { chamber.push(0) }
                for y in 0..current_rock.0.len() {
                    chamber[y + rock_pos.1 as usize] |= current_rock.0[y] << (7 - rock_pos.0 as u8 - current_rock.1);
                }
                break;
            }
            rock_pos = new_pos;
        }
        i = i + 1;
    }
    // println!("{}", (0..chamber.len()).rev().map(|y| (0..7).rev().map(|x| if chamber[y] & 1 << x != 0 {'#'} else {'.'}).collect::<String>() + "\n").collect::<String>());
    Ok(println!("Height: {}", chamber.len() + height_offset))
}