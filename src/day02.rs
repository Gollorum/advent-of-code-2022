use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io;
use std::iter::Map;
use crate::day02::Outcome::{Win, Draw, Loss};
use crate::day02::Symbol::{Rock, Paper, Scissors};
use crate::utils;
use regex::Regex;

pub fn run_sample(part2: bool) {
    run("input/day02_sample.txt", part2);
}

pub fn run_actual(part2: bool) {
    run("input/day02.txt", part2);
}

enum Outcome {
    Win, Draw, Loss
}
impl Outcome {
    fn score(&self) -> i32 { match self {
        Win => 6,
        Draw => 3,
        Loss => 0
    }}
}
impl Display for Outcome {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Win => "Win",
            Draw => "Draw",
            Loss => "Loss"
        })
    }
}
#[derive(Clone, Copy, Eq, Hash)]
enum Symbol {
    Rock, Paper, Scissors
}
impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Rock => "Rock",
            Paper => "Paper",
            Scissors => "Scissors"
        })
    }
}
impl Symbol {
    fn score(&self) -> i32 { match self {
        Rock => 1,
        Paper => 2,
        Scissors => 3
    }}
    fn from(score: i32) -> Option<Symbol> { match (score + 3) % 3 {
        0 => Some(Scissors),
        1 => Some(Rock),
        2 => Some(Paper),
        _ => None
    }}
}

fn play(a: &Symbol, b: &Symbol) -> Result<Outcome, (Symbol, Symbol)> {
    match (a.score() - b.score() + 3) % 3 {
        0 => Ok(Draw),
        1 => Ok(Loss),
        2 => Ok(Win),
        _ => Err((*a, *b))
    }
}

fn inverse_play(a: &Symbol, outcome: &Outcome) -> Option<Symbol> {
    Symbol::from((a.score() - match outcome {
        Draw => 0,
        Loss => 1,
        Win => 2
    }))
}

fn score(a: &Symbol, b: &Symbol) -> Result<i32, (Symbol, Symbol)> {
    Ok(b.score() + play(a, b)?.score())
}

fn translate_first_column(c: &char) -> Option<Symbol> { match c {
    'A' => Some(Rock),
    'B' => Some(Paper),
    'C' => Some(Scissors),
    _ => None
}}

fn translate_second_column_p1(c: &char) -> Option<Symbol> { match c {
    'X' => Some(Rock),
    'Y' => Some(Paper),
    'Z' => Some(Scissors),
    _ => None
}}

fn translate_second_column_p2(c: &char) -> Option<Outcome> { match c {
    'X' => Some(Loss),
    'Y' => Some(Draw),
    'Z' => Some(Win),
    _ => None
}}

fn run(path: &str, part2: bool) -> Option<()> {
    let lines = utils::read_lines(path);
    let games: Vec<(Symbol, Symbol)> = lines.ok()?.map(|str_r| {
        let re = Regex::new(r"(\w) (\w)").unwrap();
        let str = str_r.ok()?;
        let captures = re.captures(str.as_str())?;
        let theirs = translate_first_column(&captures[1].chars().nth(0)?)?;
        let second_char = &captures[2].chars().nth(0)?;
        let ours = if part2 {
            inverse_play(&theirs, &translate_second_column_p2(second_char)?)?
        } else {
            translate_second_column_p1(second_char)?
        };
        Some((theirs, ours))
    }).collect::<Option<Vec<(Symbol, Symbol)>>>()?;
    let scores: Vec<i32> = games.iter().map(|(theirs, ours)| score(theirs, ours).ok()).collect::<Option<Vec<i32>>>()?;
    let res: i32 = scores.iter().sum();
    Some(println!("{}", res))
}