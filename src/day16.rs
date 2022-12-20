use std::cmp::{max, min};
use std::collections::HashMap;
use std::str::FromStr;
use crate::utils::ErrorMsg;
use regex::Regex;
use crate::utils;
use itertools::Itertools;

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
    // for i in 0..node.tunnels.len() {
    //     if edges.get(node.tunnels[i].as_str()) == None { continue; }
    //     let cost_from_i = edges.entry(node.tunnels[i].clone()).or_default().remove(node.id.as_str());
    //     let cost_to_i = edges.get(node.id.as_str()).unwrap().get(node.tunnels[i].as_str()).map(|v| *v);
    //     for j in (i+1)..node.tunnels.len() {
    //         if edges.get(node.tunnels[j].as_str()) == None { continue; }
    //         let cost_from_j = edges.entry(node.tunnels[j].clone()).or_default().get(node.id.as_str()).map(|v| *v);
    //         let cost_to_j = edges.get(node.id.as_str()).unwrap().get(node.tunnels[j].as_str()).map(|v| *v);
    //         if cost_from_i != None && cost_to_j != None {
    //             let e: &mut _ = edges.entry(node.tunnels[i].clone()).or_default().entry(node.tunnels[j].clone()).or_insert(10000);
    //             *e = min(*e, cost_from_i.unwrap() + cost_to_j.unwrap());
    //         } else {println!("There was no way from {} to {}", node.tunnels[i], node.tunnels[j])}
    //         if cost_from_j != None && cost_to_i != None {
    //             let e: &mut _ = edges.entry(node.tunnels[j].clone()).or_default().entry(node.tunnels[i].clone()).or_insert(10000);
    //             *e = min(*e, cost_from_j.unwrap() + cost_to_i.unwrap());
    //         } else {println!("There was no way from {} to {}", node.tunnels[j], node.tunnels[i])}
    //     }
    // }
    // edges.remove(node.id.as_str());
    for to in edges.values_mut() {
        to.remove(node.id.as_str());
    }
}

fn max_pressure_for(
    edges: &HashMap<String,HashMap<String,u32>>,
    nodes: &HashMap<String, &Node>,
    start: &str,
    current_flow: u32,
    minutes_left: u32
) -> u32 {
    // println!("All edges: {}", to_str(edges));
    // println!("Minute {}: Expand {}, edges: {}", 30 - minutes_left+1, start, start_edges.iter().map(|e| format!("{}: {} | ", e.0, e.1)).collect::<String>());
    edges.get(start).unwrap().iter().filter_map(|e| if  e.1 + 1 > minutes_left || edges.get(e.0) == None { None } else {
        let mut new_edges = edges.clone();
        let n = nodes.get(e.0).unwrap();
        remove_node(&mut new_edges, n);
        let cost = e.1 + 1;
        Some(max_pressure_for(&new_edges, nodes, e.0.as_str(), current_flow + n.flow_rate, minutes_left - cost) + cost * current_flow)
    }).max().unwrap_or(minutes_left * current_flow)
}

fn eval_permutation(
    nodes: &Vec<(&Node, Vec<u32>)>,
    permutation_cache: &mut Vec<Option<(u32, u32, u32)>>,
    permutation: u64,
    max_time: u32
) -> (u32, u32, u32) {
    if let Some(res) = permutation_cache[permutation as usize] {
        return res;
    }
    let prev_permutation = permutation >> 4;
    let (prev_time, prev_press, prev_final_flow) = eval_permutation(nodes, permutation_cache, prev_permutation, max_time);
    let last_index = permutation & 0b1111;
    let node = nodes[last_index as usize].0;
    let prev_last_index = (prev_permutation & 0b1111) as usize;
    let cost_to_last = nodes[prev_last_index].1[last_index as usize];
    let time = prev_time + cost_to_last;
    let pressure = prev_press + cost_to_last * prev_final_flow;
    let final_flow = prev_final_flow + node.flow_rate;
    let res = if time > max_time {
        (max_time, prev_press + prev_final_flow * (max_time-prev_time), prev_final_flow)
    } else {
        (time, pressure, final_flow)
    };
    permutation_cache[permutation as usize] = Some(res);
    res
}

