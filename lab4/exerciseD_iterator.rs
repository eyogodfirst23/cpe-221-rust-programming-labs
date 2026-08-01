struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let next = self.a;
        self.a = self.b;
        self.b = next + self.b;
        Some(self.a)
    }
}

struct Primes {
    found: Vec<u64>,
    candidate: u64,
}

impl Primes {
    fn new() -> Self {
        Primes {
            found: Vec::new(),
            candidate: 2,
        }
    }
}

impl Iterator for Primes {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        loop {
            let n = self.candidate;
            self.candidate += 1;
            let is_prime = self
                .found
                .iter()
                .take_while(|&&p| p * p <= n)
                .all(|&p| n % p != 0);
            if is_prime {
                self.found.push(n);
                return Some(n);
            }
        }
    }
}

fn main() {
    let fibs: Vec<u64> = Fibonacci::new().take(15).collect();
    println!("First 15 Fibonacci: {:?}", fibs);

    let big_fib = Fibonacci::new().find(|&x| x > 1_000_000).unwrap();
    println!("First Fibonacci > 1,000,000: {}", big_fib);

    let primes: Vec<u64> = Primes::new().take(15).collect();
    println!("First 15 primes: {:?}", primes);
}
