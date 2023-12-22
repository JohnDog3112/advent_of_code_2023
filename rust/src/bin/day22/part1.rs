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
    'outer: for i in (0..inp.len()).rev() {
        for j in &supports[i] {
            if supported[*j] < 2 {
                continue 'outer;
            }
        }

        count += 1;
    }

    count
}