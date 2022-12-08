use crate::utils;
use crate::utils::ErrorMsg;

pub fn run_sample() {
    ErrorMsg::print(run("input/day08_sample.txt"));
}

pub fn run_actual() {
    ErrorMsg::print(run("input/day08.txt"));
}

fn run(path: &str) -> Result<(), ErrorMsg> {
    let lines: Vec<String> = utils::read_lines_to_vec(path)?;
    let height = lines.len();
    let width = lines[0].len();
    let mut tree_heights = vec![vec![0i8; width]; height];
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            tree_heights[y][x] = c.to_digit(10).ok_or(ErrorMsg {wrapped: format!("Failed to parse char {}", c)})? as i8
        }
    }

    // part 1
    let mut tree_visibility = vec![vec![false; width]; height];
    for (y, row) in tree_heights.iter().enumerate() {
        let mut max = -1i8;
        for (x, height) in row.iter().enumerate() {
            if *height > max {
                tree_visibility[y][x] = true;
                max = *height;
            }
        }
        max = -1i8;
        // println!("Inverse traversing row {:?}, starting at {max}", row);
        for x in (0..width).rev() {
            let height = row[x];
            // println!("{height}");
            if height > max {
                tree_visibility[y][x] = true;
                max = height;
            }
        }
    }
    for x in 0..width {
        let mut max = -1i8;
        for (y, row) in tree_heights.iter().enumerate() {
            let height = row[x];
            if height > max {
                tree_visibility[y][x] = true;
                max = height;
            }
        }
        max = -1i8;
        for y in (0..height).rev() {
            let height = tree_heights[y][x];
            if height > max {
                tree_visibility[y][x] = true;
                max = height;
            }
        }
    }

    // part 2
    let mut tree_scenic_view = vec![vec![1u32; width]; height];
    let scenic_view_in_dir = |max_height: i8, trees_to_consider: Vec<(usize, usize)>| -> u32 {
        let mut ret = 0u32;
        for (x, y) in trees_to_consider {
            ret += 1;
            if tree_heights[y][x] >= max_height { return ret; }
        }
        return ret;
    };
    for (y, row) in tree_heights.iter().enumerate() {
        for (x, h) in row.iter().enumerate() {
            tree_scenic_view[y][x] =
                scenic_view_in_dir(*h, (0..=x).rev().map(|xn| (xn, y)).skip(1).collect()) *
                scenic_view_in_dir(*h, (x..width).map(|xn| (xn, y)).skip(1).collect()) *
                scenic_view_in_dir(*h, (0..=y).rev().map(|yn| (x, yn)).skip(1).collect()) *
                scenic_view_in_dir(*h, (y..height).map(|yn| (x, yn)).skip(1).collect())
        }
    }

    // Ok({
    //     tree_visibility.into_iter().for_each(|it| {
    //         println!("{:?}", it);
    //     });
    //     tree_heights.into_iter().for_each(|it| {
    //         println!("{:?}", it);
    //     });
    // })
    Ok(println!("{} trees are visible and the best scenic view is {}",
        tree_visibility.iter().map(|row| row.iter().filter(|b| **b).count()).sum::<usize>(),
        tree_scenic_view.iter().map(|row| row.iter().max().ok_or(ErrorMsg::new("Row has no max"))).collect::<Result<Vec<&u32>, ErrorMsg>>()?.iter().max().ok_or(ErrorMsg::new("Forest has no max"))?
    ))
}