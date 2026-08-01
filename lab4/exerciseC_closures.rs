fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}

fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

fn main() {
    let double = |x| x * 2;
    println!("apply_twice result: {}", apply_twice(double, 3));

    let add10 = make_adder(10);
    println!("add10(5): {}", add10(5));

    let result: Vec<String> = (1..=20)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .take(5)
        .map(|x| format!("{}", x))
        .collect();
    println!("First 5 even squares: {}", result.join(", "));

    let sum_odd_squares: u64 = (1..=99)
        .filter(|x| x % 2 != 0)
        .map(|x| (x as u64) * (x as u64))
        .fold(0, |acc, x| acc + x);
    println!("Sum of squares of odd 1..=99: {}", sum_odd_squares);

    let primes: Vec<u32> = (2..=50)
        .filter(|&n| (2..n).all(|d| n % d != 0))
        .collect();
    println!("Primes up to 50: {:?}", primes);
}
