use std::collections::{HashMap, VecDeque, HashSet};

use crate::{Module, Pulse};

pub fn solution(modules: HashMap<String, Module>) -> usize {
    
    let final_gate = {
        let mut op = None;

        for (name, module) in &modules {
            match module {
                Module::Broadcast(out)
                | Module::FlipFlop(_, out)
                | Module::Conjunction(_, out) => {
                    if out.len() == 1 && out[0] == "rx" {
                        op = Some(name.clone());
                        break;
                    }
                },
                Module::Untyped => (),
            }
        }
        op.unwrap()
    };
    
    let sections;
    if let Module::Broadcast(out) = modules.get("broadcaster").unwrap() {
        sections = out;
    } else {
        unreachable!()
    }

    let mut answer = 1;
    for section in sections.clone() {
        let mut map = HashMap::new();
        map.insert("broadcaster".to_string(), Module::Broadcast(vec![section.clone()]));

        let mut to_add = vec![section];

        while let Some(val) = to_add.pop() {
            if map.contains_key(&val) {
                continue;
            }

            let module = modules.get(&val).unwrap();

            

            let output = match module {
                Module::Broadcast(_) => unreachable!(),
                Module::FlipFlop(_, out)
                | Module::Conjunction(_, out) => out,
                Module::Untyped => {
                    map.insert(val, module.clone());
                    continue;
                },
            };

            let output: Vec<String> = output.clone().into_iter().map(|a| {
                if a == final_gate {
                    "rx".to_string()
                } else {
                    a
                }
            }).collect();
            
            map.insert(val, match module.clone() {
                Module::Broadcast(_) => Module::Broadcast(output.clone()),
                Module::FlipFlop(state, _) => Module::FlipFlop(state, output.clone()),
                Module::Conjunction(state, _) => Module::Conjunction(state, output.clone()),
                Module::Untyped => unreachable!(),
            });

            for module in output {
                if module == final_gate {
                    continue;
                }
                to_add.push(module.clone());
            }
        }

        //print_map(&map);
        

        let mut rx_flips = vec![];
        let mut states: HashSet<Vec<HashableModule>> = HashSet::new();


        states.insert(map.clone().into_values().map(|a| a.into()).collect());

        loop {
            let pulses = push_button(&mut map);
            rx_flips.push(pulses);

            let state = map.clone().into_values().map(|a| a.into()).collect();

            if states.contains(&state) {
                break;
            }
            states.insert(state);
            
        }
        
        //println!("Pressed {} times!", states.len());
        //println!("{:?}", rx_flips[rx_flips.len()-2][0]);

        //println!("----------");
        answer = lcm(answer, states.len()-1);
    }

    
    answer
}

fn lcm(a: usize, b: usize) -> usize {
    a/gcd(a,b) * b
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0{
        return a;
    }
    gcd(b, a%b)
}

#[allow(unused)]
fn print_map(map: &HashMap<String, Module>) {
    for (name, module) in map {
        let prefix = match module {
            Module::Broadcast(_) => "",
            Module::FlipFlop(_, _) => "%",
            Module::Conjunction(_, _) => "&",
            Module::Untyped => continue,
        };

        let out = match module {
            Module::Broadcast(out)
            | Module::FlipFlop(_, out)
            | Module::Conjunction(_, out) => {
                let mut output = "".to_string();
                for (i, st) in out.iter().enumerate() {
                    if i != 0 {
                        output += ", ";
                    }
                    output += st;
                }
                output
            },
            Module::Untyped => unreachable!()
        };

        println!("{prefix}{name} -> {out}");
    }
}
pub fn push_button(modules: &mut HashMap<String, Module>) -> Vec<(usize, Pulse)> {
    let mut to_process = VecDeque::new();

    to_process.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));

    let mut iterations = 0;
    let mut rx_pulses = vec![];
    while let Some((prev, name, pulse)) = to_process.pop_front() {
        
        iterations += 1;
        if name == "rx" {
            rx_pulses.push((iterations, pulse));
            continue;
        }
        
        let module = modules.get_mut(&name).unwrap();

        /*match pulse {
            Pulse::High => {
                high += 1;
                //println!("{prev} -high-> {name}")
            },
            Pulse::Low => {
                low += 1;
                //println!("{prev} -low-> {name}")
            },
        }*/

        match module {
            Module::Broadcast(dest) => {
                for dest in dest {
                    to_process.push_back((name.clone(),dest.clone(), pulse));
                }
            },
            Module::FlipFlop(on, dest) => {
                match pulse {
                    Pulse::High => (),
                    Pulse::Low => {
                        let pulse = if *on {
                            Pulse::Low
                        } else {
                            Pulse::High
                        };
                        *on = !(*on);

                        for dest in dest {
                            to_process.push_back((name.clone(), dest.clone(), pulse));
                        }
                    },
                }
            },
            Module::Conjunction(inputs, dest) => {
                let inp = inputs.get_mut(&prev).unwrap();
                *inp = pulse;

                let mut pulse = Pulse::Low;
                
                for state in inputs.values() {
                    if *state == Pulse::Low {
                        pulse = Pulse::High;
                        break;
                    }
                }

                for dest in dest {
                    to_process.push_back((name.clone(), dest.clone(), pulse));
                }
            },
            Module::Untyped => (),
        }
    }
    
    rx_pulses
}


/*#[derive(Clone, Debug)]
pub enum Module {
    Broadcast(Vec<String>),
    FlipFlop(bool, Vec<String>),
    Conjunction(HashMap<String, Pulse>, Vec<String>),
    Untyped,
}*/

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum HashableModule {
    Broadcast,
    FlipFlop(bool),
    Conjunction(Vec<Pulse>),
    Untyped,
}

impl From<Module> for HashableModule {
    fn from(value: Module) -> Self {
        match value {
            Module::Broadcast(_) => HashableModule::Broadcast,
            Module::FlipFlop(state, _) => HashableModule::FlipFlop(state),
            Module::Conjunction(state, _) => {
                HashableModule::Conjunction(state.into_values().collect())
            },
            Module::Untyped => HashableModule::Untyped,
        }
    }
}