mod day20b;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    // let mut ans = 1;
    // for _ in 0..10000 {
    //     ans = day20b::solve_day()
    // }
    // dbg!(&ans);
    println!("{}", day20b::solve_day());
}
