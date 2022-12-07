use std::collections::HashMap;
use std::ffi::c_int;
use std::io::{BufReader, Lines};
use std::iter::Map;
use crate::day07::Element::{Directory, File};
use crate::utils;
use crate::utils::ErrorMsg;
use by_address::ByAddress;

pub fn run_sample() {
    ErrorMsg::print(run("input/day07_sample.txt"));
}

pub fn run_actual() {
    ErrorMsg::print(run("input/day07.txt"));
}

struct DirectoryData {
    name: String
}
impl DirectoryData {
    fn size(&self, elements_of: &HashMap<ByAddress<&DirectoryData>, Vec<&Element>>) -> u32 {
        elements_of.get(&ByAddress(self)).map_or(0, |elems| elems.iter().map(|e| e.size(elements_of)).sum())
    }
}

enum Element {
    File { name: String, size: u32 },
    Directory(DirectoryData)
}

impl Element{
    fn parse(line: &String) -> Result<Element, ErrorMsg> {
        if line.starts_with("dir ") { Ok(Directory(DirectoryData { name: line[4..].to_string() })) }
        else {
            let (size_str, name) = line.split_at(line.find(" ").ok_or(ErrorMsg{wrapped: "Failed to parse file: no whitespace found.".to_string()})?);
            Ok(File {
                name: name[1..].to_string(),
                size: size_str.parse::<u32>()?
            })
        }
    }
    fn size(&self, elements_of: &HashMap<ByAddress<&DirectoryData>, Vec<&Element>>) -> u32 {
        match self {
            File { size: x, .. } => *x,
            Directory(d) => d.size(elements_of)
        }
    }
}

fn run(path: &str) -> Result<(), ErrorMsg> {
    let mut lines: Vec<String> = utils::read_lines(path)?.collect::<Result<Vec<String>,std::io::Error>>()?;
    let mut root: DirectoryData = DirectoryData { name: "/".to_string() };
    let mut current_dir: Vec<&DirectoryData> = Vec::from([&root]);

    let mut elements_of: HashMap<ByAddress<&DirectoryData>, Vec<&Element>> = HashMap::new();

    let mut all_elements: HashMap<usize, Element> = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        if !line.starts_with("$ ") {
            all_elements.entry(i).or_insert(Element::parse(line)?);
        }
    }

    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("$ ") {
            match line[2..].split_at(2) {
                ("ls", "") => {},
                ("cd", x) => match x {
                    " .." => { current_dir.pop().ok_or(ErrorMsg::new("Tried to get parent of root"))?; },
                    " /" => { current_dir = Vec::from([&root]) },
                    _ => {
                        let dir_name = &x[1..];
                        let mut success = false;
                        for e in elements_of.get(
                            &ByAddress(current_dir.last().ok_or(ErrorMsg::new("Was not in any dir"))?)
                        ).ok_or(ErrorMsg::new("No elements found"))? {
                            match e {
                                Directory(d) => {
                                    if d.name.as_str() == dir_name {
                                        current_dir.push(d);
                                        success = true;
                                        break;
                                    }
                                },
                                _ => { }
                            }
                        }
                        if !success {
                            return Err(ErrorMsg { wrapped: format!("Failed to step in dir {}", dir_name) });
                        }
                    }
                },
                _ => return Err(ErrorMsg { wrapped: format!("Failed to parse command {}", line) })
            }
        } else {
            elements_of.entry(ByAddress(current_dir.last().ok_or(ErrorMsg::new("Was not in any dir"))?))
                .or_insert(Vec::new()).push(all_elements.get(&i).expect(""));
        }
    }

    let total_size = 70000000;
    let used_size = root.size(&elements_of);
    let needed_size = 30000000;
    let min_to_delete = used_size + needed_size - total_size;

    Ok(println!("Part 1: {}, Part 2: {}",
        elements_of.keys()
            .map(|dir| dir.size(&elements_of))
            .filter(|s| *s <= 100000)
            .sum::<u32>(),
        elements_of.keys()
            .map(|dir| dir.size(&elements_of))
                .filter(|s| *s >= min_to_delete)
                .min().ok_or(ErrorMsg::new("No folder big enough"))?
    ))
}