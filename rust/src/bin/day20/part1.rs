use std::collections::{HashMap, VecDeque};

use crate::{Module, Pulse};

pub fn solution(mut modules: HashMap<String, Module>) -> usize {
    
    let mut high = 0;
    let mut low = 0;

    for _ in 0..1000 {
        let (l, h) = push_button(&mut modules);
        high += h;
        low += l;

        //println!("--------------------");
    }
    
    high * low
}

pub fn push_button(modules: &mut HashMap<String, Module>) -> (usize, usize) {
    let mut to_process = VecDeque::new();

    to_process.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));

    let mut low = 0;
    let mut high = 0;
    while let Some((prev, name, pulse)) = to_process.pop_front() {
        let module = modules.get_mut(&name).unwrap();

        match pulse {
            Pulse::High => {
                high += 1;
                //println!("{prev} -high-> {name}")
            },
            Pulse::Low => {
                low += 1;
                //println!("{prev} -low-> {name}")
            },
        }

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
    (low, high)
}