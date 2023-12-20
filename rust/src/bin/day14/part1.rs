use crate::Tile;

pub fn solution(mut inp: Vec<Vec<Tile>>) -> usize {
    //print_grid(&inp);
    //println!("----------------");
    //move every tile north
    for row in 0..inp.len() {
        'a: for col in 0..inp[0].len() {
            if inp[row][col] != Tile::MoveRock {
                continue;
            }
            for up_amt in 0..row {
                if inp[row-up_amt-1][col] != Tile::Empty {
                    inp[row][col] = Tile::Empty;
                    inp[row-up_amt][col] = Tile::MoveRock;
                    continue 'a;
                }
            }
            //didn't hit anything
            inp[row][col] = Tile::Empty;
            inp[0][col] = Tile::MoveRock;
        }
    }

    //print_grid(&inp);


    inp.iter().enumerate().map(|(i, row)| {
        let move_rocks = row.iter().filter(|tile| **tile == Tile::MoveRock).count();
        move_rocks * (inp.len()-i)
    }).sum()
}