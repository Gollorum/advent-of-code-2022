use std::collections::HashMap;
use std::ops::{Add, Sub};
use crate::utils::ErrorMsg;
use std::str::FromStr;
use regex::Regex;
use crate::utils;

pub fn run_sample() {
    ErrorMsg::print(run("input/day19_sample.txt", true));
}

pub fn run_actual() {
    ErrorMsg::print(run("input/day19.txt", true));
}

#[derive(Default, Clone, Copy, Eq, PartialEq, Hash)]
struct Material {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32
}
impl Add<Material> for Material {
    type Output = Material;
    fn add(self, rhs: Material) -> Self::Output {
        Material {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode
        }
    }
}
impl Sub<Material> for Material {
    type Output = Material;
    fn sub(self, rhs: Material) -> Self::Output {
        Material {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode
        }
    }
}
impl Material {
    fn except_geodes(&self) -> (u32, u32, u32) { (self.ore, self.clay, self.obsidian) }
}

lazy_static! {
    static ref ORE_REGEX: Regex = Regex::new(r"Each ore robot costs (\d+) ore.").unwrap();
    static ref CLAY_REGEX: Regex = Regex::new(r"Each clay robot costs (\d+) ore.").unwrap();
    static ref OBSIDIAN_REGEX: Regex = Regex::new(r"Each obsidian robot costs (\d+) ore and (\d+) clay.").unwrap();
    static ref GEODE_REGEX: Regex = Regex::new(r"Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
}
struct Blueprint {
    ore_robot_cost: Material,
    clay_robot_cost: Material,
    obsidian_robot_cost: Material,
    geode_robot_cost: Material,
    expand_options: [(Material, Material); 5]
}
impl Blueprint {
    fn new(
       ore_robot_cost: Material,
       clay_robot_cost: Material,
       obsidian_robot_cost: Material,
       geode_robot_cost: Material
    ) -> Blueprint { Blueprint {
        ore_robot_cost,
        clay_robot_cost,
        obsidian_robot_cost,
        geode_robot_cost,
        expand_options: [
            (ore_robot_cost, Material{ore: 1, ..Default::default()}),
            (clay_robot_cost, Material{clay: 1, ..Default::default()}),
            (obsidian_robot_cost, Material{obsidian: 1, ..Default::default()}),
            (geode_robot_cost, Material{geode: 1, ..Default::default()}),
            (Material{..Default::default()}, Material{..Default::default()})
        ]
    }}
}
impl FromStr for Blueprint {
    type Err = ErrorMsg;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ore = ORE_REGEX.captures(s).ok_or(ErrorMsg{wrapped:format!("No ore robot cost found in {s}")})?;
        let clay = CLAY_REGEX.captures(s).ok_or(ErrorMsg{wrapped:format!("No clay robot cost found in {s}")})?;
        let obsidian = OBSIDIAN_REGEX.captures(s).ok_or(ErrorMsg{wrapped:format!("No obsidian robot cost found in {s}")})?;
        let geode = GEODE_REGEX.captures(s).ok_or(ErrorMsg{wrapped:format!("No geode robot cost found in {s}")})?;
        Ok(Blueprint::new(
            Material {ore: ore[1].parse()?, ..Default::default()},
            Material {ore: clay[1].parse()?, ..Default::default()},
            Material {ore: obsidian[1].parse()?, clay: obsidian[2].parse()?, ..Default::default()},
            Material {ore: geode[1].parse()?, obsidian: geode[2].parse()?, ..Default::default()}
        ))
    }
}

fn max_geodes_for(blueprint: &Blueprint, current_materials: Material, current_robots: Material, time_left: u32, cache: &mut HashMap<((u32, u32, u32),(u32, u32, u32),u32),u32>) -> u32 {
    if time_left == 0 { return current_materials.geode; }
    if let Some(res) = cache.get(&(current_materials.except_geodes(), current_robots.except_geodes(), time_left)) {
        return *res + current_materials.geode + current_robots.geode * time_left
    }
    let res = blueprint.expand_options.iter().filter(|(cost,_)| cost.ore<=current_materials.ore && cost.clay<=current_materials.clay && cost.obsidian<=current_materials.obsidian)
        .map(|&(cost, gain)| max_geodes_for(
            blueprint,
            current_materials + current_robots - cost,
            current_robots + gain,
            time_left - 1,
            cache
        )).max().unwrap();
    cache.insert((current_materials.except_geodes(), current_robots.except_geodes(), time_left), res - (current_materials.geode + current_robots.geode * time_left));
    res
}

fn run(path: &str, part2: bool) -> Result<(), ErrorMsg> {
    let blueprints = utils::read_lines(path)?.map(|l| l?.parse()).collect::<Result<Vec<Blueprint>, ErrorMsg>>()?;
    if !part2 {
        let mut sum = 0;
        for (i, blueprint) in blueprints.iter().enumerate() {
            println!("STarting blueprint {i}");
            let max_geodes = max_geodes_for(
                blueprint,
                Material { ..Default::default() },
                Material { ore: 1, ..Default::default() },
                24,
                &mut HashMap::new()
            );
            println!("Max was {max_geodes}");
            sum += (i as u32 + 1) * max_geodes;
        }
        Ok(println!("Result: {sum}"))
    } else {
        let reduced_blueprints = blueprints.iter().take(3).collect::<Vec<_>>();
        let mut prod = 1;
        for (i, blueprint) in reduced_blueprints.iter().enumerate() {
            println!("STarting blueprint {i}");
            let max_geodes = max_geodes_for(
                blueprint,
                Material { ..Default::default() },
                Material { ore: 1, ..Default::default() },
                32,
                &mut HashMap::new()
            );
            println!("Max was {max_geodes}");
            prod *= max_geodes;
        }
        Ok(println!("Result: {prod}"))
    }
}