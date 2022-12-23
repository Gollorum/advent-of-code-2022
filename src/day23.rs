use std::cmp::Ordering;
use crate::utils;
use crate::utils::ErrorMsg;
use Dir::{North, South, East, West};

pub fn run_sample() {
    ErrorMsg::print(run("input/day23_sample.txt"));
}

pub fn run_actual() {
    ErrorMsg::print(run("input/day23.txt"));
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Dir {
    North, East, South, West
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum NoEntry {
    None, TooMany
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32
}
impl Pos {
    fn new(x: i32, y: i32) -> Pos { Pos {x, y} }
}
impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Equal => self.x.cmp(&other.x),
            other => other
        }
    }
}
impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn run(path: &str) -> Result<(), ErrorMsg> {
    let mut map: Vec<Vec<bool>> = utils::read_lines(path)?.map(|l_r| l_r?.chars().map(|c|
        match c {
            '.' => Ok(false),
            '#' => Ok(true),
            _ => Err(ErrorMsg{wrapped:format!("Invalid char {c} read")})
        }).collect::<Result<Vec<_>,_>>()).collect::<Result<Vec<_>,_>>()?;
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = map[0].len() as i32;
    let mut max_y = map.len() as i32;
    fn expand(
        dir: Dir, map: &mut Vec<Vec<bool>>, buffers: &mut Vec<Vec<Result<Dir, NoEntry>>>,
        min_x: &mut i32, min_y: &mut i32, max_x: &mut i32, max_y: &mut i32
    ) -> () {
        match dir {
            North => { map.insert(0, vec![false; map[0].len()]); *min_y -= 1 },
            East => {
                for i in 0..map.len() { map[i].push(false) }
                for i in 0..buffers.len() { buffers[i].push(Err(NoEntry::None)) }
                *max_x += 1
            },
            South => { map.push(vec![false; map[0].len()]); *max_y += 1 },
            West => {
                for i in 0..map.len() { map[i].insert(0, false) }
                for i in 0..buffers.len() { buffers[i].insert(0, Err(NoEntry::None)) }
                *min_x -= 1 }
        }
    }
    let mut movement_precedence = vec![North, South, West, East];
    fn get_at(p: Pos, map: &mut Vec<Vec<bool>>, min_x: i32, min_y: i32) -> &mut bool {
        &mut map[(p.y - min_y) as usize][(p.x - min_x) as usize]
    }
    fn desired_move_of(p: Pos, map: &mut Vec<Vec<bool>>, movement_precedence: &Vec<Dir>, min_x: i32, min_y: i32) -> Option<Dir> {
        let n = *get_at(Pos::new(p.x, p.y-1), map, min_x, min_y);
        let ne = *get_at(Pos::new(p.x+1, p.y-1), map, min_x, min_y);
        let e = *get_at(Pos::new(p.x+1, p.y), map, min_x, min_y);
        let se = *get_at(Pos::new(p.x+1, p.y+1), map, min_x, min_y);
        let s = *get_at(Pos::new(p.x, p.y+1), map, min_x, min_y);
        let sw = *get_at(Pos::new(p.x-1, p.y+1), map, min_x, min_y);
        let w = *get_at(Pos::new(p.x-1, p.y), map, min_x, min_y);
        let nw = *get_at(Pos::new(p.x-1, p.y-1), map, min_x, min_y);
        if !(n||ne||e||se||s||sw||w||nw) {return None;}
        movement_precedence.iter().filter(|dir| match dir {
            North => !(nw||n||ne),
            East => !(ne||e||se),
            South => !(sw||s||se),
            West => !(nw||w||sw)
        }).next().map(|&dir| dir)
    }
    let mut buffers = vec![vec![Err(NoEntry::None); map[0].len()]; 4];
    expand(North, &mut map, &mut buffers, &mut min_x, &mut min_y, &mut max_x, &mut max_y);
    expand(East, &mut map, &mut buffers, &mut min_x, &mut min_y, &mut max_x, &mut max_y);
    expand(South, &mut map, &mut buffers, &mut min_x, &mut min_y, &mut max_x, &mut max_y);
    expand(West, &mut map, &mut buffers, &mut min_x, &mut min_y, &mut max_x, &mut max_y);
    fn apply(buffer: &mut Vec<Result<Dir, NoEntry>>, map: &mut Vec<Vec<bool>>, y: usize, extend_l: &mut bool, extend_r: &mut bool, extend_u: &mut bool, extend_d: &mut bool, has_someone_moved: &mut bool) -> () {
        for x in 0..buffer.len() {
            if let Ok(origin) = buffer[x] {
                map[y][x] = true;
                let (old_x, old_y) = match origin {
                    North => (x, y+1),
                    East => (x-1, y),
                    South => (x, y-1),
                    West => (x+1, y)
                };
                map[old_y][old_x] = false;
                *has_someone_moved = true;
                if y == 0 { *extend_u = true }
                else if y == map.len() - 1 { *extend_d = true }
                if x == 0 { *extend_l = true }
                else if x == buffer.len() - 1 { *extend_r = true }
            }
            // print!("{}", match buffer[x] {
            //     Ok(West) => "<",
            //     Ok(North) => "^",
            //     Ok(East) => ">",
            //     Ok(South) => "v",
            //     Err(NoEntry::None) => ".",
            //     Err(NoEntry::TooMany) => "x"
            // });
            buffer[x] = Err(NoEntry::None);
        }
        // println!()
    }
    let mut i = 0;
    loop {
        // println!();
        // println!("{}", map.iter().map(|row| row.iter().map(|&b| if b {'#'} else {'.'}).collect::<String>() + "\n").collect::<String>());
        // println!();
        let mut extend_l = false;
        let mut extend_r = false;
        let mut extend_u = false;
        let mut extend_d = false;
        let mut has_moved = false;
        // let mut column_buffers = vec![vec![None; map[0].len()]; 3];
        for y in (min_y+1)..(max_y-1) {
            for x in (min_x+1)..(max_x-1) {
                if !*get_at(Pos{x,y}, &mut map, min_x, min_y) { continue; }
                if let Some(next_dir) = desired_move_of(Pos::new(x,y), &mut map, &movement_precedence, min_x, min_y) {
                    let (buf_x, buf_y) = match next_dir {
                        North => ((x - min_x) as usize, 1),
                        West => ((x - min_x - 1) as usize, 2),
                        South => ((x - min_x) as usize, 3),
                        East => ((x - min_x + 1) as usize, 2)
                    };
                    if buffers[buf_y][buf_x] == Err(NoEntry::None) {
                        buffers[buf_y][buf_x] = Ok(next_dir)
                    } else { buffers[buf_y][buf_x] = Err(NoEntry::TooMany) }
                }
            }
            if y >= 2 + min_y {
                apply(&mut buffers[0], &mut map, (y - 2 - min_y) as usize, &mut extend_l, &mut extend_r, &mut extend_u, &mut extend_d, &mut has_moved);
            }
            let b = buffers.remove(0);
            buffers.push(b)
        }
        for b_i in 0..4 {
            apply(&mut buffers[b_i], &mut map, (max_y - min_y - 3 + b_i as i32) as usize, &mut extend_l, &mut extend_r, &mut extend_u, &mut extend_d, &mut has_moved);
        }
        if extend_l { expand(West, &mut map, &mut buffers, &mut min_x, &mut min_y, &mut max_x, &mut max_y); }
        if extend_r { expand(East, &mut map, &mut buffers, &mut min_x, &mut min_y, &mut max_x, &mut max_y); }
        if extend_u { expand(North, &mut map, &mut buffers, &mut min_x, &mut min_y, &mut max_x, &mut max_y); }
        if extend_d { expand(South, &mut map, &mut buffers, &mut min_x, &mut min_y, &mut max_x, &mut max_y); }
        let m = movement_precedence.remove(0);
        movement_precedence.push(m);
        i += 1;
        if i == 10 {
            let num_empty: usize = (1..(map.len()-1)).map(|y|
                (1usize..(map[y].len()-1)).filter(|x| !map[y][*x]).count()
            ).sum();
            println!("Map:\n{}\n, Empty tiles: {num_empty}",
                        map.iter().map(|row| row.iter().map(|&b| if b {'#'} else {'.'}).collect::<String>() + "\n").collect::<String>())
        }
        if !has_moved {
            println!("Nobody move! at {i}");
            break;
        }
        has_moved = false;
    }
    Ok(())
}