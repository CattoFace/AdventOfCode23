mod day20b;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
<<<<<<< HEAD
    // let mut ans = 0;
    // for _ in 0..10000 {
    // ans = day19a::solve_day()
    // }
    // dbg!(&ans);
    println!("{}", day19a::solve_day());
=======
    // let mut ans = 1;
    // for _ in 0..10000 {
    //     ans = day20b::solve_day()
    // }
    // dbg!(&ans);
    println!("{}", day20b::solve_day());
>>>>>>> 0332c5b (day 20)
}
