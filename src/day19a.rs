use std::{collections::HashMap, fs::read_to_string, iter, str};

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
struct WFGate<'a> {
    threshold: u16,
    gate_category: GateCategory,
    gate_type: GateType,
    result: &'a [u8],
}
impl<'a> WFGate<'a> {
    fn check(&self, gear: &Gear19) -> Option<&'a [u8]> {
        match self.gate_type {
            GateType::LessThan => {
                if gear[self.gate_category as usize] < self.threshold {
                    Some(self.result)
                } else {
                    None
                }
            }
            GateType::GreaterThan => {
                if gear[self.gate_category as usize] > self.threshold {
                    Some(self.result)
                } else {
                    None
                }
            }
            GateType::FallBack => Some(self.result),
        }
    }
}
#[derive(Debug)]
struct WorkFlow<'a> {
    gates: Vec<WFGate<'a>>,
}
impl<'a> WorkFlow<'a> {
    fn pass_gear(&self, gear: &Gear19) -> &'a [u8] {
        self.gates.iter().find_map(|gate| gate.check(gear)).unwrap()
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
fn analyze_gears(gears: &[Gear19], workflows: HashMap<&[u8], WorkFlow>) -> u32 {
    let in_str: &[u8] = b"in";
    let in_workflow = workflows.get(in_str).unwrap();
    gears
        .iter()
        .filter_map(|gear| {
            let mut workflow = in_workflow;
            loop {
                let next_workflow_name = workflow.pass_gear(gear);
                match next_workflow_name[0] {
                    b'A' => {
                        return Some(
                            gear[0] as u32 + gear[1] as u32 + gear[2] as u32 + gear[3] as u32,
                        )
                    }
                    b'R' => return None,
                    _ => {}
                }
                workflow = workflows.get(next_workflow_name).unwrap();
            }
        })
        .sum()
}
pub fn solve_day() -> u32 {
    solve_file(read_to_string("inputs/day19.txt").unwrap())
}
fn solve_file(text: String) -> u32 {
    let text = text.as_bytes();
    let width = text.iter().position(|&c| c == b'\n').unwrap() + 1;
    let workflows_end = text
        .iter()
        .tuple_windows()
        .position(|(&a, &b)| a == b'\n' && b == b'\n')
        .unwrap();
    let (workflow_str, mut gears_str) = text.split_at(workflows_end);
    gears_str = &gears_str[2..]; // skip the double \n
    let workflows: HashMap<&[u8], WorkFlow> = create_workflows(workflow_str, workflows_end / width);
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
