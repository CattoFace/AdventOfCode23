#![feature(float_next_up_down)]
mod day14b;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    let mut ans = 0u32;
    for _ in 0..1000 {
        ans = day14b::solve_day()
    }
    dbg!(&ans);
    // println!("{}", day14b::solve_day());
}
