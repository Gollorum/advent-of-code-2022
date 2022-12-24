use std::cmp::{max, min, Ordering};
use std::collections::binary_heap::BinaryHeap;
use std::collections::{HashSet};
use std::ops::Add;
use crate::day24::Dir::{Down, Left, Right, Up};
use crate::utils;
use crate::utils::ErrorMsg;

pub fn run_sample() {
    ErrorMsg::print(run("input/day24_sample.txt"));
}

pub fn run_actual() {
    ErrorMsg::print(run("input/day24.txt"));
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Dir {
    Up, Right, Down, Left
}

struct Blizzard {
    pos: Pos,
    dir: Dir
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize
}

impl Add<(Dir, (usize,usize))> for Pos {
    type Output = Pos;

    fn add(self, rhs: (Dir, (usize,usize))) -> Self::Output {
        match rhs.0 {
            Right => Pos { x: (self.x % rhs.1.0) + 1, y: self.y},
            Down => Pos { y: (self.y % rhs.1.1) + 1, x: self.x},
            Left => Pos { x: ((self.x + rhs.1.0 - 2) % rhs.1.0) + 1, y: self.y},
            Up => Pos { y: ((self.y + rhs.1.1 - 2) % rhs.1.1) + 1, x: self.x},
        }
    }
}
impl Add<(i32, i32)> for Pos {
    type Output = Option<Pos>;
    fn add(self, rhs: (i32, i32)) -> Option<Pos> {
        if (rhs.1 < 0 && self.y == 0) || rhs.0 < 0 && self.x == 0 {
            None
        } else {
            Some(Pos {
                x: (self.x as i32 + rhs.0) as usize,
                y: (self.y as i32 + rhs.1) as usize,
            })
        }
    }
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct State {
    player_pos: Pos,
    blizzard_index: usize,
    cost_so_far: usize,
    total_estimated_cost: usize
}
impl State {
    fn new(player_pos: Pos, target_pos: Pos, blizzard_index: usize, cost_so_far: usize) -> State {
        State {
            player_pos,
            blizzard_index,
            cost_so_far,
            total_estimated_cost: cost_so_far
                + max(target_pos.x,player_pos.x) - min(target_pos.x,player_pos.x)
                + max(target_pos.y,player_pos.y) - min(target_pos.y,player_pos.y)
        }
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.total_estimated_cost.cmp(&self.total_estimated_cost)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn time_to_reach(start_pos: Pos, target_pos: Pos, blizzard_index: usize, map_at: &Vec<Vec<Vec<bool>>>) -> Result<usize, ErrorMsg> {
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(State::new(start_pos, target_pos, blizzard_index, 0));
    let mut has_been_explored: HashSet<(usize,usize,usize)> = HashSet::new();
    let explore_options: [(i32, i32); 5] = [(1,0), (0,1), (0,0), (0,-1), (-1,0)];
    while let Some(state) = queue.pop() {
        if has_been_explored.contains(&(state.player_pos.x, state.player_pos.y, state.blizzard_index)) {
            continue;
        }
        if state.player_pos == target_pos {
            return Ok(state.cost_so_far)
        }
        let next_blizzard_i = (state.blizzard_index + 1) % map_at.len();
        for next_pos in explore_options.iter().filter_map(|&offset| state.player_pos + offset).filter(|pos| pos.y < map_at[next_blizzard_i].len() && !map_at[next_blizzard_i][pos.y][pos.x]) {
            queue.push(State::new(
                next_pos,
                target_pos,
                next_blizzard_i,
                state.cost_so_far + 1
            ))
        }
        has_been_explored.insert((state.player_pos.x, state.player_pos.y, state.blizzard_index));
    }
    Err(ErrorMsg::new("Did not find the target. Sad."))
}

fn run(path: &str) -> Result<(), ErrorMsg> {
    let mut blizzards: Vec<Blizzard> = Vec::new();
    let mut map: Vec<Vec<bool>> = Vec::new();
    for (y, line_r) in utils::read_lines(path)?.enumerate() {
        let mut row = Vec::new();
        let line = line_r?;
        for (x, c) in line.chars().enumerate() {
            let pos = Pos {x, y};
            match c {
                '>' => Ok({ blizzards.push(Blizzard { pos, dir: Right }); row.push(false) }),
                '<' => Ok({ blizzards.push(Blizzard { pos, dir: Left }); row.push(false) }),
                '^' => Ok({ blizzards.push(Blizzard { pos, dir: Up }); row.push(false) }),
                'v' => Ok({ blizzards.push(Blizzard { pos, dir: Down }); row.push(false) }),
                '.' => Ok(row.push(false)),
                '#' => Ok(row.push(true)),
                _ => Err(ErrorMsg{wrapped:format!("Not a valid char: {c}")})
            }?
        }
        map.push(row);
    }
    let num_states = lcm(map.len() - 2, map[0].len() - 2);
    let mut blizzards_at: Vec<Vec<Blizzard>> = Vec::new();
    blizzards_at.push(blizzards);
    let blizzard_width = map[0].len() - 2;
    let blizzard_height = map.len() - 2;
    for i in 1..num_states {
        let new_blizzards: Vec<Blizzard> = blizzards_at[i-1].iter().map(|b| Blizzard {
            pos: b.pos + (b.dir, (blizzard_width, blizzard_height)),
            dir: b.dir
        }).collect();
        blizzards_at.push(new_blizzards)
    }
    let map_at = blizzards_at.iter().map(|current_blizzards| map.iter().enumerate()
        .map(|(y,row)| row.iter().enumerate().map(|(x, &is_wall)| is_wall || current_blizzards.iter().any(|b| b.pos == Pos {x, y})
        ).collect::<Vec<_>>()).collect::<Vec<_>>()).collect::<Vec<_>>();
    let start_pos = Pos {x: 1, y: 0};
    let target_pos = Pos {x: map[0].len()-2, y: map.len()-1};
    let time_to_reach_target = time_to_reach(start_pos, target_pos, 0, &map_at)?;
    println!("Found target in {time_to_reach_target}");
    let time_to_go_back = time_to_reach(target_pos, start_pos, time_to_reach_target % map_at.len(), &map_at)?;
    println!("Got back in {time_to_go_back}");
    let time_to_return = time_to_reach(start_pos, target_pos, (time_to_reach_target + time_to_go_back) % map_at.len(), &map_at)?;
    println!("Returned in {time_to_return}");
    Ok(println!("Total travel time: {}", time_to_reach_target + time_to_go_back + time_to_return))
}