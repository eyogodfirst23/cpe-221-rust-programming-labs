use std::fmt::Display;

fn print_largest<T: PartialOrd + Display>(list: &[T]) {
    if list.is_empty() {
        return;
    }
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    println!("The largest is {}", largest);
}

#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Pair { first, second }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.first >= self.second {
            println!("First is larger: {}", self.first);
        } else {
            println!("Second is larger: {}", self.second);
        }
    }
}

fn zip_with<A, B, C, F>(a: &[A], b: &[B], f: F) -> Vec<C>
where
    F: Fn(&A, &B) -> C,
{
    a.iter().zip(b.iter()).map(|(x, y)| f(x, y)).collect()
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    print_largest(&numbers);

    let p = Pair::new(5, 10);
    p.cmp_display();

    let fruits = vec!["mango", "apple", "banana"];
    let counts = vec![3, 5, 2];
    let combined = zip_with(&fruits, &counts, |f, c| format!("{}: {}", f, c));
    println!("{:?}", combined);
}
