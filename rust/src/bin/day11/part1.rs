pub fn solution(mut inp: Vec<Vec<char>>) -> usize {
    let mut i = 0;

    let expansion_line: Vec<char> = ".".repeat(inp[0].len()).chars().collect();
    while i < inp.len() {
        let mut expand = true;
        for ch in &inp[i] {
            if *ch != '.' {
                expand = false;
                break;
            }
        }
        if expand {
            inp.insert(i, expansion_line.clone());
            i += 1;
        }
        i += 1;
    }

    let mut i = 0;

    while i < inp[0].len() {
        let mut expand = true;
        for line in &inp {
            if line[i] != '.' {
                expand = false;
                break;
            }
        }
        if expand {
            for line in &mut inp {
                line.insert(i, '.');
            }
            i += 1;
        }
        i += 1;
    }
   /*for line in &inp {
        println!("{}", line.iter().collect::<String>())
    }*/

    let mut galaxies = vec![];

    for (i, line) in inp.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if *ch == '#' {
                galaxies.push((i, j));
            }
        }
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