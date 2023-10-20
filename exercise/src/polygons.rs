use std::ops::Add;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point {x, y}
    }
    
    fn magnitude(&self) -> f64 {
        f64::from(self.x.pow(2) + self.y.pow(2)).sqrt()
    }
    
    fn dist(&self, other: Point) -> f64 {
        f64::from(((self.x - other.x).pow(2) + (self.y - other.y).pow(2)).abs()).sqrt()
    }
}

pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn new() -> Polygon {
        Polygon {
            points: Vec::new(),
        }
    }

    fn add_point(&mut self, p: Point) {
        self.points.push(p);
    }
    
    fn left_most_point(&self) -> Option<Point> {
        self.points.iter().min_by_key(|p| p.x).copied()
    }
    
    fn iter(&self) -> impl Iterator<Item=&Point> {
        self.points.iter()
    }
    
    fn perimeter(&self) -> f64 {
        let mut perimeter = 0.0;
        let mut p1 = self.points[0];
        for p in &self.points[1..] {
            perimeter += p1.dist(*p);
            p1 = *p;
        }
        perimeter += p1.dist(self.points[0]);
        perimeter
    }
}

pub struct Circle {
    center: Point,
    radius: u32,
}

impl Circle {
    fn new(center: Point, radius: u32) -> Circle {
        Circle{center, radius}
    }
    
    fn circumference(&self) -> f64 {
        ((2 * self.radius) as f64) * std::f64::consts::PI
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl From<Polygon> for Shape {
    fn from(poly: Polygon) -> Shape {
        Shape::Polygon(poly)
    }
}

impl From<Circle> for Shape {
    fn from(poly: Circle) -> Shape {
        Shape::Circle(poly)
    }
}

impl Shape {
    fn perimeter(&self) -> f64 {
        match self {
            Shape::Polygon(p) => p.perimeter(),
            Shape::Circle(c) => c.circumference(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }

}

fn main() {
}

