pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
pub fn translate(&self, dx: f64, dy: f64) -> Point {
        Point { x: self.x + dx, y: self.y + dy }
    }

    pub fn scale(&self, factor: f64) -> Point {
        Point { x: self.x * factor, y: self.y * factor }
    }

pub struct Polygon {
    pub vertices: Vec<Point>,
}

impl Polygon {
    pub fn perimeter(&self) -> f64 {
        let n = self.vertices.len();
        if n < 2 {
            return 0.0;
        }
        (0..n)
            .map(|i| {
                let a = &self.vertices[i];
                let b = &self.vertices[(i + 1) % n];
                a.distance(b)
            })
            .sum()
    }

    pub fn is_closed(&self) -> bool {
        self.vertices.len() >= 3
    }
}
