use crate::utils;
use crate::utils::ErrorMsg;
use std::num::ParseIntError;

pub fn run_sample(part_2: bool) {
    ErrorMsg::print(run("input/day11_sample.txt", part_2));
}

pub fn run_actual(part_2: bool) {
    ErrorMsg::print(run("input/day11.txt", part_2));
}

enum Operation {
    Old,
    Lit(u64),
    Add(Box<Operation>, Box<Operation>),
    Mul(Box<Operation>, Box<Operation>)
}
impl Operation {
    fn eval(&self, old_val: u64, md: u64) -> u64 { match self {
        Operation::Old => old_val,
        Operation::Lit(val) => *val,
        Operation::Add(l, r) => (l.eval(old_val, md) + r.eval(old_val, md)) % md,
        Operation::Mul(l, r) => (l.eval(old_val, md) * r.eval(old_val, md)) % md
    }}
    fn parse(s: &str) -> Result<Operation, ErrorMsg> {
        return if s == "old" { Ok(Operation::Old) } 
        else if let Some(idx) = s.find(" + ") {
            Ok(Operation::Add(
                Box::new(Operation::parse(&s[..idx])?),
                Box::new(Operation::parse(&s[(idx + 3)..])?)
            ))
        } else if let Some(idx) = s.find(" * ") {
            Ok(Operation::Mul(
                Box::new(Operation::parse(&s[..idx])?),
                Box::new(Operation::parse(&s[(idx + 3)..])?)
            ))
        } else {
            Ok(Operation::Lit(s.parse::<u64>()?))
        }
    }
}

struct Monkey {
    items: Vec<u64>,
    op: Operation,
    div_check: u64,
    if_true: usize,
    if_false: usize
}

fn run(path: &str, part_2: bool) -> Result<(), ErrorMsg> {
    const STARTIN_ITEMS_LINE_HEAD: &str = "  Starting items: ";
    const OP_LINE_HEAD: &str = "  Operation: new = ";
    const TEST_LINE_HEAD: &str = "  Test: divisible by ";
    const TRUE_LINE_HEAD: &str = "    If true: throw to monkey ";
    const FALSE_LINE_HEAD: &str = "    If false: throw to monkey ";
    let mut lines = utils::read_lines(path)?.enumerate();
    let mut monkeys: Vec<Monkey> = Vec::new();
    while let Some((i, Ok(line0))) = lines.next() {
        if line0 != format!("Monkey {}:", i / 7) {
            return Err(ErrorMsg{wrapped: format!("Unexpected line encountered: {line0}") })
        }
        let line1 = lines.next().ok_or(ErrorMsg::new("Starting items not found"))?.1?;
        if !line1.starts_with(STARTIN_ITEMS_LINE_HEAD) {
            return Err(ErrorMsg{wrapped: format!("Line did not start with '{}', was '{}'", STARTIN_ITEMS_LINE_HEAD, line1)})
        }
        let starting_items: Vec<u64> = line1[STARTIN_ITEMS_LINE_HEAD.len()..].split(", ").map(|n| n.parse::<u64>()).collect::<Result<Vec<u64>, ParseIntError>>()?;
        let line2 = lines.next().ok_or(ErrorMsg::new("Operation not found"))?.1?;
        if !line2.starts_with(OP_LINE_HEAD) {
            return Err(ErrorMsg{wrapped: format!("Line did not start with '{}', was '{}'", OP_LINE_HEAD, line2)})
        }
        let op = Operation::parse(&line2[OP_LINE_HEAD.len()..])?;
        let line3 = lines.next().ok_or(ErrorMsg::new("Test not found"))?.1?;
        if !line3.starts_with(TEST_LINE_HEAD) {
            return Err(ErrorMsg{wrapped: format!("Line did not start with '{}', was '{}'", TEST_LINE_HEAD, line3)})
        }
        let div_check = line3[TEST_LINE_HEAD.len()..].parse::<u64>()?;
        let line4 = lines.next().ok_or(ErrorMsg::new("True not found"))?.1?;
        if !line4.starts_with(TRUE_LINE_HEAD) {
            return Err(ErrorMsg{wrapped: format!("Line did not start with '{}', was '{}'", TRUE_LINE_HEAD, line4)})
        }
        let if_true = line4[TRUE_LINE_HEAD.len()..].parse::<usize>()?;
        let line5 = lines.next().ok_or(ErrorMsg::new("False not found"))?.1?;
        if !line5.starts_with(FALSE_LINE_HEAD) {
            return Err(ErrorMsg{wrapped: format!("Line did not start with '{}', was '{}'", FALSE_LINE_HEAD, line5)})
        }
        let if_false = line5[FALSE_LINE_HEAD.len()..].parse::<usize>()?;
        monkeys.push(Monkey {
            items: starting_items,
            op,
            div_check,
            if_true,
            if_false
        });
        lines.next();
    }
    let mut inspections = vec![0usize; monkeys.len()];
    let div_check_p: u64 = monkeys.iter().map(|m| m.div_check).product();
    for _ in 0..(if part_2 {10000} else {20}) {
        for i in 0..monkeys.len() {
            while monkeys[i].items.len() > 0 {
                let old_val: u64 = monkeys[i].items.remove(0);
                let item_val: u64 = monkeys[i].op.eval(old_val, div_check_p) / if part_2 {1} else {3};
                let target = if (item_val % monkeys[i].div_check) == 0 {
                    monkeys[i].if_true
                } else {monkeys[i].if_false};
                monkeys[target as usize].items.push(item_val);
                inspections[i] += 1;
            }
        }
    }
    inspections.sort();
    Ok(println!("{}", inspections[inspections.len() - 1] * inspections[inspections.len() - 2]))
}