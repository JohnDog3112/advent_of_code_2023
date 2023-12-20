use crate::Map;

pub fn solution(inp: &[Map]) -> usize {

    inp.iter().map(solve_map).sum()
}

pub fn solve_map(map: &Map) -> usize {
    //row
    //0 is between row 1 and 2
    //1 is between row 2 and 3, etc.
    'outer: for reflect in 0..map.0.len()-1 {
        let rows_covered = (reflect+1).min(map.0.len()-reflect-1);
        for row in 0..rows_covered {
            for column in 0..map.0[0].len() {
                if map.0[reflect+row+1][column] != map.0[reflect-row][column] {
                    continue 'outer;
                }
            }
        }
        return (reflect+1)*100;
    }

    //column
    //0 is beteen column 1 and 2
    //1 is between column 2 and 3
    'outer: for reflect in 0..map.0[0].len()-1 {
        let columns_covered = (reflect+1).min(map.0[0].len()-reflect-1);
        for column in 0..columns_covered {
            for row in 0..map.0.len() {
                if map.0[row][reflect+column+1] != map.0[row][reflect-column] {
                    continue 'outer;
                }
            }
        }
        return reflect+1;
    }

    0
}