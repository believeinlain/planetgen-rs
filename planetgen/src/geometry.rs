#[derive(Debug, Default, Clone, Copy)]
struct Vert {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug)]
struct Edge {
    endpoint: (usize, usize),
}

#[derive(Debug)]
struct Face {
    vertex: (usize, usize, usize),
}

#[derive(Debug)]
struct Icosahedron {
    radius: f32,
    verts: [Vert; 12],
    edges: [Edge; 30],
}
impl Icosahedron {
    fn new(radius: f32) -> Self {
        // initialize all verts at 0, 0, 0
        let mut verts = [Vert::default(); 12];

        // top
        verts[0].z = radius;
        // bottom
        verts[11].z = -radius;

        let lat_angle = f32::atan(0.5);
        let long_angle = f32::to_radians(36.0);

        // top ring is the opposite side of a triangle with hypotenuse radius and angle latitude_angle
        let top_ring_height = radius * lat_angle.sin();
        let top_ring_radius = radius * lat_angle.cos();

        for i in 1..5 {
            // top ring [1..5]
            verts[1 + i] = Vert {
                x: top_ring_radius * f32::cos(i as f32 * 2.0 * long_angle),
                y: top_ring_radius * f32::sin(i as f32 * 2.0 * long_angle),
                z: top_ring_height,
            };
            // bottom ring [6..10]
            verts[6 + i] = Vert {
                x: top_ring_radius * f32::cos((i as f32 * 2.0 - 1.0) * long_angle),
                y: top_ring_radius * f32::sin((i as f32 * 2.0 - 1.0) * long_angle),
                z: -top_ring_height,
            }
        }

        // initialize edge array
        let mut edges = Vec::<Edge>::with_capacity(30);

        // add top edges connecting to top vertex
        edges.push(Edge { endpoint: (0, 1) }); // 0
        edges.push(Edge { endpoint: (0, 2) }); // 1
        edges.push(Edge { endpoint: (0, 3) }); // 2
        edges.push(Edge { endpoint: (0, 4) }); // 3
        edges.push(Edge { endpoint: (0, 5) }); // 4

        // add top ring of edges
        edges.push(Edge { endpoint: (1, 2) }); // 5
        edges.push(Edge { endpoint: (2, 3) }); // 6
        edges.push(Edge { endpoint: (3, 4) }); // 7
        edges.push(Edge { endpoint: (4, 5) }); // 8
        edges.push(Edge { endpoint: (5, 1) }); // 9

        // add middle zigzag edges
        edges.push(Edge { endpoint: (6, 1) }); // 10
        edges.push(Edge { endpoint: (1, 7) }); // 11
        edges.push(Edge { endpoint: (7, 2) }); // 12
        edges.push(Edge { endpoint: (2, 8) }); // 13
        edges.push(Edge { endpoint: (8, 3) }); // 14
        edges.push(Edge { endpoint: (3, 9) }); // 15
        edges.push(Edge { endpoint: (9, 4) }); // 16
        edges.push(Edge { endpoint: (4, 10) }); // 17
        edges.push(Edge { endpoint: (10, 5) }); // 18
        edges.push(Edge { endpoint: (5, 6) }); // 19

        // add bottom ring of edges
        edges.push(Edge { endpoint: (6, 7) }); // 20
        edges.push(Edge { endpoint: (7, 8) }); // 21
        edges.push(Edge { endpoint: (8, 9) }); // 22
        edges.push(Edge { endpoint: (9, 10) }); // 23
        edges.push(Edge { endpoint: (10, 6) }); // 24

        // add bottom edges connecting to bottom vertex
        edges.push(Edge { endpoint: (11, 6) }); // 25
        edges.push(Edge { endpoint: (11, 7) }); // 26
        edges.push(Edge { endpoint: (11, 8) }); // 27
        edges.push(Edge { endpoint: (11, 9) }); // 28
        edges.push(Edge { endpoint: (11, 10) }); // 29

        Icosahedron {
            radius,
            verts: verts,
            edges: edges.try_into().unwrap(),
        }
    }
}
