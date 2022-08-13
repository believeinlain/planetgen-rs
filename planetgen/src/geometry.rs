use std::{array, cell::RefCell, rc::Rc};

use crate::graphics;

// wrapping coordinates in a RefCell allows vertices to move
type MutScalar = RefCell<f32>;

#[derive(Debug, Default, Clone)]
struct Vert {
    index: usize,
    x: MutScalar,
    y: MutScalar,
    z: MutScalar,
}
impl Vert {
    pub fn new(index: usize, x: f32, y: f32, z: f32) -> Self {
        Vert {
            index,
            x: MutScalar::new(x),
            y: MutScalar::new(y),
            z: MutScalar::new(z),
        }
    }
    pub fn set_coords(&self, x: f32, y: f32, z: f32) {
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
    pub fn get_index(&self) -> usize {
        self.index
    }
    pub fn get_x(&self) -> f32 {
        *self.x.borrow()
    }
    pub fn get_y(&self) -> f32 {
        *self.y.borrow()
    }
    pub fn get_z(&self) -> f32 {
        *self.z.borrow()
    }
}

#[derive(Debug, Default)]
struct Edge {
    endpoint: (Rc<Vert>, Rc<Vert>),
}
impl Edge {
    fn new(verts: &[Rc<Vert>], endpoints: (usize, usize)) -> Self {
        Edge {
            endpoint: (verts[endpoints.0].clone(), verts[endpoints.1].clone()),
        }
    }
}

#[derive(Debug, Default)]
struct Face {
    corner: (Rc<Vert>, Rc<Vert>, Rc<Vert>),
    edge: (Rc<Edge>, Rc<Edge>, Rc<Edge>),
}
impl Face {
    fn new(
        verts: &[Rc<Vert>],
        edges: &[Rc<Edge>],
        corners: (usize, usize, usize),
        sides: (usize, usize, usize),
    ) -> Self {
        Face {
            corner: (
                verts[corners.0].clone(),
                verts[corners.1].clone(),
                verts[corners.2].clone(),
            ),
            edge: (
                edges[sides.0].clone(),
                edges[sides.1].clone(),
                edges[sides.2].clone(),
            ),
        }
    }
}

#[derive(Debug, Clone)]
struct Icosahedron {
    radius: f32,
    verts: [Rc<Vert>; 12],
    edges: [Rc<Edge>; 30],
    faces: [Rc<Face>; 20],
}
impl Default for Icosahedron {
    fn default() -> Self {
        Icosahedron::new(5.0)
    }
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
                    0 => Vert::new(i, 0.0, 0.0, radius),
                    // bottom
                    11 => Vert::new(i, 0.0, 0.0, -radius),
                    // top ring [1..=5]
                    1..=5 => Vert::new(
                        i,
                        top_ring_radius * ((i - 1) as f32 * 2.0 * long_angle).cos(),
                        top_ring_radius * ((i - 1) as f32 * 2.0 * long_angle).sin(),
                        top_ring_height,
                    ),
                    // bottom ring [6..=10]
                    6..=10 => Vert::new(
                        i,
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

        // create array of faces
        let faces = array::from_fn(|i| {
            Rc::new(match i {
                // top faces
                0 => Face::new(&verts, &edges, (2, 1, 0), (0, 1, 5)),
                1 => Face::new(&verts, &edges, (3, 2, 0), (1, 2, 6)),
                2 => Face::new(&verts, &edges, (4, 3, 0), (2, 3, 7)),
                3 => Face::new(&verts, &edges, (5, 4, 0), (3, 4, 8)),
                4 => Face::new(&verts, &edges, (1, 5, 0), (4, 0, 9)),
                // ring faces
                5 => Face::new(&verts, &edges, (7, 6, 1), (20, 10, 11)),
                6 => Face::new(&verts, &edges, (7, 1, 2), (5, 11, 12)),
                7 => Face::new(&verts, &edges, (8, 7, 2), (21, 12, 13)),
                8 => Face::new(&verts, &edges, (8, 2, 3), (6, 13, 14)),
                9 => Face::new(&verts, &edges, (9, 8, 3), (22, 14, 15)),
                10 => Face::new(&verts, &edges, (9, 3, 4), (7, 15, 16)),
                11 => Face::new(&verts, &edges, (10, 9, 4), (23, 16, 17)),
                12 => Face::new(&verts, &edges, (10, 4, 5), (8, 17, 18)),
                13 => Face::new(&verts, &edges, (6, 10, 5), (24, 18, 19)),
                14 => Face::new(&verts, &edges, (6, 5, 1), (9, 19, 10)),
                // bottom faces
                15 => Face::new(&verts, &edges, (6, 7, 11), (25, 26, 20)),
                16 => Face::new(&verts, &edges, (7, 8, 11), (26, 27, 21)),
                17 => Face::new(&verts, &edges, (8, 9, 11), (27, 28, 22)),
                18 => Face::new(&verts, &edges, (9, 10, 11), (28, 29, 23)),
                19 => Face::new(&verts, &edges, (10, 6, 11), (29, 25, 24)),
                _ => panic!("Invalid number of faces for Icosahedron"),
            })
        });

        Icosahedron {
            radius,
            verts,
            edges,
            faces,
        }
    }
    pub fn get_vertex_buffer(&self) -> Vec<graphics::Vertex> {
        Vec::from_iter(self.verts.iter().map(|v| graphics::Vertex {
            position: [v.get_x(), v.get_y(), v.get_z()],
            color: [1.0, 1.0, 1.0],
        }))
    }
    pub fn get_index_buffer(&self) -> Vec<graphics::Index> {
        Vec::from_iter(self.faces.iter().flat_map(|f| {
            [
                f.corner.0.get_index() as graphics::Index,
                f.corner.1.get_index() as graphics::Index,
                f.corner.2.get_index() as graphics::Index,
            ]
        }))
    }
}
