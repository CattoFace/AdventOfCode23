mod day18b;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    // let mut ans = 0;
    // for _ in 0..10000 {
    //     ans = day18b::solve_day()
    // }
    // dbg!(&ans);
    println!("{}", day18b::solve_day());
}
