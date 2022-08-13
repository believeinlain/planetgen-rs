use std::{array, cell::RefCell, rc::Rc};

// wrapping coordinates in a RefCell allows vertices to move
type MutScalar = RefCell<f32>;

type VertIndex = usize;
type EdgeIndex = usize;
type FaceIndex = usize;

#[derive(Debug, Default, Clone)]
struct Vert {
    x: MutScalar,
    y: MutScalar,
    z: MutScalar,
}
impl Vert {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vert {
            x: MutScalar::new(x),
            y: MutScalar::new(y),
            z: MutScalar::new(z),
        }
    }
    pub fn set(&self, x: f32, y: f32, z: f32) {
        self.x.replace(x);
        self.y.replace(y);
        self.z.replace(z);
    }
    pub fn set_x(&self, x: f32) {
        self.x.replace(x);
    }
    pub fn set_y(&self, y: f32) {
        self.y.replace(y);
    }
    pub fn set_z(&self, z: f32) {
        self.z.replace(z);
    }
}

#[derive(Debug, Default)]
struct Edge {
    endpoint: (Rc<Vert>, Rc<Vert>),
}
impl Edge {
    pub fn new(vert_array: &[Rc<Vert>], endpoints: (VertIndex, VertIndex)) -> Self {
        Edge {
            endpoint: (
                // cloning the Rc simply creates another reference
                // to the same data
                vert_array[endpoints.0].clone(),
                vert_array[endpoints.1].clone(),
            ),
        }
    }
}

#[derive(Debug, Default)]
struct Face {
    corner: (VertIndex, VertIndex, VertIndex),
    edge: (EdgeIndex, EdgeIndex, EdgeIndex),
}

#[derive(Debug)]
struct Icosahedron {
    radius: f32,
    verts: [Rc<Vert>; 12],
    edges: [Rc<Edge>; 30],
}
impl Icosahedron {
    fn new(radius: f32) -> Self {
        // create array of vertices
        let verts = array::from_fn(|i| {
            Rc::new({
                // calculate latitude and longitude angles
                let lat_angle = f32::atan(0.5);
                let long_angle = f32::to_radians(36.0);

                // top ring is the opposite side of a triangle
                // with hypotenuse radius and angle latitude_angle
                let top_ring_height = radius * lat_angle.sin();
                let top_ring_radius = radius * lat_angle.cos();

                match i {
                    // top
                    0 => Vert::new(0.0, 0.0, radius),
                    // bottom
                    11 => Vert::new(0.0, 0.0, -radius),
                    // top ring [1..=5]
                    1..=5 => Vert::new(
                        top_ring_radius * ((i - 1) as f32 * 2.0 * long_angle).cos(),
                        top_ring_radius * ((i - 1) as f32 * 2.0 * long_angle).sin(),
                        top_ring_height,
                    ),
                    // bottom ring [6..=10]
                    6..=10 => Vert::new(
                        top_ring_radius * (((i - 6) as f32 * 2.0 - 1.0) * long_angle).cos(),
                        top_ring_radius * (((i - 6) as f32 * 2.0 - 1.0) * long_angle).sin(),
                        -top_ring_height,
                    ),
                    _ => panic!("Invalid number of vertices for Icosahedron"),
                }
            })
        });

        // create array of edges
        let edges = array::from_fn(|i| {
            Rc::new(match i {
                // add top edges connecting to top vertex
                0 => Edge::new(&verts, (0, 1)),
                1 => Edge::new(&verts, (0, 1)),
                2 => Edge::new(&verts, (0, 1)),
                3 => Edge::new(&verts, (0, 1)),
                4 => Edge::new(&verts, (0, 1)),
                // add top ring of edges
                5 => Edge::new(&verts, (0, 1)),
                6 => Edge::new(&verts, (0, 1)),
                7 => Edge::new(&verts, (0, 1)),
                8 => Edge::new(&verts, (0, 1)),
                9 => Edge::new(&verts, (0, 1)),
                // add middle zigzag edges
                10 => Edge::new(&verts, (0, 1)),
                11 => Edge::new(&verts, (0, 1)),
                12 => Edge::new(&verts, (0, 1)),
                13 => Edge::new(&verts, (0, 1)),
                14 => Edge::new(&verts, (0, 1)),
                15 => Edge::new(&verts, (0, 1)),
                16 => Edge::new(&verts, (0, 1)),
                17 => Edge::new(&verts, (0, 1)),
                18 => Edge::new(&verts, (0, 1)),
                19 => Edge::new(&verts, (0, 1)),
                // add bottom ring of edges
                20 => Edge::new(&verts, (0, 1)),
                21 => Edge::new(&verts, (0, 1)),
                22 => Edge::new(&verts, (0, 1)),
                23 => Edge::new(&verts, (0, 1)),
                24 => Edge::new(&verts, (0, 1)),
                // add bottom edges connecting to bottom vertex
                25 => Edge::new(&verts, (0, 1)),
                26 => Edge::new(&verts, (0, 1)),
                27 => Edge::new(&verts, (0, 1)),
                28 => Edge::new(&verts, (0, 1)),
                29 => Edge::new(&verts, (0, 1)),
                _ => panic!("Invalid number of edges for Icosahedron"),
            })
        });

        Icosahedron {
            radius,
            verts,
            edges,
        }
    }
}
