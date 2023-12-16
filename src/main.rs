#![feature(float_next_up_down)]
mod day16a;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    let mut ans = 0u32;
    for _ in 0..100000 {
        ans = day16a::solve_day()
    }
    dbg!(&ans);
    // println!("{}", day16a::solve_day());
}
