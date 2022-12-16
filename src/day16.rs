use std::cmp::min;
use std::collections::HashMap;
use std::str::FromStr;
use crate::utils::ErrorMsg;
use regex::Regex;
use crate::utils;

pub fn run_sample() {
    ErrorMsg::print(run("input/day16_sample.txt"));
}

pub fn run_actual() {
    ErrorMsg::print(run("input/day16.txt"));
}

struct Tunnel {
    from: String,
    to: String
}

struct Node {
    flow_rate: u32,
    id: String,
    tunnels: Vec<String>
}
lazy_static! {
    static ref NODE_REGEX: Regex = Regex::new(r"Valve (..) has flow rate=(\d+); tunnel(?:s?) lead(?:s?) to valve(?:s?) (.*)").unwrap();
}
impl FromStr for Node {
    type Err = ErrorMsg;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let node_captures = NODE_REGEX.captures(s).ok_or(ErrorMsg{wrapped:format!("Failed to capture node regex in {s}")})?;
        let id = node_captures[1].to_string();
        let flow_rate = node_captures[2].parse::<u32>()?;
        let tunnels = node_captures[3].split(", ").map(|s|s.to_string()).collect();
        Ok(Node { flow_rate, id, tunnels })
    }
}

fn run(path: &str) -> Result<(), ErrorMsg> {
    let all_nodes = utils::read_lines(path)?.map(|l| l?.parse::<Node>())
        .collect::<Result<Vec<Node>, ErrorMsg>>()?;
    let mut edges: HashMap<String,HashMap<String,u32>> = all_nodes.iter().map(|n| (n.id.clone(), n.tunnels.iter().map(|s| (s.clone(), 1u32)).collect())).collect();
    for node in all_nodes.iter().filter(|n| n.flow_rate == 0) {
        for i in 0..node.tunnels.len() {
            let cost_from = edges.entry(node.tunnels[i].clone()).or_default().remove(node.id.as_str()).unwrap();
            for j in (i+1)..node.tunnels.len() {
                let cost_to = edges.entry(node.tunnels[j].clone()).or_default().remove(node.id.as_str()).unwrap();
                let e: &mut _ = edges.entry(node.tunnels[i].clone()).or_default().entry(node.tunnels[j].clone()).or_insert(10000);
                *e = min(*e, cost_from + cost_to);
                *edges.entry(node.tunnels[j].clone()).or_default().entry(node.tunnels[i].clone()).or_default() = *e;
            }
        }
    }
    Ok(println!("{}", all_nodes.iter()
        .map(|n| format!("N {} has {} and leads to {}", n.id, n.flow_rate, n.tunnels.iter().map(|s| s.to_string() + ", ").collect::<String>()) + ", ")
        .collect::<String>()))
}