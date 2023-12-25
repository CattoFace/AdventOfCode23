use aoc23::{
    day10a, day10b, day11a, day11b, day12a, day12b, day13a, day13b, day14a, day14b, day15a, day15b,
    day16a, day16b, day17a, day17b, day18a, day18b, day19a, day19b, day1a, day1b, day20a, day20b,
    day21a, day21b, day22a, day22b, day23a, day23b, day24a, day24b, day2a, day2b, day3a, day3b,
    day4a, day4b, day5a, day5b, day6a, day6b, day7a, day7b, day8a, day8b, day9a, day9b,
};

mod day25a;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    // let mut ans = 1;
    // for _ in 0..1000 {
    //     ans = day25a::solve_day()
    // }
    // dbg!(&ans);
    println!("{}", day1a::solve_day());
    println!("{}", day1b::solve_day());
    println!("{}", day2a::solve_day());
    println!("{}", day2b::solve_day());
    println!("{}", day3a::solve_day());
    println!("{}", day3b::solve_day());
    println!("{}", day4a::solve_day());
    println!("{}", day4b::solve_day());
    println!("{}", day5a::solve_day());
    println!("{}", day5b::solve_day());
    println!("{}", day6a::solve_day());
    println!("{}", day6b::solve_day());
    println!("{}", day7a::solve_day());
    println!("{}", day7b::solve_day());
    println!("{}", day8a::solve_day());
    println!("{}", day8b::solve_day());
    println!("{}", day9a::solve_day());
    println!("{}", day9b::solve_day());
    println!("{}", day10a::solve_day());
    println!("{}", day10b::solve_day());
    println!("{}", day11a::solve_day());
    println!("{}", day11b::solve_day());
    println!("{}", day12a::solve_day());
    println!("{}", day12b::solve_day());
    println!("{}", day13a::solve_day());
    println!("{}", day13b::solve_day());
    println!("{}", day14a::solve_day());
    println!("{}", day14b::solve_day());
    println!("{}", day15a::solve_day());
    println!("{}", day15b::solve_day());
    println!("{}", day16a::solve_day());
    println!("{}", day16b::solve_day());
    println!("{}", day17a::solve_day());
    println!("{}", day17b::solve_day());
    println!("{}", day18a::solve_day());
    println!("{}", day18b::solve_day());
    println!("{}", day19a::solve_day());
    println!("{}", day19b::solve_day());
    println!("{}", day20a::solve_day());
    println!("{}", day20b::solve_day());
    println!("{}", day21a::solve_day());
    println!("{}", day21b::solve_day());
    println!("{}", day22a::solve_day());
    println!("{}", day22b::solve_day());
    println!("{}", day23a::solve_day());
    println!("{}", day23b::solve_day());
    println!("{}", day24a::solve_day());
    println!("{}", day24b::solve_day());
    println!("{}", day25a::solve_day());
}
