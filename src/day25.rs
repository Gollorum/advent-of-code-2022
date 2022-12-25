use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign};
use crate::utils::ErrorMsg;
use std::str::FromStr;
use crate::utils;

pub fn run_sample() {
    ErrorMsg::print(run("input/day25_sample.txt"));
}

pub fn run_actual() {
    ErrorMsg::print(run("input/day25.txt"));
}

#[derive(Copy, Clone)]
struct Snafu {
    num: i64
}
impl FromStr for Snafu {
    type Err = ErrorMsg;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Snafu { num: s.chars().fold(Ok(0), |accum: Result<i64, ErrorMsg>, c| Ok(accum? * 5 + match c {
            '2' => Ok(2),
            '1' => Ok(1),
            '0' => Ok(0),
            '-' => Ok(-1),
            '=' => Ok(-2),
            _ => Err(ErrorMsg{wrapped:format!("Invalid char {c}")})
        }?))?})
    }
}
impl Display for Snafu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut ret: Vec<char> = Vec::new();
        let mut num = self.num;
        while num != 0 {
            let cur_digit = match num % 5 {
                2 | -3 => Ok(2),
                1 | -4 => Ok(1),
                0 => Ok(0),
                -1 | 4 => Ok(-1),
                -2 | 3 => Ok(-2),
                _ => Err(std::fmt::Error)
            }?;
            ret.insert(0, match cur_digit {
                2 => Ok('2'),
                1 => Ok('1'),
                0 => Ok('0'),
                -1 => Ok('-'),
                -2 => Ok('='),
                _ => Err(std::fmt::Error)
            }?);
            num = (num - cur_digit) / 5;
        }
        f.write_str(ret.iter().collect::<String>().as_str())
    }
}
impl Add<Snafu> for Snafu {
    type Output = Snafu;
    fn add(self, rhs: Snafu) -> Self::Output {
        Snafu {num: self.num + rhs.num}
    }
}
impl AddAssign<Snafu> for Snafu {
    fn add_assign(&mut self, rhs: Snafu) {
        *self = *self + rhs;
    }
}

fn run(path: &str) -> Result<(), ErrorMsg> {
    let mut sum: Snafu = Snafu{num:0};
    for l_r in utils::read_lines(path)? {
        let l = l_r?;
        let now = l.parse::<Snafu>()?;
        sum += now;
    }
    Ok(println!("The sum was {}, which is {sum}", sum.num))
}