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
    for i in 0..node.tunnels.len() {
        if edges.get(node.tunnels[i].as_str()) == None { continue; }
        let cost_from_i = edges.entry(node.tunnels[i].clone()).or_default().remove(node.id.as_str());
        let cost_to_i = edges.get(node.id.as_str()).unwrap().get(node.tunnels[i].as_str()).map(|v| *v);
        for j in (i+1)..node.tunnels.len() {
            if edges.get(node.tunnels[j].as_str()) == None { continue; }
            let cost_from_j = edges.entry(node.tunnels[j].clone()).or_default().get(node.id.as_str()).map(|v| *v);
            let cost_to_j = edges.get(node.id.as_str()).unwrap().get(node.tunnels[j].as_str()).map(|v| *v);
            if cost_from_i != None && cost_to_j != None {
                let e: &mut _ = edges.entry(node.tunnels[i].clone()).or_default().entry(node.tunnels[j].clone()).or_insert(10000);
                *e = min(*e, cost_from_i.unwrap() + cost_to_j.unwrap());
            } else {println!("There was no way from {} to {}", node.tunnels[i], node.tunnels[j])}
            if cost_from_j != None && cost_to_i != None {
                let e: &mut _ = edges.entry(node.tunnels[j].clone()).or_default().entry(node.tunnels[i].clone()).or_insert(10000);
                *e = min(*e, cost_from_j.unwrap() + cost_to_i.unwrap());
            } else {println!("There was no way from {} to {}", node.tunnels[j], node.tunnels[i])}
        }
    }
    edges.remove(node.id.as_str());
}

fn max_pressure_for(edges: &HashMap<String,HashMap<String,u32>>, nodes: &HashMap<String, &Node>, start: &str, start_edges: &HashMap<String, u32>, current_flow: u32, minutes_left: u32) -> u32 {
    let mut new_edges = edges.clone();
    let node = nodes.get(start).unwrap();
    remove_node(&mut new_edges, node);
    // println!("All edges: {}", to_str(edges));
    println!("Minute {}: Expand {}, edges: {}", 30 - minutes_left+1, start, start_edges.iter().map(|e| format!("{}: {} | ", e.0, e.1)).collect::<String>());
    start_edges.iter().filter_map(|e| if *e.1 > minutes_left || edges.get(e.0) == None { None } else {
        let cost = if node.flow_rate == 0 { *e.1 } else { e.1 + 1 };
        Some(max_pressure_for(&new_edges, nodes, e.0.as_str(), edges.get(e.0).unwrap(), current_flow + node.flow_rate, minutes_left - cost) + cost * current_flow)
    }).max().unwrap_or(minutes_left * current_flow)
}

fn to_str(edges: &HashMap<String,HashMap<String,u32>>) -> String {
    edges.iter().map(|e| format!("\n{} -> {}", e.0, e.1.iter().map(|ee| ee.0.clone()).collect::<String>())).collect::<String>()
}

fn run(path: &str) -> Result<(), ErrorMsg> {
    let all_nodes = utils::read_lines(path)?.map(|l| l?.parse::<Node>())
        .collect::<Result<Vec<Node>, ErrorMsg>>()?;
    let nodes_map = all_nodes.iter().map(|n| (n.id.clone(), n)).collect();
    let mut edges: HashMap<String,HashMap<String,u32>> = all_nodes.iter().map(|n| (n.id.clone(), n.tunnels.iter().map(|s| (s.clone(), 1u32)).collect())).collect();
    let start_edges = edges.get("AA").unwrap();
    // for node in all_nodes.iter().filter(|n| n.flow_rate == 0) {
    //     remove_node(&mut edges, node);
    // }
    let total_pressure = max_pressure_for(&edges, &nodes_map, "AA", start_edges, 0, 30);
    Ok(println!("Total pressure: {}", total_pressure))
}