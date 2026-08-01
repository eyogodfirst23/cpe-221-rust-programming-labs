use tokio::time::{sleep, Duration};

async fn fetch_data(id: u32) -> String {
    sleep(Duration::from_millis(100)).await;
    format!("Data from source {}", id)
}

#[tokio::main]
async fn main() {
    let t0 = std::time::Instant::now();
    for id in 1..=4 {
        let data = fetch_data(id).await;
        println!("Sequential: {}", data);
    }
    println!("Sequential time: {:?}", t0.elapsed());

    let t1 = std::time::Instant::now();
    let handles: Vec<_> = (1..=4).map(|id| tokio::spawn(fetch_data(id))).collect();
    for h in handles {
        println!("Concurrent: {}", h.await.unwrap());
    }
    println!("Concurrent time: {:?}", t1.elapsed());

    let t2 = std::time::Instant::now();
    let (r1, r2, r3, r4) = tokio::join!(
        fetch_data(1),
        fetch_data(2),
        fetch_data(3),
        fetch_data(4),
    );
    println!("Joined 1: {}", r1);
    println!("Joined 2: {}", r2);
    println!("Joined 3: {}", r3);
    println!("Joined 4: {}", r4);
    println!("join! time: {:?}", t2.elapsed());
      }
