use crate::{Map, Direction};

pub fn solution(inp: &Map) -> usize {
    let mut cur_pos = "AAA";
    let mut steps = 0;

    while cur_pos != "ZZZ" {
        let node = inp.nodes.get(cur_pos).unwrap();

        let direction = inp.path[steps%inp.path.len()];

        cur_pos = match direction {
            Direction::Left => &node.left,
            Direction::Right => &node.right,
        };
        steps += 1;
    }
    steps
}