use std::collections::HashSet;

use crate::HailStone;


//hints used: 
// -- "Look at Parallel Lines"
// -- referenced https://pastebin.com/VrYHgxKx for last part
//Probably would not have been able to solve this without them
pub fn solution(inp: &[HailStone]) -> i64 {

    
    let mut xs: Option<HashSet<i64>> = None;
    let mut ys: Option<HashSet<i64>> = None;
    let mut zs: Option<HashSet<i64>> = None;

    for i in 0..inp.len() {
        let vel1 = inp[i].vel;
        for j in i+1..inp.len() {
            let vel2 = inp[j].vel;
            if vel1.0 == vel2.0 {
                narrow_velocity(inp[i].pos.0, inp[j].pos.0, vel1.0, &mut xs);
            }
            if vel1.1 == vel2.1 {
                narrow_velocity(inp[i].pos.1, inp[j].pos.1, vel1.1, &mut ys);
            }
            if vel1.2 == vel2.2 {
                narrow_velocity(inp[i].pos.2, inp[j].pos.2, vel1.2, &mut zs);
            }

        }
        //println!("{i}");
    }

    //println!("{:?}", xs);
    //println!("{:?}", ys);
    //println!("{:?}", zs);

    // let vx = -3;
    // let vy = 1;
    // let vz = 2;

    let vx = xs.unwrap().into_iter().next().unwrap();
    let vy = ys.unwrap().into_iter().next().unwrap();
    let vz = zs.unwrap().into_iter().next().unwrap();

    //println!("{vx}, {vy}, {vz}");

    let a = &inp[0];
    let b = &inp[1];
    
    let slope_a = (a.vel.1 as f64 - vy as f64)/(a.vel.0 as f64 - vx as f64);
    let slope_b = (b.vel.1 as f64 - vy as f64)/(b.vel.0 as f64 - vx as f64);
    

    let x = (b.pos.1 as f64 - a.pos.1 as f64 + slope_a*a.pos.0 as f64- slope_b*b.pos.0 as f64)/(slope_a - slope_b);
    let x = x.floor() as i64;

    let time = (a.pos.0- x)/(vx - a.vel.0);

    let y = a.pos.1 + (a.vel.1 - vy) * time;

    let z = a.pos.2 + (a.vel.2 - vz) * time;
    
    //println!("{x}, {y}, {z}");

    x + y + z
}

fn narrow_velocity(p0: i64, p1: i64, vel: i64, valid_set: &mut Option<HashSet<i64>>) {
    let (p0, p1) = {
        if p0 < p1 {
            (p0, p1)
        } else {
            (p1, p0)
        }
    };
    
    let mut valid = HashSet::new();
    //println!("{:?}, {:?}", a, b);
    let min_forward = vel + 1;
    let max_forward = {
        let tmp = vel + (p1 - p0);
        if tmp > 300 {
            300
        } else {
            tmp
        }
    };
    
    let min_back = {
        let tmp = vel - (p1 - p0);
        if tmp < -300 {
            -300
        } else {
            tmp
        }
    };
    let max_back = vel - 1;

    for v in min_forward..=max_forward {
        let r = (p1 - p0)%(v - vel);
        if r == 0 {
            valid.insert(v);
        }
    }

    for v in min_back..=max_back {
        let r = (p1 - p0)%(v - vel);
        if r == 0 {
            valid.insert(v);
        }
    }

    if let Some(xs) = valid_set {
        let mut to_remove = vec![];
        for val in xs.iter() {
            if !valid.contains(val) {
                to_remove.push(*val);
            }
        }

        for val in to_remove {
            xs.remove(&val);
        }

    } else {
        *valid_set = Some(valid);
    }
}