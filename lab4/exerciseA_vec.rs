fn stats(data: &[f64]) -> (f64, f64, f64) {
    let sum: f64 = data.iter().sum();
    let mean = sum / data.iter().len() as f64;
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    (mean, min, max)
}

fn main() {
    let mut scores: Vec<f64> = vec![85.0, 92.0, 78.5, 95.0, 60.0, 88.0];
    let (mean, min, max) = stats(&scores);
    println!("Mean={:.2} Min={:.2} Max={:.2}", mean, min, max);

    scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("Sorted: {:?}", scores);

    let high_scores: Vec<f64> = scores.iter().filter(|&&s| s >= 80.0).copied().collect();
    println!("High scores: {:?}", high_scores);

    let n = scores.len();
    let median = if n % 2 == 0 {
        (scores[n / 2 - 1] + scores[n / 2]) / 2.0
    } else {
        scores[n / 2]
    };
    println!("Median: {:.2}", median);

    let variance = scores.iter().map(|s| (s - mean).powi(2)).sum::<f64>() / n as f64;
    let std_dev = variance.sqrt();
    println!("Variance={:.2} StdDev={:.2}", variance, std_dev);
  }
