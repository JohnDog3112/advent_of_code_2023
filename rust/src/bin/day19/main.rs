use std::{fs, collections::HashMap};

mod part1;
mod part2;
fn main() {
    let (workflows, parts) = parse_input();

    let sol1 = part1::solution(&workflows, &parts);
    println!("part 1: {sol1}");

    let sol2 = part2::solution(&workflows);
    println!("part 2: {sol2}");
}


#[derive(Clone, Debug)]
pub struct Part {
    pub properties: [usize; 4]
}
#[derive(Clone, Debug)]
pub struct Workflow {
    pub conditions: Vec<Condition>,
    pub final_dest: String,
}
#[derive(Clone, Debug)]
pub struct Condition {
    pub property: Property,
    pub comparison: Comparison,
    pub val: usize,
    pub destination: String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparison {
    LessThan,
    GreaterThan
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Property {
    Cool,
    Musical,
    Aerodynamic,
    Shiny,
}

fn parse_input() -> (HashMap<String, Workflow>, Vec<Part>) {
    let inp = fs::read_to_string("src/bin/day19/input.txt").expect("Error in reading the file");

    let segments: Vec<&str> = inp.split("\n\n").collect();

    let workflowsconditions: HashMap<String, Workflow> = HashMap::from_iter(segments[0].split('\n').map(|workflow| {
        let parts: Vec<&str> = workflow.split('{').collect();
        
        let id = parts[0].to_string();

        let conditions: Vec<&str> = parts[1].split(',').collect();

        let last_condition = conditions[conditions.len()-1];
        let last_condition = last_condition[0..last_condition.len()-1].to_string();
        
        let parsed_conditions = conditions[0..conditions.len()-1]
            .iter()
            .map(|condition| {
                let parts: Vec<&str> = condition.split(':').collect();
                
                let chars: Vec<char> = parts[0].chars().collect();

                let prop = match chars[0] {
                    'x' => Property::Cool,
                    'm' => Property::Musical,
                    'a' => Property::Aerodynamic,
                    's' => Property::Shiny,
                    _ => unreachable!()
                };

                let comparison = match chars[1] {
                    '<' => Comparison::LessThan,
                    '>' => Comparison::GreaterThan,
                    _ => unreachable!()
                };

                let val: usize = parts[0][2..].parse().unwrap();

                let dest = parts[1].to_string();

                Condition {
                    property: prop,
                    comparison,
                    val,
                    destination: dest,
                }
            }).collect();
        
        (id, Workflow {
            conditions: parsed_conditions,
            final_dest: last_condition
        })            
    }));

    let part = segments[1].split('\n').map(|line| {
        let line = &line[1..line.len()-1];

        let properties: Vec<usize> = line.split(',').map(|prop| {
            let segments: Vec<&str> = prop.split('=').collect();

            segments[1].parse().unwrap()
        }).collect();

        Part {
            properties: properties.try_into().unwrap()
        }
    }).collect();

    (workflowsconditions, part)
}