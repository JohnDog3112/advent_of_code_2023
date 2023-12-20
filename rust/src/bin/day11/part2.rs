pub fn solution(inp: Vec<Vec<char>>) -> usize {

    let multiplier = 1000000;
    
    let mut expanded_lines = vec![];
    for (i, line) in inp.iter().enumerate() {
        let mut expand = true;
        for ch in line {
            if *ch != '.' {
                expand = false;
                break;
            }
        }
        if expand {
            expanded_lines.push(i);
        }

    }

    let mut expanded_columns = vec![];
    for i in 0..inp[0].len() {
        let mut expand = true;
        for line in &inp {
            if line[i] != '.' {
                expand = false;
                break;
            }
        }
        if expand {
            expanded_columns.push(i);
        }
    }
    
   /*for line in &inp {
        println!("{}", line.iter().collect::<String>())
    }*/

    let mut galaxies = vec![];

    //get all the base un-expanded galaxies
    for (i, line) in inp.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if *ch == '#' {
                galaxies.push((i, j));
            }
        }
    }

    //expand all the galaxies
    for (row, column) in &mut galaxies {
        let mut expansions = 0;
        while expansions < expanded_lines.len() && *row > expanded_lines[expansions] {
            expansions += 1;
        }
        *row += expansions*(multiplier-1);

        let mut expansions = 0;
        while expansions < expanded_columns.len() && *column > expanded_columns[expansions] {
            expansions += 1;
        }
        *column += expansions*(multiplier-1);
    }

    let mut total_dist = 0;

    for i in 0..galaxies.len() {
        let a = galaxies[i];
        for &b in galaxies.iter().skip(i+1) {
            total_dist += b.0.abs_diff(a.0);
            total_dist += b.1.abs_diff(a.1);
        }
    }
    total_dist
}