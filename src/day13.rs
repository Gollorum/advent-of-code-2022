use std::cmp::Ordering;
use std::str::FromStr;
use crate::day13::PacketEntry::{Number, List};
use crate::utils;
use crate::utils::ErrorMsg;

pub fn run_sample() {
    ErrorMsg::print(run("input/day13_sample.txt"));
}

pub fn run_actual() {
    ErrorMsg::print(run("input/day13.txt"));
}

#[derive(Eq, PartialEq)]
enum PacketEntry {
    Number(u8),
    List(Vec<PacketEntry>)
}

impl FromStr for PacketEntry {
    type Err = ErrorMsg;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('[') {
            if !s.ends_with(']') { Err(ErrorMsg { wrapped: format!("Failed to parse packet entry {s}: Started with [, but didn't end with ]") }) }
            else {
                fn idx(s: &str, idx: usize) -> Result<char, ErrorMsg> {
                    s.chars().nth(idx).ok_or(ErrorMsg{wrapped: format!("Unexpected end of string found while trying to parse entry {s}")})
                }
                let mut entries: Vec<PacketEntry> = Vec::new();
                if s.len() == 2 { return Ok(List(entries)) }
                let mut from = 1usize;
                let mut to = 1usize;
                while from < s.len() - 1 {
                    let mut list_depth = 0;
                    while list_depth > 0 || (idx(s, to)? != ',' && idx(s, to)? != ']') {
                        match idx(s, to)? {
                            '[' => { list_depth += 1; }
                            ']' => {
                                if list_depth == 0 { return Err(ErrorMsg::new("Tried to escape root list"))}
                                list_depth -= 1; }
                            _ => {}
                        }
                        to += 1;
                    }
                    entries.push(s[from..to].parse::<PacketEntry>()?);
                    to = to + 1;
                    from = to;
                }
                Ok(List(entries))
            }
        } else {
            Ok(Number(s.parse::<u8>()?))
        }
    }
}

fn cmp_lists(l: &Vec<PacketEntry>, r: &Vec<PacketEntry>) -> Ordering {
    let mut r_it = r.iter();
    for l_e in l {
        if let Some(r_e) = r_it.next() {
            match l_e.cmp(r_e) {
                Ordering::Equal => {}
                other => return other
            }
        } else {
            return Ordering::Greater
        }
    }
    match r_it.next() {
        None => Ordering::Equal,
        Some(..) => Ordering::Less
    }
}
impl Ord for PacketEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Number(l), Number(r)) => l.cmp(r),
            (Number(l), r) => List(vec![Number(*l)]).cmp(r),
            (l, Number(r)) => l.cmp(&List(vec![Number(*r)])),
            (List(l), List(r)) => cmp_lists(l, r)
        }
    }
}

impl PartialOrd for PacketEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn run(path: &str) -> Result<(), ErrorMsg> {
    let mut lines = utils::read_lines(path)?;
    let mut idx = 0u32;
    let mut right_sum = 0u32;
    let mut all_packets: Vec<PacketEntry> = Vec::new();
    while let Some(l1_r) = lines.next() {
        idx += 1;
        let l1 = l1_r?;
        let l2 = lines.next().ok_or(ErrorMsg::new("Expected second packet, didn't find one"))??;
        let p1 = l1.parse::<PacketEntry>()?;
        let p2 = l2.parse::<PacketEntry>()?;
        if p1 < p2 {
            right_sum += idx;
        }
        all_packets.push(p1);
        all_packets.push(p2);
        lines.next();
    }
    let first_div = List(vec![List(vec![Number(2)])]);
    let second_div = List(vec![List(vec![Number(6)])]);
    all_packets.push(first_div);
    all_packets.push(second_div);
    all_packets.sort();
    let first_div_idx = all_packets.iter().position(|e| *e == List(vec![List(vec![Number(2)])])).ok_or(ErrorMsg::new("Did not find first divider"))?;
    let second_div_idx = all_packets.iter().position(|e| *e == List(vec![List(vec![Number(6)])])).ok_or(ErrorMsg::new("Did not find second divider"))?;
    Ok(println!("Right sum was {right_sum}, div index mul was {}", (first_div_idx + 1) * (second_div_idx + 1)))
}