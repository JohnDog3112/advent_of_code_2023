use crate::HailStone;

pub fn solution(inp: &[HailStone]) -> usize {

    let min = 200000000000000f64;
    let max = 400000000000000f64;

    let mut count = 0;
    for i in 0..inp.len() {
        let s1: f64 = inp[i].vel.1 as f64 / inp[i].vel.0 as f64;
        let y1: f64 = inp[i].pos.1 as f64;
        let x1: f64 = inp[i].pos.0 as f64;
        for j in i+1..inp.len() {
            let s2: f64 = inp[j].vel.1 as f64 / inp[j].vel.0 as f64;
            let y2: f64 = inp[j].pos.1 as f64;
            let x2: f64 = inp[j].pos.0 as f64;

            let x = (y2 - y1 + s1*x1 - s2*x2)/(s1 - s2);

            let y = s2*(x-x2)+y2;

            let t1 = (x-x1)/(inp[i].vel.0 as f64);
            let t2 = (x-x2)/(inp[j].vel.0 as f64);

            //println!("{}, {}; {}, {}", x1, x2, t1, t2);
            if min <= x && x <= max && min <= y && y <= max && t1 > 0f64 && t2 > 0f64 {
                count += 1;
            }
        }
        //println!("{i}");
    }
    
    count
}