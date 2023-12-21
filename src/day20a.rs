use std::{collections::VecDeque, fs::read_to_string};
#[derive(Debug)]
enum ModuleState {
    FlipFlop(FlipFlopState),
    Conjunction(ConjunctionState),
    Broadcast,
}
#[derive(Debug)]
struct Module {
    index: u16,
    outputs: Vec<u16>,
    module_state: ModuleState,
}
impl Module {
    fn new() -> Module {
        Module {
            index: 0,
            outputs: Vec::new(),
            module_state: ModuleState::Broadcast,
        }
    }
}
#[derive(Debug)]
struct FlipFlopState {
    state: bool,
}
impl FlipFlopState {
    fn new() -> FlipFlopState {
        FlipFlopState { state: false }
    }
}
#[derive(Debug)]
struct ConjunctionState {
    state: Vec<bool>,
    inputs: Vec<u16>, //mapping from module index to state index
}
impl ConjunctionState {
    fn new() -> ConjunctionState {
        ConjunctionState {
            state: Vec::new(),
            inputs: Vec::new(),
        }
    }
}

fn parse_modules(mut text: &[u8]) -> Vec<Module> {
    const SIZE: u16 = 32u16;
    let mut modules = Vec::<Module>::with_capacity((SIZE * SIZE) as usize);
    loop {
        if text.is_empty() {
            break;
        }
        let (module_state, module_index) = match text[0] {
            b'b' => {
                text = &text[13..];
                (ModuleState::Broadcast, SIZE * SIZE)
            }
            b'%' => {
                let i = (text[1] - b'a') as u16 * SIZE + (text[2] - b'a') as u16;
                text = &text[5..];
                (ModuleState::FlipFlop(FlipFlopState::new()), i)
            }
            b'&' => {
                let i = (text[1] - b'a') as u16 * SIZE + (text[2] - b'a') as u16;
                text = &text[5..];
                (ModuleState::Conjunction(ConjunctionState::new()), i)
            }
            _ => unreachable!("Invalid module type"),
        };
        if (modules.len() as u16) < module_index + 1 {
            modules.resize_with(module_index as usize + 1, Module::new)
        }
        modules[module_index as usize].index = module_index;
        modules[module_index as usize].module_state = module_state;
        while text[0] != b'\n' {
            let output_index = (text[2] - b'a') as u16 * SIZE + (text[3] - b'a') as u16;
            text = &text[4..];
            modules[module_index as usize].outputs.push(output_index);
        }
        text = &text[1..];
    }
    (0..modules.len()).for_each(|i| {
        let module_index = modules[i].index;
        let outputs = modules[i].outputs.clone();
        for &output in outputs.iter() {
            if let ModuleState::Conjunction(conj) = &mut modules[output as usize].module_state {
                conj.state.push(false);
                conj.inputs.push(module_index);
            }
        }
    });
    modules
}
fn signal_broadcast(mut modules: Vec<Module>, press_count: u16) -> u32 {
    let mut signals = [0u32, 0u32]; //low and high signals
    for _ in 0..press_count {
        let mut signal_queue: VecDeque<(bool, u16, u16)> = VecDeque::new();
        signal_queue.push_back((false, 32 * 32, 32 * 32));
        while !signal_queue.is_empty() {
            let (signal, source, target) = signal_queue.pop_front().unwrap();
            // println!("{} -{} => {}", source, signal, target);
            signals[signal as usize] += 1;
            let module = &mut modules[target as usize];
            let next_signal = match &mut module.module_state {
                ModuleState::FlipFlop(state) => {
                    if !signal {
                        state.state = !state.state
                    }
                    state.state
                }
                ModuleState::Conjunction(state) => {
                    state.state[state.inputs.iter().position(|&idx| idx == source).unwrap()] =
                        signal;
                    !state.state.iter().all(|&s| s)
                }
                ModuleState::Broadcast => false,
            };
            // if flipflop wasnt switched, dont signal
            if (matches!(module.module_state, ModuleState::FlipFlop(_)) && signal) {
                continue;
            }
            for output in &module.outputs {
                signal_queue.push_back((next_signal, target, *output));
            }
        }
    }
    signals[0] * signals[1]
}
pub fn solve_day() -> u32 {
    solve_file(read_to_string("inputs/day20.txt").unwrap())
}
fn solve_file(text: String) -> u32 {
    let text = text.as_bytes();
    let modules = parse_modules(text);
    // for module in modules {
    //     if !module.outputs.is_empty() {
    //         dbg!(module);
    //     }
    // }
    signal_broadcast(modules, 1000)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day20_test.txt").unwrap()),
            32000000
        );
        assert_eq!(
            solve_file(read_to_string("inputs/day20.txt").unwrap()),
            832957356
        );
    }
}
