#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub center: Point,
	pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64) -> Circle {
        let point = Point(x, y);
        Self {center: point, radius: r}
    }
    pub fn diameter(&self) -> f64 {
        let d: f64 = 2.0 * self.radius;
        d
    }

    pub fn area(&self) -> f64 {
        let area = std::f64::consts::PI * self.radius.powi(2);
        area
    }

    pub fn intersect(&self, circle: Circle) -> bool {
        // d<r1​+r2​
        let point_1 = &self.center;
        let point_2 = circle.center;
        let d = point_1.distance(point_2);
        if d < self.radius + circle.radius {
            return true;
        }
        false
    }

}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(&self, points: Point) -> f64 {
        let x1 = self.0;
        let y1 = self.1;
        let x2 = points.0;
        let y2 = points.1;
        let res = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
        res
    }
}