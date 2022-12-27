use std::cmp::{min};
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

fn remove_node(edges: &mut HashMap<String,HashMap<String,u32>>, node: &Node) -> () {
    for to in edges.values_mut() {
        to.remove(node.id.as_str());
    }
}

fn eval_subset(
    start: u16,
    subset: usize,
    nodes: &Vec<(&Node, Vec<u32>)>,
    max_time: u32
) -> u32 {
    if subset == 0 { return 0 }
    (0..(nodes.len()-1))
        .map(|i| i+1)
        .filter(|i| ((1 << i) & subset != 0))
        .filter_map(|i| {
            let cost = nodes[start as usize].1[
                if start == 0 {i-1}
                else {(if i > start as usize {i-1} else {i}) - 1
            }] + 1;
            if cost > max_time {
                None
            } else {
                let remaining_time = max_time - cost;
                let next_node = nodes[i].0;
                let released_by_this = remaining_time * next_node.flow_rate;
                let released_later = eval_subset(i as u16, subset & !(1<<i), nodes, remaining_time);
                // if i == 3 {
                //     println!("Expanding {i} aka {} at time {} yields {released_by_this} + {released_later}", next_node.id, 30 - remaining_time);
                // }
                Some(released_by_this + released_later)
            }
        }).max().unwrap_or(0)
}


fn to_str(edges: &HashMap<String,HashMap<String,u32>>) -> String {
    edges.iter().map(|e| format!("\n{} -> {}", e.0, e.1.iter().map(|ee| ee.0.clone()).collect::<String>())).collect::<String>()
}

fn run(path: &str) -> Result<(), ErrorMsg> {
    let all_nodes = utils::read_lines(path)?.map(|l| l?.parse::<Node>())
        .collect::<Result<Vec<Node>, ErrorMsg>>()?;
    let nodes_map: HashMap<String, &Node> = all_nodes.iter().map(|n| (n.id.clone(), n)).collect();
    let mut edges: HashMap<String,HashMap<String,u32>> = all_nodes.iter().map(|n| (n.id.clone(), n.tunnels.iter().map(|s| (s.clone(), 1u32)).collect())).collect();
    println!("All edges: {}", to_str(&edges));
    for _ in 0..all_nodes.len() {
        for node in all_nodes.iter() {
            for (i, from) in edges.get(node.id.as_str()).unwrap().iter().map(|t| (t.0.clone(), *t.1)).enumerate().collect::<Vec<(usize, (String, u32))>>() {
                // if edges.get(node.tunnels[i].as_str()) == None { continue; }
                let cost_from_i = edges.entry(from.0.clone()).or_default().get(node.id.as_str()).map(|v| *v);
                let cost_to_i = edges.get(node.id.as_str()).unwrap().get(from.0.as_str()).map(|v| *v);
                for to in edges.get(node.id.as_str()).unwrap().iter().skip(i+1).map(|t| (t.0.clone(), *t.1)).collect::<Vec<(String, u32)>>() {
                    // if edges.get(node.tunnels[j].as_str()) == None { continue; }
                    let cost_from_j = edges.entry(to.0.clone()).or_default().get(node.id.as_str()).map(|v| *v);
                    let cost_to_j = edges.get(node.id.as_str()).unwrap().get(to.0.as_str()).map(|v| *v);
                    if cost_from_i != None && cost_to_j != None {
                        let e: &mut _ = edges.entry(from.0.clone()).or_default().entry(to.0.clone()).or_insert(10000);
                        *e = min(*e, cost_from_i.unwrap() + cost_to_j.unwrap());
                    } else {println!("There was no way from {} to {}", from.0, to.0)}
                    if cost_from_j != None && cost_to_i != None {
                        let e: &mut _ = edges.entry(to.0.clone()).or_default().entry(from.0.clone()).or_insert(10000);
                        *e = min(*e, cost_from_j.unwrap() + cost_to_i.unwrap());
                    } else {println!("There was no way from {} to {}", to.0, from.0)}
                }
            }
        }
    }
    for node in all_nodes.iter().filter(|n| n.flow_rate == 0) {
        remove_node(&mut edges, node);
        if node.id != "AA" { edges.remove(node.id.as_str()); }
    }
    println!("All edges: {}", to_str(&edges));
    let mut sorted_nodes = edges.iter().map(|(name, _)| name).collect::<Vec<_>>();
    sorted_nodes.sort();
    let sorted_edges: Vec<(&Node,Vec<u32>)> = sorted_nodes.iter().enumerate().map(|(i, &name)| (
        nodes_map[name],
        sorted_nodes.iter().enumerate().skip(1).filter(|&(ii,_)| i != ii).map(|(_,&n)| edges[name][n]).collect()
    )).collect();
    let max_time = 26;
    println!("edges: {}", sorted_edges.len());
    let max_subset = (1<<(sorted_edges.len()))-1;
    let total_pressure = (0..max_subset/2).map(|subset_l|
        eval_subset(0, subset_l, &sorted_edges, max_time)
         + eval_subset(0, (!subset_l) & max_subset, &sorted_edges, max_time)
    ).max().unwrap();
    Ok(println!("Total pressure: {}", total_pressure))
}