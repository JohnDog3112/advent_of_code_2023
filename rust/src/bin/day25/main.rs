use std::{fs, collections::{HashMap, HashSet}};

mod part1;
fn main() {
    let inp = parse_input();

    let sol1 = part1::solution(inp);
    println!("part 1: {sol1}");

}


fn parse_input() -> HashMap<String, HashSet<String>> {
    let inp: String = fs::read_to_string("src/bin/day25/input.txt").expect("Error in reading the file");

    let mut nodes: HashMap<String, HashSet<String>> = HashMap::new();

    for line in inp.split('\n') {
        let parts: Vec<&str> = line.split(": ").collect();
        

        for connection in parts[1].split(' ') {
            if let Some(node) = nodes.get_mut(parts[0]) {
                node.insert(connection.to_string());
            } else {
                let mut tmp = HashSet::new();
                tmp.insert(connection.to_string());
                nodes.insert(parts[0].to_string(), tmp);
            }

            if let Some(node) = nodes.get_mut(connection) {
                node.insert(parts[0].to_string());
            } else {
                let mut tmp = HashSet::new();
                tmp.insert(parts[0].to_string());
                nodes.insert(connection.to_string(), tmp);
            }
        }
    }
    nodes
}