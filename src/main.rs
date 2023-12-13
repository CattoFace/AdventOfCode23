#![feature(float_next_up_down)]
mod day13b;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    // let mut ans = 0u64;
    // for _ in 0..100 {
    // ans = day13b::solve_day()
    // }
    println!("{}", day13b::solve_day());
}
