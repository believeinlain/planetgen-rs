use glam::Vec3;
use std::array;

#[derive(Debug)]
struct Point {
    index: u16,
    pos: Vec3,
}

#[derive(Debug)]
struct Face {
    corners: [Point; 3],
}

struct Icosphere {
    vertices: Vec<Point>,
    // faces: [Face; 20],
}
impl Icosphere {
    fn new(radius: f32) -> Self {
        // ensure we have a positive radius
        assert!(radius > 0.0);

        // calculate latitude and longitude angles
        let lat_angle = f32::atan(0.5);
        let long_angle = f32::to_radians(36.0);

        // top ring is the opposite side of a triangle
        // with hypotenuse radius and angle latitude_angle
        let top_ring_height = radius * lat_angle.sin();
        let top_ring_radius = radius * lat_angle.cos();

        // define inital icosahedron vertex positions
        let vertices: [Point; 12] = array::from_fn(|i| {
            let pos = match i {
                // top
                0 => [0.0, 0.0, radius],
                // bottom
                11 => [0.0, 0.0, -radius],
                // top ring
                1..=5 => [
                    top_ring_radius * ((i - 1) as f32 * 2.0 * long_angle).cos(),
                    top_ring_radius * ((i - 1) as f32 * 2.0 * long_angle).sin(),
                    top_ring_height,
                ],
                // bottom ring
                6..=10 => [
                    top_ring_radius * (((i - 6) as f32 * 2.0 - 1.0) * long_angle).cos(),
                    top_ring_radius * (((i - 6) as f32 * 2.0 - 1.0) * long_angle).sin(),
                    -top_ring_height,
                ],
                _ => panic!("Invalid number of vertices for Icosahedron"),
            };
            Point { index: i as u16, pos: pos.into() }
        });

        Icosphere { vertices: vertices.into() }
    }
}
