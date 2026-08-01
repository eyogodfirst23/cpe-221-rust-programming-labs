use super::shapes::Point;

pub fn translate(p: &Point, dx: f64, dy: f64) -> Point {
    Point::new(p.x + dx, p.y + dy)
}

pub fn scale(p: &Point, factor: f64) -> Point {
    Point::new(p.x * factor, p.y * factor)
}
