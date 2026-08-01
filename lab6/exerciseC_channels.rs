use std::sync::mpsc;
use std::thread;

#[derive(Debug)]
enum WorkResult {
    Sum(u64),
    Error(String),
}

fn worker(id: usize, chunk: Vec<u64>, tx_clone: mpsc::Sender<WorkResult>) {
    let sum: u64 = chunk.iter().sum();
    println!("Worker {} computed sum = {}", id, sum);

    if sum > 30000 {
        tx_clone
            .send(WorkResult::Error(format!(
                "Worker {} sum {} exceeded 30000",
                id, sum
            )))
            .unwrap();
    } else {
        tx_clone.send(WorkResult::Sum(sum)).unwrap();
    }
}

fn main() {
    let (tx, rx) = mpsc::channel::<WorkResult>();
    let dataset: Vec<u64> = (0..4)
        .flat_map(|i| (i * 250 + 1..=(i + 1) * 250).collect::<Vec<u64>>())
        .collect();

    for (id, chunk) in dataset.chunks(250).enumerate() {
        let tx_clone = tx.clone();
        let chunk = chunk.to_vec();
        thread::spawn(move || worker(id, chunk, tx_clone));
    }
    drop(tx);

    let mut total: u64 = 0;
    for result in rx.iter() {
        match result {
            WorkResult::Sum(s) => total += s,
            WorkResult::Error(msg) => println!("Error: {}", msg),
        }
    }
    println!("Grand total: {}", total);
}
