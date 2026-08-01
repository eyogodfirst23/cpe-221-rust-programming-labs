mod geometry;
mod utils;

use geometry::shapes::Polygon;
use geometry::Point;

fn main() {
    let a = Point::new(0.0, 0.0);
    let b = Point::new(3.0, 4.0);
    println!("Distance a-b: {:.2}", a.distance(&b));

    let square = Polygon {
        vertices: vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(0.0, 1.0),
        ],
    };

    println!("Perimeter: {:.2}", square.perimeter());
    println!("Is closed: {}", square.is_closed());

    let line = Polygon {
        vertices: vec![Point::new(0.0, 0.0), Point::new(1.0, 1.0)],
    };
    println!("Line is closed: {}", line.is_closed());

    println!("Rounded: {}", utils::round2(3.14159));
}
