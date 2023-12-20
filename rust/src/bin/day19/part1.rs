use std::collections::HashMap;

use crate::{Workflow, Part, Comparison};

pub fn solution(workflows: &HashMap<String, Workflow>, parts: &[Part]) -> usize {
    
    parts.iter().filter_map(|part| {
        let mut cur_dest = "in".to_string();
        'outer: while cur_dest != "R" && cur_dest != "A" {
            let conditions = workflows.get(&cur_dest).unwrap();
            
            for condition in &conditions.conditions {

                let val = part.properties[condition.property as usize];

                let mv = match condition.comparison {
                    Comparison::LessThan => val < condition.val,
                    Comparison::GreaterThan => val > condition.val,
                };

                if mv {
                    cur_dest = condition.destination.clone();
                    continue 'outer;
                }
            }
            cur_dest = conditions.final_dest.clone();
        }

        if cur_dest == "A" {
            Some(part.properties.iter().sum::<usize>())
        } else {
            None
        }
    }).sum()
}