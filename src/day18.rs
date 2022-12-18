use std::cmp::min;
use std::iter::Map;
use std::ops::Add;
use std::slice::Iter;
use std::str::FromStr;
use crate::utils::ErrorMsg;
use regex::Regex;
use crate::day18::Type::{Bubble, Exposed, Solid};
use crate::utils;

pub fn run_sample() {
    ErrorMsg::print(run("input/day18_sample.txt"));
}

pub fn run_actual() {
    ErrorMsg::print(run("input/day18.txt"));
}

#[derive(PartialEq, Eq, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
    z: i32
}
lazy_static! {
    static ref NODE_REGEX: Regex = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();
}
impl FromStr for Pos {
    type Err = ErrorMsg;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let node_captures = NODE_REGEX.captures(s).ok_or(ErrorMsg{wrapped:format!("Failed to capture node regex in {s}")})?;
        Ok(Pos {
            x: node_captures[1].parse()?,
            y: node_captures[2].parse()?,
            z: node_captures[3].parse()?
        })
    }
}
impl Add<Pos> for Pos {
    type Output = Self;

    fn add(self, rhs: Pos) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Pos {
    fn new(x: i32, y: i32, z: i32) -> Pos { Pos {x, y, z} }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Type {
    Solid, Exposed, Bubble
}

fn run(path: &str) -> Result<(), ErrorMsg> {
    let neighbors = [Pos::new(-1, 0, 0), Pos::new(1, 0, 0), Pos::new(0, -1, 0), Pos::new(0, 1, 0), Pos::new(0, 0, -1), Pos::new(0, 0, 1)];
    let all_positions = utils::read_lines(path)?.map(|l| l?.parse()).collect::<Result<Vec<Pos>, _>>()?;
    let min_x = all_positions.iter().map(|p| p.x).min().ok_or(ErrorMsg::new("No min x found"))? - 1;
    let max_x = all_positions.iter().map(|p| p.x).max().ok_or(ErrorMsg::new("No max x found"))? + 1;
    let min_y = all_positions.iter().map(|p| p.y).min().ok_or(ErrorMsg::new("No min y found"))? - 1;
    let max_y = all_positions.iter().map(|p| p.y).max().ok_or(ErrorMsg::new("No max y found"))? + 1;
    let min_z = all_positions.iter().map(|p| p.z).min().ok_or(ErrorMsg::new("No min z found"))? - 1;
    let max_z = all_positions.iter().map(|p| p.z).max().ok_or(ErrorMsg::new("No max z found"))? + 1;
    let mut map = vec![vec![vec![Bubble; (max_z - min_z + 1) as usize]; (max_y - min_y + 1) as usize]; (max_x - min_x + 1) as usize];
    let mut to_explore = vec![Pos::new(min_x, min_y, min_z)];
    for p in &all_positions {
        map[(p.x - min_x) as usize][(p.y - min_y) as usize][(p.z - min_z) as usize] = Solid;
    }
    while let Some(now) = to_explore.pop() {
        if now.x < min_x || now.x > max_x || now.y < min_y || now.y > max_y || now.z < min_z || now.z > max_z { continue; }
        let x = (now.x - min_x) as usize;
        let y = (now.y - min_y) as usize;
        let z = (now.z - min_z) as usize;
        if map[x][y][z] == Bubble {
            map[x][y][z] = Exposed;
            for n in neighbors { to_explore.push(now + n) }
        }
    }
    fn type_at(p: Pos, map: &Vec<Vec<Vec<Type>>>, min_x: i32, min_y: i32, min_z: i32) -> Type {
        let x = (p.x - min_x) as usize;
        let y = (p.y - min_y) as usize;
        let z = (p.z - min_z) as usize;
        map[x][y][z]
    }
    let free_faces: usize = all_positions.iter().map(|p| neighbors.iter().filter(|&o| type_at(*p + *o, &map, min_x, min_y, min_z) == Exposed).count()).sum();
    Ok(println!("{free_faces}"))
}