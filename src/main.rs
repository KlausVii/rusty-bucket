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

fn euler_maker(sample_size: i32) -> f64 {
    let mut results = Vec::<i64>::new();
    for i in 0..sample_size {
        results.push(random_to_1())
    }
    (results.iter().sum::<i64>() as f64) / (results.len() as f64)
}

fn euler_maker_par(sample_size: i32) -> f64 {
    let (tx, rx) = mpsc::channel::<i64>();
    for i in 0..4 {
        let tx = tx.clone();
        thread::spawn(move || {
            while true {
                let answer = random_to_1();
                match tx.send(answer) {
                    Err(e) => {
                        return;
                    },
                    Ok(_) => (),
                }
            }
        });
    }
    let mut results = Vec::<i64>::new();
    while results.len() < sample_size as usize {
        results.push(rx.recv().unwrap());
    }
   (results.iter().sum::<i64>() as f64) / (results.len() as f64)
}

fn random_to_1() -> i64 {
    let mut x = 0.0;
    let mut n = 0;
    while x < 1.0 {
        let r = random::<Closed01<f64>>().0;
        x = x + r;
        n += 1
    }
    n
}