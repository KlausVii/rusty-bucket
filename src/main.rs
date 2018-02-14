use time::PreciseTime;
extern crate time;
extern crate euler;


fn main() {
    let n = 1000000;

    let par_start = PreciseTime::now();
    let par_res = euler::euler_maker_par(n);
    let par_end = PreciseTime::now();

    let start = PreciseTime::now();
    let res = euler::euler_maker(n);
    let end = PreciseTime::now();

    println!("{} seconds for parallel e {}", par_start.to(par_end), par_res);
    println!("{} seconds for single thread e {}", start.to(end), res);
}