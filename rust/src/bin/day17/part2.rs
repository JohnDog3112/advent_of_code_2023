use std::collections::{HashMap, BinaryHeap};

#[allow(unused)]
use crate::{Process, print_grid, Dir, Node};

pub fn solution(inp: &Vec<Vec<i32>>) -> i32 {

    let mut grid = vec![vec![Node {connections: HashMap::new()}; inp[0].len()]; inp.len()];

    let start = (0,0);

    grid[start.1][start.0].connections.insert((Dir::None, 0), 0);
    
    let mut to_process = BinaryHeap::new();
    to_process.push(Process {
        x: 0,
        y: 0,
        dir: Dir::None,
        consecutive: 0,
        weight: 0,
    });

    while let Some(cur_node) = to_process.pop() {
        for dir in cur_node.dir.get_other_dirs() {
            let offset = dir.get_offset();
            let x = cur_node.x + offset.0;
            let y = cur_node.y + offset.1;

            if x < 0 || y < 0 || x >= inp[0].len() as i32 || y >= inp.len() as i32 {
                continue;
            }

            let consecutive = {
                if dir == cur_node.dir {
                    cur_node.consecutive + 1
                } else {
                    0
                }
            };

            if consecutive > 9 {
                continue;
            }
            if dir != cur_node.dir && cur_node.dir != Dir::None{
                if cur_node.consecutive < 3 {
                    continue;
                }
                let x = x + offset.0*3;
                let y = y + offset.1*3;

                if x < 0 || y < 0 || x >= inp[0].len() as i32 || y >= inp.len() as i32 {
                    continue
                }
            }

            let target_node = &mut grid[y as usize][x as usize];
            
            let new_weight = cur_node.weight + inp[y as usize][x as usize];

            if let Some(weight) = target_node.connections.get_mut(&(dir, consecutive)) {
                if *weight <= new_weight {
                    continue;
                }
                *weight = new_weight;
            } else {
                target_node.connections.insert((dir, consecutive), new_weight);
            }

            to_process.push(Process {
                x,
                y,
                dir,
                consecutive,
                weight: new_weight
            });
        }
    }

    //println!("{:?}", grid[4][10]);
    //print_grid(&grid);

    grid[grid.len()-1][grid[0].len()-1].get_min_weight()
}

