use std::fmt;

trait Describable {
    fn describe(&self) -> String;
    fn short_name(&self) -> String {
        format!("[{}]", &self.describe()[..20.min(self.describe().len())])
    }
}

trait Area {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Describable for Circle {
    fn describe(&self) -> String {
        format!("Circle with radius {:.2}", self.radius)
    }
}

impl Describable for Rectangle {
    fn describe(&self) -> String {
        format!("Rectangle {:.2} x {:.2}", self.width, self.height)
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle(radius={:.2}, area={:.2})", self.radius, self.area())
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Rectangle({:.2} x {:.2}, area={:.2})",
            self.width,
            self.height,
            self.area()
        )
    }
}

fn print_area(shape: &dyn Area) {
    println!("Area = {:.4}", shape.area());
}

fn main() {
    let c = Circle { radius: 3.0 };
    let r = Rectangle { width: 4.0, height: 5.0 };
    print_area(&c);
    print_area(&r);
    println!("{}", c.describe());
    println!("{}", r.short_name());
    println!("{}", c);
    println!("{}", r);
}