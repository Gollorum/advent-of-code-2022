use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::thread::sleep;
use crate::utils;
use crate::utils::ErrorMsg;

pub fn run_sample() {
    ErrorMsg::print(run("input/day12_sample.txt"));
}

pub fn run_actual() {
    ErrorMsg::print(run("input/day12.txt"));
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct SearchEntry {
    x: usize,
    y: usize,
    total_cost: i32
}
impl Ord for SearchEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.total_cost.cmp(&self.total_cost) // min heap
    }
}
impl PartialOrd for SearchEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl SearchEntry {
    fn neighbors(&self, width: usize, height: usize) -> Vec<SearchEntry> {
        vec![(self.x as i32 - 1, self.y as i32), (self.x as i32, self.y as i32 - 1), (self.x as i32 + 1, self.y as i32), (self.x as i32, self.y as i32 + 1)]
            .iter()
            .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < width as i32 && *y < height as i32)
            .map(|(x, y)| SearchEntry {
                x: *x as usize, y: *y as usize,
                total_cost: self.total_cost + 1
            }).collect::<Vec<SearchEntry>>()
    }
}

fn search(width: usize, height: usize, elevation: &Vec<Vec<i8>>, start: (usize, usize), target_elevation: i8, reverse: bool) -> Result<i32, ErrorMsg> {
    let mut cost_to_reach = vec![vec![-1i32; width]; height];
    let mut has_been_handled = vec![vec![false; width]; height];
    let mut to_search: BinaryHeap<SearchEntry> = BinaryHeap::new();
    cost_to_reach[start.1][start.0] = 0;
    to_search.push(SearchEntry {
        x: start.0,
        y: start.1,
        total_cost: 0
    });
    while let Some(next) = to_search.pop() {
        if elevation[next.y][next.x] == target_elevation {
            return Ok(next.total_cost)
        }
        if has_been_handled[next.y][next.x] { continue; }
        has_been_handled[next.y][next.x] = true;
        let my_elevation = elevation[next.y][next.x];
        for n in next.neighbors(width, height) {
            let elevation_diff = if reverse {
                elevation[n.y][n.x] - my_elevation
            } else { my_elevation - elevation[n.y][n.x] };
            if elevation_diff > 1 { continue; }
            let old_cost = cost_to_reach[n.y][n.x];
            if (old_cost < 0 || old_cost > n.total_cost) && !has_been_handled[n.y][n.x] {
                cost_to_reach[n.y][n.x] = n.total_cost;
                to_search.push(n);
            }
        }
    }
    println!("Costs: \n{}", cost_to_reach.iter().map(|row| row.iter().map(|c| c.to_string() + ", ").collect::<String>() + "\n").collect::<String>());
    println!("Has been handled: \n{}", has_been_handled.iter().map(|row| row.iter().map(|c| (if *c {"1"} else {"0"})).collect::<String>() + "\n").collect::<String>());
    Err(ErrorMsg::new("Did not find the destination"))
}

fn run(path: &str) -> Result<(), ErrorMsg> {
    let lines: Vec<String> = utils::read_lines_to_vec(path)?;
    let height = lines.len();
    let width = lines[0].len();
    let mut elevation = vec![vec![0i8; width]; height];
    let mut start_coord: Result<(usize, usize), ErrorMsg> = Err(ErrorMsg::new("Did not find starting position"));
    let mut target_coord: Result<(usize, usize), ErrorMsg> = Err(ErrorMsg::new("Did not find target position"));
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            elevation[y][x] = match c {
                'S' => Ok(-1),
                'a'..='z' => Ok(c as i8 - 'a' as i8),
                'E' => Ok('z' as i8 - 'a' as i8 + 1),
                _ => Err(ErrorMsg{wrapped: format!("Failed to parse char {c}")})
            }?;
            if c == 'S' { start_coord = Ok((x, y)) }
            if c == 'E' { target_coord = Ok((x, y)) }
        }
    }
    Ok({
        println!("Hill top found in {}", search(width, height, &elevation, start_coord?, 'z' as i8 - 'a' as i8 + 1, true)?);
        println!("Scenic route is {}", search(width, height, &elevation, target_coord?, 0, false)?)
    })
}