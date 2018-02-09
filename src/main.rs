use std::thread;
use std::sync::mpsc;
use rand::{random, Closed01};
use time::PreciseTime;

extern crate time;
extern crate rand;

fn main() {
    let n = 10000000;

    let par_start = PreciseTime::now();
    let par_res = euler_maker_par(n);
    let par_end = PreciseTime::now();

    let start = PreciseTime::now();
    let res = euler_maker(n);
    let end = PreciseTime::now();

    println!("{} seconds for parallel e {}", par_start.to(par_end), par_res);
    println!("{} seconds for single thread e {}", start.to(end), res);
}

fn euler_maker(sample_size: u64) -> f64 {
    let mut results: u64 = 0;
    for _i in 0..sample_size {
        results += random_to_1()
    }
    (results as f64) / (sample_size as f64)
}

fn euler_maker_par(sample_size: u64) -> f64 {
    let (tx, rx) = mpsc::channel::<u64>();
    let n_threads = 4;
    let sample_frac = sample_size / n_threads;
    for _i in 0..n_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            for _j in 0..sample_frac {
                let answer = random_to_1();
                match tx.send(answer) {
                    Err(_) => {
                        return;
                    }
                    Ok(_) => (),
                }
            }
        });
    }
    drop(tx);
    let sum = rx.iter().sum::<u64>();
    (sum as f64) / (sample_size as f64)
}

fn random_to_1() -> u64 {
    let mut x = 0.0;
    let mut n = 0;
    while x < 1.0 {
        let Closed01(r) = random::<Closed01<f64>>();
        x = x + r;
        n += 1
    }
    n
}