use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("[thread] count = {}", i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    for i in 1..=3 {
        println!("[main] count = {}", i);
        thread::sleep(Duration::from_millis(80));
    }

    handle.join().expect("Thread panicked");
    println!("All done");
}
