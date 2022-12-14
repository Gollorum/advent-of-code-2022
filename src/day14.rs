use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops;
use std::str::FromStr;
use regex::Regex;
use crate::utils;
use crate::utils::ErrorMsg;

pub fn run_sample() {
    ErrorMsg::print(run("input/day14_sample.txt"));
}

pub fn run_actual() {
    ErrorMsg::print(run("input/day14.txt"));
}

#[derive(Clone, Copy, PartialEq)]
struct Point { x: i32, y: i32 }
struct RockStrip { handles: Vec<Point> }

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}
impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("({}|{})", self.x, self.y).as_str())
    }
}

impl FromStr for Point {
    type Err = ErrorMsg;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split_once(',')
            .ok_or(ErrorMsg{wrapped:format!("Failed to find point separator in {s}")})?;
        Ok(Point {
            x: l.parse::<i32>()?,
            y: r.parse::<i32>()?
        })
    }
}
lazy_static! {
    static ref STRIP_REGEX: Regex = Regex::new(r"(\d+,\d+)").unwrap();
}
impl FromStr for RockStrip {
    type Err = ErrorMsg;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let capturs = STRIP_REGEX.captures_iter(s).filter_map(|n| n.get(0).map(|m| m.as_str()));
        let points = capturs.map(|s| s.parse::<Point>()).collect::<Result<Vec<Point>, ErrorMsg>>()?;
        Ok(RockStrip{handles: points})
    }
}

fn run(path: &str) -> Result<(), ErrorMsg> {
    let rock_formations: Vec<RockStrip> = utils::read_lines(path)?.map(|l| l?.parse::<RockStrip>())
        .collect::<Result<Vec<RockStrip>, ErrorMsg>>()?;
    let sand_start = Point {x: 500, y: 0};
    let min_x = rock_formations.iter().flat_map(|s| s.handles.iter().map(|p| p.x)).min().unwrap();
    let max_x = rock_formations.iter().flat_map(|s| s.handles.iter().map(|p| p.x)).max().unwrap();
    let max_y = rock_formations.iter().flat_map(|s| s.handles.iter().map(|p| p.y)).max().unwrap();
    let height = (max_y + 2) as usize;
    let mut is_blocked: HashMap<i32, Vec<bool>> = HashMap::new();
    for (from, to) in rock_formations.iter().flat_map(|strip| strip.handles.iter().zip(strip.handles.iter().skip(1))) {
        for x in min(from.x,to.x)..=max(from.x,to.x) {
            for y in min(from.y,to.y)..=max(from.y,to.y) {
                is_blocked.entry(x).or_insert(vec![false; height])[y as usize] = true;
            }
        }
    }
    let search_pattern = vec![Point{x:0,y:1}, Point{x:-1,y:1}, Point{x:1,y:1}];
    let mut num_sand = 0;
    let mut reached_bottom_after = None;
    let mut has_finished = false;
    println!("Map: \n{}", (0..height).map(|y| (min_x..=max_x).map(|x| if is_blocked.entry(x).or_insert(vec![false; height])[y as usize] {"#"} else {" "}).collect::<String>() + "\n").collect::<String>());
    while !has_finished {
        let mut cur_sand_pos = Point{x:sand_start.x, y:sand_start.y};
        while let Some(next) = search_pattern.iter().map(|diff| cur_sand_pos + *diff).find(|p| !is_blocked.entry(p.x).or_insert(vec![false; height])[p.y as usize]) {
            cur_sand_pos = next;
            if reached_bottom_after == None && (next.x < min_x || next.x > max_x || next.y > max_y) {
                reached_bottom_after = Some(num_sand);
            }
            if next.y > max_y {
                break;
            }
        }
        num_sand += 1;
        if cur_sand_pos == sand_start {
            has_finished = true;
        } else {
            assert!(!is_blocked.entry(cur_sand_pos.x).or_insert(vec![false; height])[cur_sand_pos.y as usize]);
            is_blocked.entry(cur_sand_pos.x).or_insert(vec![false; height])[cur_sand_pos.y as usize] = true;
        }
    }
    Ok(println!("Finished after {num_sand} sands. Reached edge after {} sands", reached_bottom_after.unwrap()))
}