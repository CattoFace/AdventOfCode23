use std::{collections::HashMap, fs::read_to_string, str};

use itertools::Itertools;
#[derive(Debug)]
enum GateType {
    LessThan,
    GreaterThan,
    FallBack,
}
#[derive(Debug, Copy, Clone)]
enum GateCategory {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}
type GearRange = [std::ops::Range<u16>; 4]; // x,m,a,s in order

fn calc_gear_range(gear: GearRange) -> u64 {
    (gear[0].end as u64 - gear[0].start as u64)
        * (gear[1].end as u64 - gear[1].start as u64)
        * (gear[2].end as u64 - gear[2].start as u64)
        * (gear[3].end as u64 - gear[3].start as u64)
}
#[derive(Debug)]
struct WFGate<'a> {
    threshold: u16,
    gate_category: GateCategory,
    gate_type: GateType,
    result: &'a [u8],
}
impl<'a> WFGate<'a> {
    fn pass_gear(&self, mut gear: GearRange) -> (Option<GearRange>, Option<GearRange>) {
        let index = self.gate_category as usize;
        match self.gate_type {
            GateType::LessThan => {
                if gear[index].start > self.threshold {
                    // no splitting, the whole range fails
                    (Some(gear), None)
                } else if gear[index].end < self.threshold {
                    // no splitting, the whole range passes
                    (None, Some(gear))
                } else {
                    // split, lower part passes, higher part fails
                    let mut lower_gear = gear.clone();
                    lower_gear[index].end = self.threshold;
                    gear[index].start = self.threshold;
                    (Some(lower_gear), Some(gear))
                }
            }
            GateType::GreaterThan => {
                if gear[index].end < self.threshold {
                    // no splitting, the whole range fails
                    (None, Some(gear))
                } else if gear[index].start > self.threshold {
                    // no splitting, the whole range passes
                    (Some(gear), None)
                } else {
                    // split, lower part fails, higher part passes
                    let mut lower_gear = gear.clone();
                    lower_gear[index].end = self.threshold + 1;
                    gear[index].start = self.threshold + 1;
                    (Some(gear), Some(lower_gear))
                }
            }
            // the whole range passes
            GateType::FallBack => (Some(gear), None),
        }
    }
}
#[derive(Debug)]
struct WorkFlow<'a> {
    gates: Vec<WFGate<'a>>,
}
impl<'a> WorkFlow<'a> {
    fn pass_gear(&self, mut gear: GearRange) -> Vec<(GearRange, &[u8])> {
        let mut result: Vec<(GearRange, &[u8])> = Vec::with_capacity(4096);
        for gate in self.gates.iter() {
            let (pass, fail) = gate.pass_gear(gear);
            if let Some(pass) = pass {
                result.push((pass, gate.result));
            }
            if let Some(fail) = fail {
                gear = fail;
            } else {
                return result;
            }
        }
        result
    }
}
fn create_workflows(workflow_str: &[u8], workflow_count: usize) -> HashMap<&[u8], WorkFlow> {
    let mut map: HashMap<&[u8], WorkFlow> = HashMap::with_capacity(workflow_count);
    workflow_str
        .split(|&c| c == b'\n')
        .map(|line| {
            let name_end = line.iter().position(|&c| c == b'{').unwrap();
            let name = &line[..name_end];
            let gates_str = &line[name_end + 1..line.len() - 1];
            let mut gates: Vec<WFGate> = Vec::with_capacity(5);
            gates_str.split(|&c| c == b',').for_each(|gate_str| {
                // fallback gate
                if gate_str.len() == 1 || (gate_str[1] != b'>' && gate_str[1] != b'<') {
                    gates.push(WFGate {
                        threshold: 0,
                        gate_category: GateCategory::X,
                        gate_type: GateType::FallBack,
                        result: gate_str,
                    });
                    return;
                }
                let num_end = gate_str[2..].iter().position(|&c| c == b':').unwrap();
                // came from utf8 originally, so its safe
                let threshold = unsafe {
                    str::from_utf8_unchecked(&gate_str[2..2 + num_end])
                        .parse::<u16>()
                        .unwrap()
                };
                let gate_category = match gate_str[0] {
                    b'x' => GateCategory::X,
                    b'm' => GateCategory::M,
                    b'a' => GateCategory::A,
                    b's' => GateCategory::S,
                    _ => unreachable!("only 4 meterics exist!"),
                };
                let gate_type = match gate_str[1] {
                    b'>' => GateType::GreaterThan,
                    b'<' => GateType::LessThan,
                    _ => unreachable!("Only 2 gate types exist!"),
                };
                gates.push(WFGate {
                    threshold,
                    gate_category,
                    gate_type,
                    result: &gate_str[3 + num_end..],
                });
            });
            (name, WorkFlow { gates })
        })
        .for_each(|(k, v)| {
            map.insert(k, v);
        });
    map
}

pub fn solve_day() -> u64 {
    solve_file(read_to_string("inputs/day19.txt").unwrap())
}
fn solve_file(text: String) -> u64 {
    let text = text.as_bytes();
    let width = text.iter().position(|&c| c == b'\n').unwrap() + 1;
    let cropped_len = text
        .iter()
        .tuple_windows()
        .position(|(&a, &b)| a == b'\n' && b == b'\n')
        .unwrap();
    let workflow_str = &text[..cropped_len];
    let workflows: HashMap<&[u8], WorkFlow> = create_workflows(workflow_str, cropped_len / width);
    // dbg!(&workflows);
    analyze_workflows(workflows)
}

fn analyze_workflows(workflows: HashMap<&[u8], WorkFlow>) -> u64 {
    let gear = [1..4001, 1..4001, 1..4001, 1..4001];
    let in_name: &[u8] = b"in";
    let mut sum = 0u64;
    let mut gear_bucket = vec![(gear, in_name)];
    while let Some((gear, workflow_name)) = gear_bucket.pop() {
        if workflow_name[0] == b'A' {
            sum += calc_gear_range(gear);
            continue;
        } else if workflow_name[0] == b'R' {
            continue;
        }
        let workflow = workflows.get(&workflow_name).unwrap();
        gear_bucket.extend(workflow.pass_gear(gear));
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day19_test.txt").unwrap()),
            167409079868000
        );
        assert_eq!(
            solve_file(read_to_string("inputs/day19.txt").unwrap()),
            103557657654583
        );
    }
}
