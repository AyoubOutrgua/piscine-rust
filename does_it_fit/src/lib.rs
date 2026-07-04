pub mod areas_volumes;
use crate::areas_volumes::*;
pub use areas_volumes::{GeometricalShapes, GeometricalVolumes};

pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let shape_area = (x * y) as f64;

    let area = match kind {
        GeometricalShapes::Square => square_area(a) as f64,
        GeometricalShapes::Rectangle => rectangle_area(a, b) as f64,
        GeometricalShapes::Triangle => triangle_area(a, b),
        GeometricalShapes::Circle => circle_area(a),
    };

    shape_area <= (area * times as f64)
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let shape_volume = (x * y * z) as f64;

    let volume = match kind {
        GeometricalVolumes::Cube => cube_volume(a) as f64,
        GeometricalVolumes::Sphere => sphere_volume(a) as f64,
        GeometricalVolumes::Cone => cone_volume(a, b) as f64,
        GeometricalVolumes::TriangularPyramid => triangular_pyramid_volume(a as f64, b),
        GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c) as f64,
    };

    shape_volume <= (volume * times as f64)
}