fn max_pressure_for_two(
    edges: &HashMap<String,HashMap<String,u32>>,
    nodes: &HashMap<String, &Node>,
    start: (&str, &str),
    current_flow: (u32, u32),
    minutes_left: (u32, u32)
) -> u32 {
    vec![true, false].iter().filter_map(|&use_left| {
        let chosen_start = if use_left {start.0} else {start.1};
        // println!("All edges: {}", to_str(edges));
        // println!("Minute {}: Expand {}, edges: {}", 30 - if use_left {minutes_left.0} else {minutes_left.1}+1, chosen_start, edges.get(chosen_start).unwrap().iter().map(|e| format!("{}: {} | ", e.0, e.1)).collect::<String>());
        edges.get(chosen_start).unwrap().iter().filter_map(|e| if e.1 + 1 > if use_left {minutes_left.0} else {minutes_left.1} || edges.get(e.0.as_str()) == None { None } else {
            let mut new_edges = edges.clone();
            let chosen_current_flow = if use_left {current_flow.0} else {current_flow.1};
            let n = nodes.get(e.0.as_str()).unwrap();
            remove_node(&mut new_edges, n);
            let cost = e.1 + 1;
            let new_start = if use_left {(e.0.as_str(),start.1)} else {(start.0,e.0.as_str())};
            let new_flow = if use_left {(chosen_current_flow + n.flow_rate,current_flow.1)} else {(current_flow.0,chosen_current_flow + n.flow_rate)};
            let new_minutes_left = if use_left {(minutes_left.0-cost,minutes_left.1)} else {(minutes_left.0,minutes_left.1-cost)};
            let res = max_pressure_for_two(&new_edges, nodes, new_start, new_flow, new_minutes_left);
            let to_add = cost * chosen_current_flow;
            Some(res+to_add)
    }).max()}).max().unwrap_or(minutes_left.0*current_flow.0 + minutes_left.1*current_flow.1)
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
    // TODO DS: Total pressure of subgroups in the same order can be cached
    // TODO DS: Cache max pressure for subgroup / subset?
    println!("All edges: {}", to_str(&edges));
    // let total_pressure = max_pressure_for(&edges, &nodes_map, "AA", 0, 30);
    let mut sorted_nodes = edges.iter().map(|(name, _)| name).collect::<Vec<_>>();
    sorted_nodes.sort();
    let sorted_edges: Vec<(&Node,Vec<u32>)> = sorted_nodes.iter().enumerate().map(|(i, &name)| (
        nodes_map[name],
        sorted_nodes.iter().enumerate().filter(|&(ii,_)| i != ii).map(|(_,&n)| edges[name][n]).collect()
    )).collect();
    let mut cache = vec![None; 1 << edges.len()];
    cache[0] = Some((0,0,0));
    let total_pressure = (1..sorted_edges.len()).permutations(sorted_edges.len()-1)
        .map(|p_list| eval_permutation(
            &sorted_edges,
            &mut cache,
            p_list.iter().fold(0, |accum, now| accum << 4 | *now as u64),
            26
        ).1).max().unwrap();
    // let edges_with_indices = edges.iter().enumerate().collect::<Vec<_>>();
    // let a_i = edges_with_indices.iter().find(|e|e.1.0 == "AA").unwrap().0;
    // let total_pressure =
    //     // {
    //     (0..(1 << edges.len())).filter(|i|(i & (1<<a_i)) == 0).map(|i| {
    //         max_pressure_for(&edges_with_indices.iter().filter(|(ii,e)| e.0 == "AA" || (i & (1<<ii)) == 0).map(|(_,e)|(e.0.clone(), e.1.clone())).collect(), &nodes_map, "AA", 0, 26) +
    //             max_pressure_for(&edges_with_indices.iter().filter(|(ii,e)| e.0 == "AA" || (i & (1<<ii)) != 0).map(|(_,e)|(e.0.clone(), e.1.clone())).collect(), &nodes_map, "AA", 0, 26)
    //     }).max().unwrap()
    // //     max_pressure_for_two(&edges, &nodes_map, ("AA", "AA"), (0, 0), (26, 26))
    // // }
    //     ;
    Ok(println!("Total pressure: {}", total_pressure))
    // Ok(println!("Total pressure: {}", total_pressure_2.0 + total_pressure_2.1))
}