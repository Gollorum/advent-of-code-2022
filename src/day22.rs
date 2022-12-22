use std::ops::Add;
use crate::day22::Instruction::{Move, Rotate};
use crate::day22::Tile::{Free, Wall};
use crate::{hashmap, utils};
use crate::utils::ErrorMsg;

pub fn run_sample() {
    ErrorMsg::print(run(
        "input/day22_sample.txt",
        [(2,0), (0,1), (1,1), (2,1), (2,2), (3,2)],
        [ // part 2
            [(5, 2), (3, 0), (2, 3), (1, 2)],
            [(2, 0), (4, 2), (5, 1), (0, 2)],
            [(3, 0), (4, 3), (1, 0), (0, 1)],
            [(5, 1), (4, 0), (2, 0), (0, 0)],
            [(5, 0), (1, 2), (2, 1), (3, 0)],
            [(0, 2), (1, 3), (4, 0), (3, 3)]
        ],
        // [ // part1
        //     [(0, 0), (3, 0), (0, 0), (4, 0)],
        //     [(2, 0), (1, 0), (3, 0), (1, 0)],
        //     [(3, 0), (2, 0), (1, 0), (2, 0)],
        //     [(1, 0), (4, 0), (2, 0), (0, 0)],
        //     [(5, 0), (0, 0), (5, 0), (3, 0)],
        //     [(4, 0), (5, 0), (4, 0), (5, 0)]
        // ],
        4
    ));
}

pub fn run_actual() {
    ErrorMsg::print(run(
        "input/day22.txt",
        [(1,0), (2,0), (1,1), (0,2), (1,2), (0,3)],
        [ // part 2
            [(1,0), (2,0), (3,2), (5,1)],
            [(4,2), (2,1), (0,0), (5,0)],
            [(1,3), (4,0), (3,3), (0,0)],
            [(4,0), (5,0), (0,2), (2,1)],
            [(1,2), (5,1), (3,0), (2,0)],
            [(4,3), (1,0), (0,3), (3,0)]
        ],
        // [ // part 1
        //     [(1,0), (2,0), (1,0), (4,0)],
        //     [(0,0), (1,0), (0,0), (1,0)],
        //     [(2,0), (4,0), (2,0), (0,0)],
        //     [(4,0), (5,0), (4,0), (5,0)],
        //     [(3,0), (0,0), (3,0), (2,0)],
        //     [(5,0), (3,0), (5,0), (3,0)]
        // ],
        50
    ));
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
    Free, Wall
}

enum Instruction {
    Move(u32),
    Rotate(u8)
}

#[derive(Clone, Copy)]
struct Pose {
    face: usize,
    x: usize,
    y: usize,
    rot: u8
}

