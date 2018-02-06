use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use rand::Rng;

extern crate rand;

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();

    for i in 0..10 {
        let tx = tx.clone();

        thread::spawn(move || {
            println!("spawned {}", i);
            while true {
                thread::sleep(Duration::new(rand::thread_rng().gen_range(0, 10), 0));
                let answer = i * i;
                match tx.send(answer) {
                    Ok(_) => println!("send ok"),
                    Err(e) => println!("send failed: {}", e),
                }
            }
        });
    }
    println!("started receiving");
    thread::spawn(move || {
        while true {
            println!("Writing {}", rx.recv().unwrap());
        }
    });

    thread::sleep(Duration::new(900, 0))
}