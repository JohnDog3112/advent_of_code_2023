use std::collections::HashMap;

use crate::{Workflow, Condition, Comparison};

pub fn solution(workflows: &HashMap<String, Workflow>) -> usize {

    let mut to_process = vec![];
    to_process.push(("in".to_string(), PartRange::new()));

    let mut valid_ranges = vec![];
    'outer: while let Some((id, range)) = to_process.pop() {
        if id == "A" {
            valid_ranges.push(range);
            continue;
        } else if id == "R" {
            continue;
        }
        
        let workflow = workflows.get(&id).unwrap();

        let (valid, invalid) = range.try_reduce(&workflow.conditions[0]);

        if let Some(valid) = valid {
            to_process.push((workflow.conditions[0].destination.clone(), valid));
        }

        let mut last_invalid = {
            if let Some(invalid) = invalid {
                invalid
            } else {
                continue;
            }
        };

        for condition in workflow.conditions.iter().skip(1) {
            let (valid, invalid) = last_invalid.try_reduce(condition);

            if let Some(valid) = valid {
                to_process.push((condition.destination.clone(), valid));
            }
            if let Some(invalid) = invalid {
                last_invalid = invalid;
            } else {
                continue 'outer;
            }
        }

        to_process.push((workflow.final_dest.clone(), last_invalid));

    }

    //println!("{:?}", valid_ranges);

    valid_ranges.into_iter().fold(0, |prev, next| {
        prev + next.ranges.into_iter().fold(1, |prev, next| {
            prev * (next.max - next.min + 1)
        })
    })
}

#[derive(Copy, Clone, Debug)]
struct PartRange {
    ranges: [Range; 4]
}
impl PartRange {
    pub fn new() -> Self {
        Self {
            ranges: [Range::new(); 4]
        }
    }

    pub fn try_reduce(self, cond: &Condition) -> (Option<Self>, Option<Self>) {
        match cond.comparison {
            Comparison::LessThan => self.reduce_less(cond),
            Comparison::GreaterThan => self.reduce_greater(cond),
        }
    }

    fn reduce_less(self, cond: &Condition) -> (Option<Self>, Option<Self>) {
        let range = self.ranges[cond.property as usize];
        
        let valid;
        if range.min < cond.val && cond.val != 1 {
            let mut out = self;
            out.ranges[cond.property as usize] = Range {
                min: range.min,
                max: cond.val-1,
            };
            valid = Some(out);
        } else {
            valid = None;
        }

        let invalid;
        if range.max >= cond.val {
            let mut out = self;
            out.ranges[cond.property as usize] = Range {
                min: cond.val,
                max: range.max,
            };
            invalid = Some(out);
        } else {
            invalid = None;
        }
        (valid, invalid)
    }

    fn reduce_greater(self, cond: &Condition) -> (Option<Self>, Option<Self>) {
        let range = self.ranges[cond.property as usize];
        
        let valid;
        if range.max > cond.val {
            let mut out = self;
            out.ranges[cond.property as usize] = Range {
                min: cond.val+1,
                max: range.max,
            };
            valid = Some(out);
        } else {
            valid = None;
        }

        let invalid;
        if range.min <= cond.val {
            let mut out = self;
            out.ranges[cond.property as usize] = Range {
                min: range.min,
                max: cond.val,
            };
            invalid = Some(out);
        } else {
            invalid = None;
        }

        (valid, invalid)
    }
}

#[derive(Copy, Clone, Debug)]
struct Range {
    min: usize,
    max: usize, 
}
impl Range {
    pub fn new() -> Self {
        Self {
            min: 1,
            max: 4000,
        }
    }
}