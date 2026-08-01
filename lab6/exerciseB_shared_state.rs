use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn main() {
    let counter = Arc::new(Mutex::new(0u64));
    let mut handles = vec![];

    let t0 = Instant::now();
    for _ in 0..8 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1_000 {
                let mut num = c.lock().unwrap();
                *num += 1;
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("[naive] Final counter: {}", *counter.lock().unwrap());
    println!("[naive] time: {:?}", t0.elapsed());

    let counter2 = Arc::new(Mutex::new(0u64));
    let mut handles2 = vec![];

    let t1 = Instant::now();
    for _ in 0..8 {
        let c = Arc::clone(&counter2);
        handles2.push(thread::spawn(move || {
            let mut local_sum: u64 = 0;
            for _ in 0..1_000 {
                local_sum += 1;
            }
            let mut num = c.lock().unwrap();
            *num += local_sum;
        }));
    }
    for h in handles2 {
        h.join().unwrap();
    }
    println!("[refactored] Final counter: {}", *counter2.lock().unwrap());
    println!("[refactored] time: {:?}", t1.elapsed());

    let quarters: [(u64, u64); 4] = [(1, 250), (251, 500), (501, 750), (751, 1000)];
    let mut sum_handles = vec![];
    for (start, end) in quarters {
        sum_handles.push(thread::spawn(move || -> u64 { (start..=end).sum() }));
    }
    let total: u64 = sum_handles.into_iter().map(|h| h.join().unwrap()).sum();
    println!("Total sum of 1..=1000 (via 4 threads) = {}", total);
          }
