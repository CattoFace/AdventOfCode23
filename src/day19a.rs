use std::{fs::read_to_string, iter, str};

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
type Gear19 = [u16; 4]; // x,m,a,s in order
#[derive(Debug)]
struct WFGate {
    threshold: u16,
    gate_category: GateCategory,
    gate_type: GateType,
    result_index: u16,
}
impl WFGate {
    fn check(&self, gear: &Gear19) -> Option<u16> {
        match self.gate_type {
            GateType::LessThan => {
                if gear[self.gate_category as usize] < self.threshold {
                    Some(self.result_index)
                } else {
                    None
                }
            }
            GateType::GreaterThan => {
                if gear[self.gate_category as usize] > self.threshold {
                    Some(self.result_index)
                } else {
                    None
                }
            }
            GateType::FallBack => Some(self.result_index),
        }
    }
}
#[derive(Debug)]
struct WorkFlow {
    gates: Vec<WFGate>,
}
impl WorkFlow {
    fn pass_gear(&self, gear: &Gear19) -> u16 {
        self.gates.iter().find_map(|gate| gate.check(gear)).unwrap()
    }
}

fn create_workflows(workflow_str: &[u8]) -> Vec<Option<WorkFlow>> {
    let mut map: Vec<Option<WorkFlow>> = Vec::new();
    map.resize_with(32 * 32 * 32, || None);
    workflow_str.split(|&c| c == b'\n').for_each(|line| {
        let name_end = line.iter().position(|&c| c == b'{').unwrap();
        let name = &line[..name_end];
        let index = to_index(name) as usize;
        let gates_str = &line[name_end + 1..line.len() - 1];
        let mut gates: Vec<WFGate> = Vec::with_capacity(5);
        gates_str.split(|&c| c == b',').for_each(|gate_str| {
            // fallback gate
            if gate_str.len() == 1 || (gate_str[1] != b'>' && gate_str[1] != b'<') {
                gates.push(WFGate {
                    threshold: 0,
                    gate_category: GateCategory::X,
                    gate_type: GateType::FallBack,
                    result_index: to_index(gate_str),
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
                result_index: to_index(&gate_str[3 + num_end..]),
            });
        });
        map[index] = Some(WorkFlow { gates });
    });
    map
}

fn create_gears(mut gears_str: &[u8]) -> Vec<Gear19> {
    iter::from_fn(|| {
        if gears_str.is_empty() {
            return None;
        }
        let mut values = [0u16; 4];
        // x
        gears_str = &gears_str[3..];
        let num_end = gears_str.iter().position(|&c| c == b',').unwrap();
        values[0] = unsafe {
            str::from_utf8_unchecked(&gears_str[..num_end])
                .parse()
                .unwrap()
        };
        // m
        gears_str = &gears_str[num_end + 3..];
        let num_end = gears_str.iter().position(|&c| c == b',').unwrap();
        values[1] = unsafe {
            str::from_utf8_unchecked(&gears_str[..num_end])
                .parse()
                .unwrap()
        };
        // a
        gears_str = &gears_str[num_end + 3..];
        let num_end = gears_str.iter().position(|&c| c == b',').unwrap();
        values[2] = unsafe {
            str::from_utf8_unchecked(&gears_str[..num_end])
                .parse()
                .unwrap()
        };
        // s
        gears_str = &gears_str[num_end + 3..];
        let num_end = gears_str.iter().position(|&c| c == b'}').unwrap();
        values[3] = unsafe {
            str::from_utf8_unchecked(&gears_str[..num_end])
                .parse()
                .unwrap()
        };
        gears_str = &gears_str[num_end + 2..];
        Some(values)
    })
    .collect()
}
fn to_index(name: &[u8]) -> u16 {
    if name[0] == b'A' {
        return 1; // a - a+1
    } else if name[0] == b'R' {
        return (b'r' - b'a' + 1) as u16;
    }
    name.iter()
        .fold(0u16, |acc, &c| acc * 32 + (c - b'a' + 1) as u16)
}
fn analyze_gears(gears: &[Gear19], workflows: Vec<Option<WorkFlow>>) -> u32 {
    let accept_str: &[u8] = b"A";
    let reject_str: &[u8] = b"R";
    let in_str: &[u8] = b"in";
    let accept = to_index(accept_str);
    let reject = to_index(reject_str);
    let in_workflow = &workflows[to_index(in_str) as usize];
    gears
        .iter()
        .filter_map(|gear| {
            let mut workflow = in_workflow;
            loop {
                let next_workflow = workflow.as_ref().unwrap().pass_gear(gear);
                match next_workflow {
                    _ if next_workflow == accept => {
                        return Some(
                            gear[0] as u32 + gear[1] as u32 + gear[2] as u32 + gear[3] as u32,
                        )
                    }
                    _ if next_workflow == reject => return None,
                    _ => {}
                }
                workflow = &workflows[next_workflow as usize];
            }
        })
        .sum()
}
pub fn solve_day() -> u32 {
    solve_file(read_to_string("inputs/day19.txt").unwrap())
}
fn solve_file(text: String) -> u32 {
    let text = text.as_bytes();
    let workflows_end = text
        .iter()
        .tuple_windows()
        .position(|(&a, &b)| a == b'\n' && b == b'\n')
        .unwrap();
    let (workflow_str, mut gears_str) = text.split_at(workflows_end);
    gears_str = &gears_str[2..]; // skip the double \n
    let workflows = create_workflows(workflow_str);
    // dbg!(&workflows);
    let gears = create_gears(gears_str);
    // dbg!(&gears);
    analyze_gears(&gears, workflows)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day19_test.txt").unwrap()),
            19114
        );
        assert_eq!(
            solve_file(read_to_string("inputs/day19.txt").unwrap()),
            395382
        );
    }
}
