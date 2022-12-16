use std::cmp::{max, min, Ordering};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use regex::Regex;
use crate::utils;
use crate::utils::ErrorMsg;

pub fn run_sample() {
    ErrorMsg::print(run("input/day15_sample.txt", 10, 20));
}

pub fn run_actual() {
    ErrorMsg::print(run("input/day15.txt", 2000000, 4000000));
}

lazy_static! {
    static ref POS_REGEX: Regex = Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();
    static ref SENSOR_REGEX: Regex = Regex::new(r"Sensor at (.+): closest beacon is at (.+)").unwrap();
}
#[derive(Clone, Copy, PartialEq, Eq, Ord)]
struct Pos { x: i32, y: i32 }
impl FromStr for Pos {
    type Err = ErrorMsg;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = POS_REGEX.captures(s).ok_or(ErrorMsg{wrapped:format!("Failed to capture pos regex in {s}")})?;
        Ok(Pos {
            x: captures[1].parse::<i32>()?,
            y: captures[2].parse::<i32>()?
        })
    }
}
impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("x={}, y={}", self.x, self.y).as_str())
    }
}
impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(if self.x == other.x {self.y.cmp(&other.y)} else {self.x.cmp(&other.x)})
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Sensor {
    pos: Pos,
    beacon: Pos
}
impl FromStr for Sensor {
    type Err = ErrorMsg;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = SENSOR_REGEX.captures(s).ok_or(ErrorMsg{wrapped:format!("Failed to capture sensor regex in {s}")})?;
        Ok(Sensor {
            pos: captures[1].parse::<Pos>()?,
            beacon: captures[2].parse::<Pos>()?
        })
    }
}
impl Sensor {
    fn range(&self) -> i32 {
        (self.beacon.x - self.pos.x).abs() + (self.beacon.y - self.pos.y).abs()
    }
    fn range_at(&self, row: i32) -> Option<i32> {
        let ret = self.range() - (self.pos.y - row).abs();
        if ret > 0 { Some(ret) } else { None }
    }
}

struct Ranges {
    wrapped: Vec<(i32, i32)>
}
impl Ranges {
    fn combine_with(&self, center: i32, radius: i32) -> Ranges {
        let mut new_range = (center - radius.abs(), center + radius.abs());
        let mut ret = Vec::new();
        let mut was_inserted = false;
        for old_range in self.wrapped.as_slice() {
            if !was_inserted && old_range.0 > new_range.1 {
                ret.push(new_range);
                was_inserted = true;
            }
            if old_range.0 > new_range.1 || old_range.1 < new_range.0 {
                ret.push(*old_range);
            } else {
                new_range.0 = min(new_range.0, old_range.0);
                new_range.1 = max(new_range.1, old_range.1);
            }
        }
        if !was_inserted {
            ret.push(new_range);
        }
        Ranges {wrapped: ret}
    }
    fn sub(&self, slice: &(i32, i32)) -> Ranges {
        let mut ret = Vec::new();
        for old_range in self.wrapped.as_slice() {
            if old_range.0 > slice.1 || old_range.1 < slice.0 { ret.push(*old_range); }
            else {
                if old_range.0 < slice.0 {
                    ret.push((old_range.0, min(old_range.1, slice.0 - 1)));
                }
                if old_range.1 > slice.1 {
                    ret.push((max(old_range.0, slice.1 + 1), old_range.1));
                }
            }
        }
        Ranges {wrapped: ret}
    }
    fn contains(&self, point: i32) -> bool {
        self.wrapped.iter().any(|r| r.0 <= point && r.1 >= point)
    }
    fn num(&self) -> usize {
        self.wrapped.iter().map(|s| (s.1 - s.0 + 1) as usize).sum()
    }
    fn free_in(&self, min: i32, max: i32) -> Ranges {
        let mut ret = Ranges{wrapped: vec![(min, max)]};
        for slice in self.wrapped.iter() {
            ret = ret.sub(slice);
        }
        ret
    }
    fn all(&self) -> impl Iterator<Item=i32> + '_ {
        self.wrapped.iter().flat_map(|sub| sub.0..=sub.1)
    }
}

fn run(path: &str, row: i32, max: i32) -> Result<(), ErrorMsg> {
    let sensors = utils::read_lines(path)?.map(|l| l?.parse::<Sensor>()).collect::<Result<Vec<Sensor>, ErrorMsg>>()?;
    let mut beacons = sensors.iter().map(|s| s.beacon).collect::<Vec<Pos>>();
    beacons.sort();
    beacons.dedup();
    let ranges_on = |y| sensors.iter()
        .filter_map(|sensor| sensor.range_at(y).map(|r| (sensor.pos.x, r)))
        .fold(Ranges{wrapped:Vec::new()}, |ranges, sensor| ranges.combine_with(sensor.0, sensor.1));
    let ranges_on_dest_row = ranges_on(row);
    let blocked_on_row = ranges_on_dest_row.num() - beacons.iter().filter(|b| b.y == row && ranges_on_dest_row.contains(b.x)).count();
    let beancon_pos = (0..=max).flat_map(|y| ranges_on(y).free_in(0, max).all().map(|x| (x, y)).collect::<Vec<(i32, i32)>>());
    Ok(println!("Blocked in row: {}, all possible beacon locations: {}", blocked_on_row, beancon_pos.map(|p| format!("{} ", p.0 as u64 * 4000000 + p.1 as u64)).collect::<String>()))
}