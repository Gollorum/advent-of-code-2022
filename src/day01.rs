use std::cmp;
use std::error::Error;
use crate::utils;

pub fn run_sample() {
    run("input/day01_sample.txt");
}

pub fn run_actual() {
    run("input/day01.txt");
}

fn run(path: &str) {
    if let Ok(lines) = utils::read_lines(path) {
        let (acc, last) = lines.map(|elem| Ok(elem?.parse::<i32>()?))
            .fold((Stack::Nil, 0), |(accum, current), elem: Result<i32, Box<dyn Error>>|
                return if let Ok(num) = elem { (accum, current + num) }
                else { (Stack::Cons(current, Box::from(accum)), 0) });
        let all_resources = Stack::Cons(last, Box::from(acc));
        println!("Max: {}", all_resources.max());
        let (a, b, c) = all_resources.max3();
        println!("Max3: {}", a + b + c)
    } else { println!("Failed to read file") }
}

enum Stack {
    Nil,
    Cons(i32, Box<Stack>)
}

impl Stack {
    pub fn max(&self) -> i32 { match self {
        Stack::Nil => -1,
        Stack::Cons(head, tail) => cmp::max(*head, tail.max())
    } }
    pub fn max3(&self) -> (i32, i32, i32) { match self {
        Stack::Nil => (-1, -1, -1),
        Stack::Cons(head, tail) => match tail.max3() {
            (a, b, _) if a < *head => (*head, a, b),
            (a, b, _) if b < *head => (a, *head, b),
            (a, b, c) if c < *head => (a, b, *head),
            (a, b, c) => (a, b, c)
        }
    } }
}