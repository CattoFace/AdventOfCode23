use std::fs::read_to_string;

#[derive(Debug)]
struct Edge {
    dest: u16,
    cost: u16,
}
#[derive(Copy, Clone, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug)]
struct Node {
    neighbours: [Edge; 4],
}
impl Node {
    fn new() -> Node {
        Node {
            neighbours: [
                Edge { dest: 0, cost: 0 },
                Edge { dest: 0, cost: 0 },
                Edge { dest: 0, cost: 0 },
                Edge { dest: 0, cost: 0 },
            ],
        }
    }
    fn neigh(&mut self, dir: Dir) -> &mut Edge {
        &mut self.neighbours[dir as usize]
    }
}
fn opposite_dir(dir: Dir) -> Dir {
    use Dir::*;
    match dir {
        Up => Down,
        Down => Up,
        Left => Right,
        Right => Left,
    }
}
fn build_graph(text: String) -> Vec<Node> {
    use Dir::*;
    let mut text = text.into_bytes();
    text[1] = b'#';
    let width = (text.iter().position(|&c| c == b'\n').unwrap() + 1) as u16;
    let mut queue = vec![width + 1];
    let mut graph = Vec::<Node>::new();
    graph.resize_with(text.len(), Node::new);
    while let Some(coord) = queue.pop() {
        if coord == text.len() as u16 - 3 {
            continue;
        }
        if graph[coord as usize].neigh(Left).dest == 0 && text[(coord - 1) as usize] != b'#' {
            let (edge, incoming_dir) = explore_edge(&text, coord, width, Left);
            let other = edge.dest;
            let rev_edge = Edge {
                dest: coord,
                cost: edge.cost,
            };
            *graph[coord as usize].neigh(Left) = edge;
            *graph[other as usize].neigh(incoming_dir) = rev_edge;
            queue.push(other);
        }
        if graph[coord as usize].neigh(Right).dest == 0 && text[(coord + 1) as usize] != b'#' {
            let (edge, incoming_dir) = explore_edge(&text, coord, width, Right);
            let other = edge.dest;
            let rev_edge = Edge {
                dest: coord,
                cost: edge.cost,
            };
            *graph[coord as usize].neigh(Right) = edge;
            *graph[other as usize].neigh(incoming_dir) = rev_edge;
            queue.push(other);
        }
        if graph[coord as usize].neigh(Up).dest == 0 && text[(coord - width) as usize] != b'#' {
            let (edge, incoming_dir) = explore_edge(&text, coord, width, Up);
            let other = edge.dest;
            let rev_edge = Edge {
                dest: coord,
                cost: edge.cost,
            };
            *graph[coord as usize].neigh(Up) = edge;
            *graph[other as usize].neigh(incoming_dir) = rev_edge;
            queue.push(other);
        }
        if graph[coord as usize].neigh(Down).dest == 0 && text[(coord + width) as usize] != b'#' {
            let (edge, incoming_dir) = explore_edge(&text, coord, width, Down);
            let other = edge.dest;
            let rev_edge = Edge {
                dest: coord,
                cost: edge.cost,
            };
            *graph[coord as usize].neigh(Down) = edge;
            *graph[other as usize].neigh(incoming_dir) = rev_edge;
            queue.push(other);
        }
    }
    graph
}
fn explore_edge(text: &[u8], start_coord: u16, width: u16, mut dir: Dir) -> (Edge, Dir) {
    use Dir::*;
    let mut coord = match dir {
        Up => start_coord - 2 * width,
        Down => start_coord + 2 * width,
        Left => start_coord - 2,
        Right => start_coord + 2,
    };
    let mut cost = 3u16;
    loop {
        if text[coord as usize] != b'.' {
            let edge = Edge {
                // to go in the same direction again
                dest: match dir {
                    Up => coord - width,
                    Down => coord + width,
                    Left => coord - 1,
                    Right => coord + 1,
                },
                cost,
            };
            return (edge, opposite_dir(dir));
        } else if coord == text.len() as u16 - 3 {
            return (Edge { dest: coord, cost }, opposite_dir(dir));
        }
        cost += 1;
        if !matches!(dir, Right) && text[(coord - 1) as usize] != b'#' {
            coord -= 1;
            dir = Left;
            continue;
        }
        if !matches!(dir, Left) && text[(coord + 1) as usize] != b'#' {
            coord += 1;
            dir = Right;
            continue;
        }
        if !matches!(dir, Down) && text[(coord - width) as usize] != b'#' {
            coord -= width;
            dir = Up;
            continue;
        }
        if !matches!(dir, Up) && text[(coord + width) as usize] != b'#' {
            coord += width;
            dir = Down;
            continue;
        }
    }
}
fn find_longest(graph: Vec<Node>, start: u16, end: u16) -> u16 {
    let mut history = vec![false; graph.len()];
    history[0] = true;
    history[start as usize] = true;
    find_longest_rec(&graph, &mut history, start, end, 0)
}

fn find_longest_rec(
    graph: &[Node],
    history: &mut [bool],
    start: u16,
    end: u16,
    curr_cost: u16,
) -> u16 {
    if start == end {
        return curr_cost;
    }
    let mut longest = 0u16;
    for neigh in &graph[start as usize].neighbours {
        if !history[neigh.dest as usize] {
            history[neigh.dest as usize] = true;
            longest = longest.max(find_longest_rec(
                graph,
                history,
                neigh.dest,
                end,
                curr_cost + neigh.cost,
            ));
            history[neigh.dest as usize] = false;
        }
    }
    longest
}
#[allow(dead_code)]
fn print_hist(history: &[u16], width: u16) {
    print!("[");
    history.iter().for_each(|&coord| {
        print!("({},{}),", coord / width, coord % width);
    });
    println!("]");
}
pub fn solve_day() -> u16 {
    solve_file(read_to_string("inputs/day23.txt").unwrap())
}
fn solve_file(text: String) -> u16 {
    let width = 1 + text.bytes().position(|c| c == b'\n').unwrap() as u16;
    let start = width + 1;
    let end = text.len() as u16 - 3;
    let graph = build_graph(text);
    // print_graph(&graph, width);
    find_longest(graph, start, end)
}
#[allow(dead_code)]
fn print_graph(graph: &[Node], width: u16) {
    graph.iter().enumerate().for_each(|(idx, node)| {
        if node.neighbours.iter().any(|neigh| neigh.dest != 0) {
            println!("{},({},{}):", idx, idx as u16 / width, idx as u16 % width);
        }
        node.neighbours.iter().for_each(|neigh| {
            if neigh.dest != 0 {
                println!(
                    "   ({},{}):{},",
                    neigh.dest / width,
                    neigh.dest % width,
                    neigh.cost
                );
            }
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day23_test.txt").unwrap()),
            154
        );
        assert_eq!(
            solve_file(read_to_string("inputs/day23.txt").unwrap()),
            6470
        );
    }
}
