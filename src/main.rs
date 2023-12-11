#![feature(float_next_up_down)]
mod day10b;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    // for _i in 0..10000 {
    // day4b::solve_day();
    // }
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    println!("{}", day10b::solve_day());
}
