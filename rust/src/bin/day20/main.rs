use std::{fs, collections::HashMap};

mod part1;
mod part2;
fn main() {
    let inp = parse_input();

    let sol1 = part1::solution(inp.clone());
    println!("part 1: {sol1}");

    let sol2 = part2::solution(inp);
    println!("part 2: {sol2}");
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Pulse {
    High,
    Low
}

#[derive(Clone, Debug)]
pub enum Module {
    Broadcast(Vec<String>),
    FlipFlop(bool, Vec<String>),
    Conjunction(HashMap<String, Pulse>, Vec<String>),
    Untyped,
}
fn parse_input() -> HashMap<String, Module> {
    let inp = fs::read_to_string("src/bin/day20/input.txt").expect("Error in reading the file");

    let mut modules: HashMap<String, Module> = HashMap::from_iter(inp.split('\n').map(|line| {
        let segments: Vec<&str> = line.split(" -> ").collect();
        
        let dest:Vec<String> = segments[1].split(", ").map(|a| a.to_string()).collect();
        
        let first_char: char = segments[0].chars().next().unwrap();
        
        let mut name = segments[0][1..].to_string();
        let module = if segments[0] == "broadcaster" {
            name = segments[0].to_string();
            Module::Broadcast(dest)
        } else if first_char == '%' {
            Module::FlipFlop(false, dest)
        } else if first_char == '&' {
            Module::Conjunction(HashMap::new(), dest)
        } else {
            Module::Untyped
        };

        (name, module)
    }));

    let keys: Vec<String> = modules.keys().map(|a| a.to_owned()).collect();

    for name in keys {

        let destinations = match modules.get(&name).unwrap() {
            Module::Broadcast(dest)
            | Module::FlipFlop(_, dest)
            | Module::Conjunction(_, dest) => {
                dest.to_owned()
            },
            Module::Untyped => continue,
        };

        for dest in destinations {
            if let Some(module) = modules.get_mut(&dest) {
                if let Module::Conjunction(inputs, _) = module {
                    inputs.insert(name.clone(), Pulse::Low);
                }
            } else {
                modules.insert(dest, Module::Untyped);
            }
            
        }
    }

    /*for (name, module) in &modules {
        println!("{name} -> {:?}", module);
    }*/

    modules
}