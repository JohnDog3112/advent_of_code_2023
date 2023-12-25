use std::{collections::{HashMap, HashSet}, fs};

pub fn solution(mut inp: HashMap<String, HashSet<String>>) -> usize {
    //generate_graph(inp);

    let to_remove = [("kns", "dct"), ("nqq", "pxp"), ("jxb", "ksq")];

    for remove in to_remove {
        let tmp = inp.get_mut(remove.0).unwrap();
        tmp.remove(remove.1);

        let tmp = inp.get_mut(remove.1).unwrap();
        tmp.remove(remove.0);
    }

    let a = count_nodes(&inp, to_remove[0].0);
    let b = count_nodes(&inp, to_remove[0].1);

    println!("{a}, {b}");

    a*b
}

fn count_nodes(graph: &HashMap<String, HashSet<String>>, start: &str) -> usize{
    let mut nodes = HashSet::new();

    nodes.insert(start.to_string());
    let mut to_check = vec![start.to_string()];

    while let Some(check) = to_check.pop() {
        for connection in graph.get(&check).unwrap() {
            if !nodes.contains(connection) {
                nodes.insert(connection.clone());
                to_check.push(connection.clone());
            }
        }
    }

    nodes.len()
}
#[allow(unused)]
fn generate_graph(inp: &HashMap<String, HashSet<String>>) {

    let mut out = "strict graph {\n".to_string();

    for (name, connections) in inp {
        let mut tmp = name.clone() + " -- {";

        for (i, connection) in connections.iter().enumerate() {
            if i != 0 {
                tmp += " ";
            }
            tmp += connection;
        }
        tmp += "}";

        out += &(tmp + "\n");
    }
    out += "}";

    fs::write("src/bin/day25/graph", out).unwrap();
}