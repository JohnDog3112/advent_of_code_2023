use crate::{Map, Direction};

pub fn solution(inp: &Map) -> usize {
    let start_nodes: Vec<String> = inp.nodes.clone().into_keys()
        .filter(|node| {
            node.chars().nth(2).unwrap() == 'A'
        }).collect();


    let cycles = start_nodes.into_iter()
        .map(|node| 
            get_loops(inp, node)
        ).collect::<Vec<usize>>();
    
    cycles.into_iter().reduce(|a, b| {
            lcm(a,b)
        }).unwrap()
}

fn lcm(a: usize, b: usize) -> usize {
    (a*b)/gcd(a,b)
}
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a%b)
}
fn get_loops(map: &Map, starting_point: String) -> usize {
    let mut cur_node = starting_point;

    let mut steps_to_loop = 0;
    
    while cur_node.chars().nth(2).unwrap() != 'Z' {
        let node = map.nodes.get(&cur_node).unwrap();
        let dir = map.path[steps_to_loop%map.path.len()];

        steps_to_loop += 1;

        cur_node = match dir {
            Direction::Left => node.left.clone(),
            Direction::Right => node.right.clone(),
        };
    }
    steps_to_loop
}