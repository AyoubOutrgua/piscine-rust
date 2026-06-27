#[derive(Debug, Clone, Copy)]
// pub struct Circle {
// 	pub center //..
// 	pub radius //..
// }

// impl Circle {
//     // ...
// }

// #[derive(Debug, Clone, Copy)]
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