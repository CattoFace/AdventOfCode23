#![feature(float_next_up_down)]
mod day12b;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    // for _i in 0..10000 {
    // day4b::solve_day();
    // }
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    // let mut ans = 0u64;
    // for _ in 0..100 {
    // ans = day12b::solve_day()
    // }
    println!("{}", day12b::solve_day());
}
