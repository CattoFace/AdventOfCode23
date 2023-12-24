mod day24b;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    // let mut ans = 1;
    // for _ in 0..1000 {
    //     ans = day24b::solve_day()
    // }
    // dbg!(&ans);
    println!("{}", day24b::solve_day());
}
