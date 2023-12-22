use std::collections::{HashSet, HashMap, BinaryHeap};

use crate::Brick;

pub fn solution(mut inp: Vec<Brick>) -> usize {


    for i in 0..inp.len() {
        let mut found_z = 0;
        for j in (0..i).rev() {
            if inp[j].is_below(&inp[i]) {
                found_z = inp[j].z.higher();
                break;
            }
        }
        found_z += 1;

        match &mut inp[i].z {
            crate::Pos::Int(a) => *a = found_z,
            crate::Pos::Range(a, b) => {
                *b -= *a-found_z;
                *a = found_z;
            },
        }
        inp.sort();
    }

    let mut supports = vec![vec![]; inp.len()];
    let mut supported = vec![0_i64; inp.len()];

    for i in 0..inp.len() {
        let mut found_z = 0;
        for j in (0..i).rev() {
            if found_z != 0 && found_z != inp[j].z.higher() {
                continue;
            }
            if inp[j].is_below(&inp[i]) {
                found_z = inp[j].z.higher();
                supported[i] += 1;
                supports[j].push(i);
            }
        }
    }
    let mut count = 0;

    /*for l in &supports {
        println!("{:?}", l);
    }*/
    for i in 0..supports.len() {
        let tmp = find_num_supporting(i, &supports, &mut supported, &inp);
        
        //println!("{i}: {tmp}");
        count += tmp;
    }

    count
}

fn find_num_supporting(index: usize, supports: &[Vec<usize>], supported: &mut [i64], bricks: &[Brick]) -> usize {
    let mut total = 0;

    let mut layers = HashMap::new();
    layers.insert(bricks[index].z.higher(), vec![index]);

    let mut check_order = BinaryHeap::new();
    check_order.push(-(bricks[index].z.higher()));


    while let Some(next) = check_order.pop() {
        let layer = layers.remove(&-next).unwrap();

        for i in &layer {
            for j in &supports[*i] {
                supported[*j] -= 1;
            }
        }

        let mut to_check = HashSet::new();
        
        for i in &layer{
            for j in &supports[*i] {
                if supported[*j] == 0 {
                    to_check.insert(*j);
                }
            }
        }

        for i in &layer {
            for j in &supports[*i] {
                supported[*j] += 1;
            }
        }
        
        total += to_check.len();

        for index in to_check {
            let height = bricks[index].z.higher();
            if let Some(layer) = layers.get_mut(&height) {
                layer.push(index);
            } else {
                layers.insert(height, vec![index]);
                check_order.push(-height);
            }
        }
    }

    total
}