fn run(path: &str, face_locations: [(usize, usize); 6], edge_mapping: [[(usize, u8); 4]; 6], face_len: usize) -> Result<(), ErrorMsg> {
    let mut lines = utils::read_lines_to_vec(path)?;
    let mut map: [Vec<Vec<Tile>>; 6] = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    for face_i in 0..6 {
        let (face_x, face_y) = face_locations[face_i];
        let map_entry = &mut map[face_i];
        for y in (face_len * face_y)..(face_len * (face_y + 1)) {
            map_entry.push(lines[y][(face_len * face_x)..(face_len * (face_x + 1))].chars().map(|c| match c {
                '.' => Ok(Free),
                '#' => Ok(Wall),
                _ => Err(ErrorMsg { wrapped: format!("Invalid char {c}") })
            }).collect::<Result<Vec<Tile>, ErrorMsg>>()?)
        }
    }
    let instructions_line = lines.last().ok_or(ErrorMsg::new("Did not find instruction line"))?;
    let mut instructions: Vec<Instruction> = Vec::new();
    for c in instructions_line.chars() {
        match c {
            'L' => Ok(instructions.push(Rotate(3))),
            'R' => Ok(instructions.push(Rotate(1))),
            '0'..='9' => Ok({
                let by = c.to_digit(10).ok_or(ErrorMsg{wrapped:format!("Failed to parse char {c}")})?;
                if let Some(Move(last_move_ref)) = instructions.last() {
                    let last_move = *last_move_ref;
                    instructions.pop();
                    instructions.push(Move(last_move * 10 + by))
                } else { instructions.push(Move(by)) }
            }),
            _ => Err(ErrorMsg{wrapped:format!("Failed to parse char {c}")})
        }?
    }
    let next = |mut p: Pose| -> Result<Pose, ErrorMsg> {
        match p.rot {
            0 => {
                if p.x == (face_len-1) {
                    let (new_face, rot_offset) = edge_mapping[p.face][0];
                    p.face = new_face;
                    p.rot = rot_offset;
                    (p.x, p.y) = match rot_offset {
                        0 => Ok((0, p.y)),
                        1 => Ok((face_len-p.y-1, 0)),
                        2 => Ok((face_len-1, face_len-p.y-1)),
                        3 => Ok((p.y, face_len-1)),
                        _ => Err(ErrorMsg::new("Unexpected rotation"))
                    }?
                } else {
                    p.x += 1
                }
            },
            1 => {
                if p.y == (face_len-1) {
                    let (new_face, rot_offset) = edge_mapping[p.face][1];
                    p.face = new_face;
                    p.rot = (rot_offset + 1) & 3;
                    (p.x, p.y) = match rot_offset {
                        0 => Ok((p.x, 0)),
                        1 => Ok((face_len - 1, p.x)),
                        2 => Ok((face_len-p.x-1, face_len-1)),
                        3 => Ok((0, face_len-p.x-1)),
                        _ => Err(ErrorMsg::new("Unexpected rotation"))
                    }?
                } else {
                    p.y += 1;
                }
            },
            2 => {
                if p.x == 0 {
                    let (new_face, rot_offset) = edge_mapping[p.face][2];
                    p.face = new_face;
                    p.rot = (rot_offset + 2) & 3;
                    (p.x, p.y) = match rot_offset {
                        0 => Ok((face_len-1, p.y)),
                        1 => Ok((face_len-p.y-1, face_len-1)),
                        2 => Ok((0, face_len-p.y-1)),
                        3 => Ok((p.y, 0)),
                        _ => Err(ErrorMsg::new("Unexpected rotation"))
                    }?
                } else {
                    p.x -= 1;
                }
            },
            3 => {
                if p.y == 0 {
                    let (new_face, rot_offset) = edge_mapping[p.face][3];
                    p.face = new_face;
                    p.rot = (rot_offset + 3) & 3;
                    (p.x, p.y) = match rot_offset {
                        0 => Ok((p.x, face_len-1)),
                        1 => Ok((0, p.x)),
                        2 => Ok((face_len-p.x-1, 0)),
                        3 => Ok((face_len-1, face_len-p.x-1)),
                        _ => Err(ErrorMsg::new("Unexpected rotation"))
                    }?
                } else {
                    p.y -= 1;
                }
            },
            _ => Err(ErrorMsg { wrapped: format!("Invalid rotation {}", p.rot) })?
        }
        Ok(p)
    };
    let mut pos = Pose {face: 0, x: 0, y: 0, rot: 0};
    for i in instructions {
        match i {
            Rotate(r) => pos.rot = (pos.rot + r) % 4,
            Move(m) => {
                for _ in 0..m {
                    let new_pos = next(pos)?;
                    if map[new_pos.face][new_pos.y][new_pos.x] == Wall {
                        break;
                    } else {pos = new_pos}
                }
            }
        }
    }
    let x = face_len * face_locations[pos.face].0 + pos.x + 1;
    let y = face_len * face_locations[pos.face].1 + pos.y + 1;
    Ok(println!("Ending at {x}, {y} at rot:{}\n => result is {}", pos.rot, 1000 * y + 4 * x + pos.rot as usize))
}