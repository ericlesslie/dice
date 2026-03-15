use gtk::{gdk, glib, prelude::*, subclass::prelude::*};
// use std::{time::Instant};

// TODO Use Die Crate
use crate::die::{Die, DieKind};

mod imp {

    use std::{cell::RefCell, rc::Rc, f32::consts::PI};
    use glium::{
        implement_vertex, index::PrimitiveType, program, uniform, Frame, IndexBuffer, Surface,
        VertexBuffer
    };
    use gtk::{glib, prelude::*, subclass::prelude::*};

    use crate::die::{Die, DieKind};

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 3],
        color: [f32; 3],
    }
    implement_vertex!(Vertex, position, color);

    #[derive(Copy, Clone)]
    struct Attr {
        // Fit both rotations and translations here
        world_matrix: [[f32; 4]; 4],
    }
    implement_vertex!(Attr, world_matrix);

    pub(super) const SPIN_DURATION: f32 = 1.5;
    pub(super) const MAX_DICE: usize = 20;

    // Builds a world matrix for the vec * mat shader convention (column-major).
    // Applies: result = Scale * Rz*Ry*Rx * position + Translation
    // M[col][row] where result_col = dot(position, M[col])
    fn build_world_matrix(scale: f32, tx: f32, ty: f32, ax: f32, ay: f32, az: f32) -> [[f32; 4]; 4] {
        let (sx, cx) = ax.sin_cos();
        let (sy, cy) = ay.sin_cos();
        let (sz, cz) = az.sin_cos();
        let s = scale;
        [
            [s*cy*cz,   s*(sx*sy*cz - cx*sz), s*(cx*sy*cz + sx*sz), tx ],
            [s*cy*sz,   s*(sx*sy*sz + cx*cz), s*(cx*sy*sz - sx*cz), ty ],
            [s*(-sy),   s*sx*cy,              s*cx*cy,              0.0],
            [0.0,       0.0,                  0.0,                  1.0],
        ]
    }

    fn settled_rotation(kind: DieKind, val: u32) -> (f32, f32, f32) {
        use std::f32::consts::FRAC_PI_2;

        // atan(1/sqrt(2)) — used for D4, D8, D20 cube-diagonal normals
        const C1: f32 = 0.6154797086703874;

        match kind {
            DieKind::Six => {
                match val {
                    1 => (0.0, 0.0, 0.0),           // front face (-Z)
                    2 => (0.0, FRAC_PI_2, 0.0),     // right face
                    3 => (-FRAC_PI_2, 0.0, 0.0),    // top face
                    4 => (FRAC_PI_2, 0.0, 0.0),     // bottom face
                    5 => (0.0, -FRAC_PI_2, 0.0),    // left face
                    6 => (0.0, PI, 0.0),             // back face (+Z)
                    _ => (0.0, 0.0, 0.0),
                }
            }
            DieKind::Four => {
                // Tetrahedron face normals: ±(1,1,-1)/√3 permutations
                // Formula: ax = atan2(-ny, -nz), ay = atan2(nx, sqrt(ny²+nz²))
                match val {
                    1 => (-PI / 4.0, C1, 0.0),        // normal ∝ (1,1,-1)
                    2 => (-3.0 * PI / 4.0, -C1, 0.0), // normal ∝ (-1,1,1)
                    3 => (3.0 * PI / 4.0, C1, 0.0),   // normal ∝ (1,-1,1)
                    4 => (PI / 4.0, -C1, 0.0),         // normal ∝ (-1,-1,-1)
                    _ => (0.0, 0.0, 0.0),
                }
            }
            DieKind::Eight => {
                // Octahedron face normals: ±(1,1,1)/√3 permutations
                // Formula: ax = atan2(-ny, -nz), ay = atan2(nx, sqrt(ny²+nz²))
                match val {
                    1 => (-3.0 * PI / 4.0, C1, 0.0),  // normal ∝ (1,1,1)
                    2 => (-PI / 4.0, C1, 0.0),         // normal ∝ (1,1,-1)
                    3 => (3.0 * PI / 4.0, C1, 0.0),   // normal ∝ (1,-1,1)
                    4 => (PI / 4.0, C1, 0.0),          // normal ∝ (1,-1,-1)
                    5 => (-3.0 * PI / 4.0, -C1, 0.0), // normal ∝ (-1,1,1)
                    6 => (-PI / 4.0, -C1, 0.0),        // normal ∝ (-1,1,-1)
                    7 => (3.0 * PI / 4.0, -C1, 0.0),  // normal ∝ (-1,-1,1)
                    8 => (PI / 4.0, -C1, 0.0),         // normal ∝ (-1,-1,-1)
                    _ => (0.0, 0.0, 0.0),
                }
            }
            DieKind::Ten => {
                // Pentagonal trapezohedron face normals, computed from geometry.
                // Upper kite faces (1-5) and lower kite faces (6-10).
                let ten_radius: f32 = 0.5;
                let upper_y: f32 = 0.0792;
                let lower_y: f32 = -0.0792;
                let apex_top: [f32; 3] = [0.0, 0.75, 0.0];
                let apex_bot: [f32; 3] = [0.0, -0.75, 0.0];

                let upper_ring: Vec<[f32; 3]> = (0..5).map(|i| {
                    let a = (i as f32) * 2.0 * PI / 5.0;
                    [ten_radius * a.cos(), upper_y, ten_radius * a.sin()]
                }).collect();
                let lower_ring: Vec<[f32; 3]> = (0..5).map(|i| {
                    let a = (i as f32) * 2.0 * PI / 5.0 + PI / 5.0;
                    [ten_radius * a.cos(), lower_y, ten_radius * a.sin()]
                }).collect();

                // Compute face centroid, then derive Euler angles
                // Formula: ax = atan2(-ny, -nz), ay = atan2(nx, sqrt(ny²+nz²))
                let face_normal = |v0: [f32; 3], v1: [f32; 3], v2: [f32; 3], v3: [f32; 3]| -> (f32, f32) {
                    // centroid of 4 kite vertices as proxy for face normal direction
                    let nx = (v0[0] + v1[0] + v2[0] + v3[0]) / 4.0;
                    let ny = (v0[1] + v1[1] + v2[1] + v3[1]) / 4.0;
                    let nz = (v0[2] + v1[2] + v2[2] + v3[2]) / 4.0;
                    let len = (nx * nx + ny * ny + nz * nz).sqrt();
                    let (nx, ny, nz) = (nx / len, ny / len, nz / len);
                    let ax = (-ny).atan2(-nz);
                    let ay = nx.atan2((ny * ny + nz * nz).sqrt());
                    (ax, ay)
                };

                let k = (val - 1) as usize;
                let (ax, ay) = if val <= 5 {
                    // Upper kite: apex_top, upper[k], lower[k], upper[k+1]
                    face_normal(apex_top, upper_ring[k], lower_ring[k], upper_ring[(k + 1) % 5])
                } else {
                    // Lower kite: apex_bot, lower[k-5], upper[k-4], lower[k-4]
                    let j = k - 5;
                    face_normal(apex_bot, lower_ring[j], upper_ring[(j + 1) % 5], lower_ring[(j + 1) % 5])
                };
                (ax, ay, 0.0)
            }
            DieKind::Twelve => {
                // Dodecahedron face normals: proportional to (0, ±φ, ±1) permutations
                // Formula: ax = atan2(-ny, -nz), ay = atan2(nx, sqrt(ny²+nz²))
                const PHI: f32 = 1.618033988749895;
                let a1: f32 = PHI.atan();           // atan(φ) ≈ 1.0172
                let a2: f32 = (1.0 / PHI).atan();   // atan(1/φ) ≈ 0.5536
                match val {
                    1  => (a1 - PI, 0.0, 0.0),        // normal ∝ (0,φ,1)
                    2  => (PI, a2, 0.0),               // normal ∝ (1,0,φ)
                    3  => (-FRAC_PI_2, a1, 0.0),       // normal ∝ (φ,1,0)
                    4  => (PI, -a2, 0.0),              // normal ∝ (-1,0,φ)
                    5  => (-FRAC_PI_2, -a1, 0.0),      // normal ∝ (-φ,1,0)
                    6  => (FRAC_PI_2, a1, 0.0),        // normal ∝ (φ,-1,0)
                    7  => (PI - a1, 0.0, 0.0),         // normal ∝ (0,-φ,1)
                    8  => (-a1, 0.0, 0.0),             // normal ∝ (0,φ,-1)
                    9  => (0.0, a2, 0.0),              // normal ∝ (1,0,-φ)
                    10 => (0.0, -a2, 0.0),             // normal ∝ (-1,0,-φ)
                    11 => (FRAC_PI_2, -a1, 0.0),       // normal ∝ (-φ,-1,0)
                    12 => (a1, 0.0, 0.0),              // normal ∝ (0,-φ,-1)
                    _  => (0.0, 0.0, 0.0),
                }
            }
            DieKind::Twenty => {
                // Icosahedron face normals computed from actual vertex positions
                // Formula: ax = atan2(-ny, -nz), ay = atan2(nx, sqrt(ny²+nz²))
                const PHI: f32 = 1.618033988749895;
                let b2: f32 = (1.0 / (PHI * PHI)).atan();  // atan(1/φ²) ≈ 0.3649
                let b3: f32 = (PHI * PHI).atan();           // atan(φ²) ≈ 1.2059
                match val {
                    1  => (PI, -b2, 0.0),              // normal ∝ (-1,0,φ²)
                    2  => (-3.0 * PI / 4.0, -C1, 0.0), // normal ∝ (-1,1,1)
                    3  => (b3 - PI, 0.0, 0.0),         // normal ∝ (0,φ²,1)
                    4  => (-3.0 * PI / 4.0, C1, 0.0),  // normal ∝ (1,1,1)
                    5  => (PI, b2, 0.0),               // normal ∝ (1,0,φ²)
                    6  => (3.0 * PI / 4.0, C1, 0.0),   // normal ∝ (1,-1,1)
                    7  => (PI - b3, 0.0, 0.0),         // normal ∝ (0,-φ²,1)
                    8  => (3.0 * PI / 4.0, -C1, 0.0),  // normal ∝ (-1,-1,1)
                    9  => (0.0, b2, 0.0),              // normal ∝ (1,0,-φ²)
                    10 => (-PI / 4.0, C1, 0.0),        // normal ∝ (1,1,-1)
                    11 => (-b3, 0.0, 0.0),             // normal ∝ (0,φ²,-1)
                    12 => (-PI / 4.0, -C1, 0.0),       // normal ∝ (-1,1,-1)
                    13 => (0.0, -b2, 0.0),             // normal ∝ (-1,0,-φ²)
                    14 => (PI / 4.0, C1, 0.0),         // normal ∝ (1,-1,-1)
                    15 => (b3, 0.0, 0.0),              // normal ∝ (0,-φ²,-1)
                    16 => (PI / 4.0, -C1, 0.0),        // normal ∝ (-1,-1,-1)
                    17 => (-FRAC_PI_2, b3, 0.0),       // normal ∝ (φ²,1,0)
                    18 => (FRAC_PI_2, b3, 0.0),        // normal ∝ (φ²,-1,0)
                    19 => (-FRAC_PI_2, -b3, 0.0),      // normal ∝ (-φ²,1,0)
                    20 => (FRAC_PI_2, -b3, 0.0),       // normal ∝ (-φ²,-1,0)
                    _  => (0.0, 0.0, 0.0),
                }
            }
        }
    }

    fn die_scale(kind: DieKind) -> f32 {
        // Normalize so all dice appear the same visual size.
        // Factor = reference_radius / actual_bounding_radius
        // Reference: D6 radius = sqrt(3)*0.5 ≈ 0.866
        const REF: f32 = 0.866;
        match kind {
            DieKind::Four   => REF / 0.866,  // 1.0
            DieKind::Six    => REF / 0.866,  // 1.0
            DieKind::Eight  => REF / 0.5,    // 1.732
            DieKind::Ten    => REF / 0.75,   // bounding radius = apex height
            DieKind::Twelve => REF / 0.866,  // 1.0
            DieKind::Twenty => REF / 0.951,  // 0.911
        }
    }

    pub struct Renderer {
        context: Rc<glium::backend::Context>,
        program: glium::Program,

        four_vertex_buffer: VertexBuffer<Vertex>,
        four_index_buffer: IndexBuffer<u16>,
        four_per_instance: VertexBuffer<Attr>,

        six_vertex_buffer: VertexBuffer<Vertex>,
        six_index_buffer: IndexBuffer<u16>,
        six_per_instance: VertexBuffer<Attr>,

        eight_vertex_buffer: VertexBuffer<Vertex>,
        eight_index_buffer: IndexBuffer<u16>,
        eight_per_instance: VertexBuffer<Attr>,

        ten_vertex_buffer: VertexBuffer<Vertex>,
        ten_index_buffer: IndexBuffer<u16>,
        ten_per_instance: VertexBuffer<Attr>,

        twelve_vertex_buffer: VertexBuffer<Vertex>,
        twelve_index_buffer: IndexBuffer<u16>,
        twelve_per_instance: VertexBuffer<Attr>,

        twenty_vertex_buffer: VertexBuffer<Vertex>,
        twenty_index_buffer: IndexBuffer<u16>,
        twenty_per_instance: VertexBuffer<Attr>,

        pub dice: Vec<Die>,
        prev_size: usize,
        pub die_screen_positions: Vec<(f32, f32, usize)>,
    }

    impl Renderer {
        fn new(context: Rc<glium::backend::Context>) -> Self {
            // The following code is based on glium's triangle example:
            // https://github.com/glium/glium/blob/2ff5a35f6b097889c154b42ad0233c6cdc6942f4/examples/triangle.rs

            // std::f32::consts::PHI is still experimental
            const PHI: f32 = 1.618033988749894848204586834365638118_f32;

            let four_vertex_buffer = VertexBuffer::new(
                &context,
                &[
                    Vertex {
                        position: [0.5, 0.5, 0.5],
                        color: [0.180, 0.761, 0.494],
                    },
                    Vertex {
                        position: [0.5, -0.5, -0.5],
                        color: [0.180, 0.761, 0.494],
                    },
                    Vertex {
                        position: [-0.5, 0.5, -0.5],
                        color: [0.180, 0.761, 0.494],
                    },
                    Vertex {
                        position: [-0.5, -0.5, 0.5],
                        color: [0.180, 0.761, 0.494],
                    },
                ],
            )
            .unwrap();


            let six_vertex_buffer = VertexBuffer::new(
                &context,
                &[
                    Vertex {
                        position: [-0.5, -0.5, -0.5],
                        color: [0.110, 0.443, 0.847],
                    },
                    Vertex {
                        position: [0.5, -0.5, -0.5],
                        color: [0.110, 0.443, 0.847],
                    },
                    Vertex {
                        position: [0.5, 0.5, -0.5],
                        color: [0.110, 0.443, 0.847],
                    },
                    Vertex {
                        position: [-0.5, 0.5, -0.5],
                        color: [0.110, 0.443, 0.847],
                    },
                    Vertex {
                        position: [-0.5, -0.5, 0.5],
                        color: [0.110, 0.443, 0.847],
                    },
                    Vertex {
                        position: [0.5, -0.5, 0.5],
                        color: [0.110, 0.443, 0.847],
                    },
                    Vertex {
                        position: [0.5, 0.5, 0.5],
                        color: [0.110, 0.443, 0.847],
                    },
                    Vertex {
                        position: [-0.5, 0.5, 0.5],
                        color: [0.110, 0.443, 0.847],
                    },
                ],
            )
            .unwrap();

            let eight_vertex_buffer = VertexBuffer::new(
                &context,
                &[
                    Vertex {
                        position: [0.5, 0., 0.],
                        color: [0.506, 0.239, 0.612],
                    },
                    Vertex {
                        position: [-0.5, 0., 0.],
                        color: [0.506, 0.239, 0.612],
                    },
                    Vertex {
                        position: [0., 0.5, 0.],
                        color: [0.506, 0.239, 0.612],
                    },
                    Vertex {
                        position: [0., -0.5, 0.],
                        color: [0.506, 0.239, 0.612],
                    },
                    Vertex {
                        position: [0., 0., 0.5],
                        color: [0.506, 0.239, 0.612],
                    },
                    Vertex {
                        position: [0., 0., -0.5],
                        color: [0.506, 0.239, 0.612],
                    },
                ],
            )
            .unwrap();

            // Pentagonal trapezohedron (D10)
            // 2 apex vertices + 2 rings of 5 vertices each
            let ten_color: [f32; 3] = [0.753, 0.110, 0.157];
            let ten_radius: f32 = 0.5;
            let ten_upper_y: f32 = 0.0792;
            let ten_lower_y: f32 = -0.0792;
            let mut ten_verts: Vec<Vertex> = Vec::with_capacity(12);
            // Vertex 0: top apex
            ten_verts.push(Vertex { position: [0.0, 0.75, 0.0], color: ten_color });
            // Vertex 1: bottom apex
            ten_verts.push(Vertex { position: [0.0, -0.75, 0.0], color: ten_color });
            // Vertices 2-6: upper ring (y=0.2, angles 0, 72, 144, 216, 288)
            for i in 0..5u32 {
                let angle = (i as f32) * 2.0 * PI / 5.0;
                ten_verts.push(Vertex {
                    position: [ten_radius * angle.cos(), ten_upper_y, ten_radius * angle.sin()],
                    color: ten_color,
                });
            }
            // Vertices 7-11: lower ring (y=-0.2, angles 36, 108, 180, 252, 324)
            for i in 0..5u32 {
                let angle = (i as f32) * 2.0 * PI / 5.0 + PI / 5.0;
                ten_verts.push(Vertex {
                    position: [ten_radius * angle.cos(), ten_lower_y, ten_radius * angle.sin()],
                    color: ten_color,
                });
            }
            let ten_vertex_buffer = VertexBuffer::new(&context, &ten_verts).unwrap();

            let twelve_vertex_buffer = VertexBuffer::new(
                &context,
                &[
                    Vertex {
                        position: [0.5, 0.5, 0.5],
                        color: [0.961, 0.761, 0.067]
                    },
                    Vertex {
                        position: [-0.5, 0.5, 0.5],
                        color: [0.961, 0.761, 0.067]
                    },
                    Vertex {
                        position: [0.5, -0.5, 0.5],
                        color: [0.961, 0.761, 0.067]
                    },
                    Vertex {
                        position: [0.5, 0.5, -0.5],
                        color: [0.961, 0.761, 0.067]
                    },
                    Vertex {
                        position: [-0.5, -0.5, 0.5],
                        color: [0.961, 0.761, 0.067]
                    },
                    Vertex {
                        position: [0.5, -0.5, -0.5],
                        color: [0.961, 0.761, 0.067]
                    },
                    Vertex {
                        position: [-0.5, 0.5, -0.5],
                        color: [0.961, 0.761, 0.067]
                    },
                    Vertex {
                        position: [-0.5, -0.5, -0.5],
                        color: [0.961, 0.761, 0.067]
                    },
                    Vertex {
                        position: [0., 0.5 / PHI, PHI / 2.],
                        color: [0.961, 0.761, 0.067]
                    },
                    Vertex {
                        position: [0., -0.5 / PHI, PHI / 2.],
                        color: [0.961, 0.761, 0.067]
                    },
                    Vertex {
                        position: [0., 0.5 / PHI, -PHI / 2.],
                        color: [0.961, 0.761, 0.067]
                    },
                    Vertex {
                        position: [0., -0.5 / PHI, -PHI / 2.],
                        color: [0.961, 0.761, 0.067]
                    },
                    Vertex {
                        position: [0.5 / PHI, PHI / 2., 0.],
                        color: [0.961, 0.761, 0.067]
                    },
                    Vertex {
                        position: [-0.5 / PHI, PHI / 2., 0.],
                        color: [0.961, 0.761, 0.067]
                    },
                    Vertex {
                        position: [0.5 / PHI, -PHI / 2., 0.],
                        color: [0.961, 0.761, 0.067]
                    },
                    Vertex {
                        position: [-0.5 / PHI, -PHI / 2., 0.],
                        color: [0.961, 0.761, 0.067]
                    },
                    Vertex {
                        position: [PHI / 2., 0., 0.5 / PHI],
                        color: [0.961, 0.761, 0.067]
                    },
                    Vertex {
                        position: [-PHI / 2., 0., 0.5 / PHI],
                        color: [0.961, 0.761, 0.067]
                    },
                    Vertex {
                        position: [PHI / 2., 0., -0.5 / PHI],
                        color: [0.961, 0.761, 0.067]
                    },
                    Vertex {
                        position: [-PHI / 2., 0., -0.5 / PHI],
                        color: [0.961, 0.761, 0.067]
                    },
                ],
            )
            .unwrap();

            let twenty_vertex_buffer = VertexBuffer::new(
                &context,
                &[
                    Vertex {
                        position: [0., 0.5, PHI / 2.],
                        color: [0.902, 0.380, 0.000]
                    },
                    Vertex {
                        position: [0., -0.5, PHI / 2.],
                        color: [0.902, 0.380, 0.000]
                    },
                    Vertex {
                        position: [0., 0.5, -PHI / 2.],
                        color: [0.902, 0.380, 0.000]
                    },
                    Vertex {
                        position: [0., -0.5, -PHI / 2.],
                        color: [0.902, 0.380, 0.000]
                    },
                    Vertex {
                        position: [0.5, PHI / 2., 0.],
                        color: [0.902, 0.380, 0.000]
                    },
                    Vertex {
                        position: [0.5, -PHI / 2., 0.],
                        color: [0.902, 0.380, 0.000]
                    },
                    Vertex {
                        position: [-0.5, PHI / 2., 0.],
                        color: [0.902, 0.380, 0.000]
                    },
                    Vertex {
                        position: [-0.5, -PHI / 2., 0.],
                        color: [0.902, 0.380, 0.000]
                    },
                    Vertex {
                        position: [PHI / 2., 0., 0.5],
                        color: [0.902, 0.380, 0.000]
                    },
                    Vertex {
                        position: [PHI / 2., 0., -0.5],
                        color: [0.902, 0.380, 0.000]
                    },
                    Vertex {
                        position: [-PHI / 2., 0., 0.5],
                        color: [0.902, 0.380, 0.000]
                    },
                    Vertex {
                        position: [-PHI / 2., 0., -0.5],
                        color: [0.902, 0.380, 0.000]
                    },
                ],
            )
            .unwrap();

            // Each triangle is a face
            let four_indices: [u16; 12] = [
                0, 1, 2,
                0, 2, 3,
                0, 1, 3,
                1, 2, 3,
            ];

            let six_indices: [u16; 36] = [
                // Front face
                0, 1, 2,
                2, 3, 0,

                // Back face
                4, 5, 6,
                6, 7, 4,

                // Left face
                0, 4, 7,
                7, 3, 0,

                // Right face
                1, 5, 6,
                6, 2, 1,

                // Top face
                3, 2, 6,
                6, 7, 3,

                // Bottom face
                0, 1, 5,
                5, 4, 0
            ];

            // Each triangle is a face
            let eight_indices: [u16; 24] = [
                0, 2, 4,
                0, 2, 5,
                0, 3, 4,
                0, 3, 5,
                1, 2, 4,
                1, 2, 5,
                1, 3, 4,
                1, 3, 5
            ];

            // 10 kite faces × 2 triangles each = 60 indices
            // Upper kites connect top apex (0) to upper[k] and lower[k]
            // Lower kites connect bottom apex (1) to lower[k] and upper[k+1]
            let mut ten_indices: Vec<u16> = Vec::with_capacity(60);
            for k in 0..5u16 {
                let upper_k = 2 + k;            // upper ring: indices 2-6
                let upper_next = 2 + (k + 1) % 5;
                let lower_k = 7 + k;            // lower ring: indices 7-11
                let lower_next = 7 + (k + 1) % 5;
                // Upper kite face
                ten_indices.extend_from_slice(&[0, upper_k, lower_k]);
                ten_indices.extend_from_slice(&[0, lower_k, upper_next]);
                // Lower kite face
                ten_indices.extend_from_slice(&[1, lower_k, upper_next]);
                ten_indices.extend_from_slice(&[1, upper_next, lower_next]);
            }

            // Pentagonal faces, 3 triangles per face (fan triangulation)
            let twelve_indices: [u16; 108] = [
                // Face 1: 0, 8, 1, 13, 12
                0, 8, 1,
                0, 1, 13,
                0, 13, 12,

                // Face 2: 0, 8, 9, 2, 16
                0, 8, 9,
                0, 9, 2,
                0, 2, 16,

                // Face 3: 0, 12, 3, 18, 16
                0, 12, 3,
                0, 3, 18,
                0, 18, 16,

                // Face 4: 1, 17, 4, 9, 8
                1, 17, 4,
                1, 4, 9,
                1, 9, 8,

                // Face 5: 1, 13, 6, 19, 17
                1, 13, 6,
                1, 6, 19,
                1, 19, 17,

                // Face 6: 2, 14, 5, 18, 16
                2, 14, 5,
                2, 5, 18,
                2, 18, 16,

                // Face 7: 2, 14, 15, 4, 9
                2, 14, 15,
                2, 15, 4,
                2, 4, 9,

                // Face 8: 3, 10, 6, 13, 12
                3, 10, 6,
                3, 6, 13,
                3, 13, 12,

                // Face 9: 3, 10, 11, 5, 18
                3, 10, 11,
                3, 11, 5,
                3, 5, 18,

                // Face 10: 7, 11, 10, 6, 19
                7, 11, 10,
                7, 10, 6,
                7, 6, 19,

                // Face 11: 7, 15, 4, 17, 19
                7, 15, 4,
                7, 4, 17,
                7, 17, 19,

                // Face 12: 7, 11, 5, 14, 15
                7, 11, 5,
                7, 5, 14,
                7, 14, 15,
            ];

            let twenty_indices: [u16; 60] = [
                // All faces with 0
                0, 1, 10,
                0, 10, 6,
                0, 6, 4,
                0, 4, 8,
                0, 8, 1,

                // Remaining faces with 1
                1, 8, 5,
                1, 5, 7,
                1, 7, 10,

                // Remaining faces with 2
                2, 3, 9,
                2, 9, 4,
                2, 4, 6,
                2, 6, 11,
                2, 11, 3,

                // Remaining faces with 3
                3, 9, 5,
                3, 5, 7,
                3, 7, 11,

                // Remaining faces with 4, 5, 6, and 7
                4, 8, 9,
                5, 8, 9,
                6, 10, 11,
                7, 10, 11,
            ];

            let four_index_buffer =
                IndexBuffer::new(&context, PrimitiveType::TrianglesList, &four_indices).unwrap();

            let six_index_buffer =
                IndexBuffer::new(&context, PrimitiveType::TrianglesList, &six_indices).unwrap();

            let eight_index_buffer =
                IndexBuffer::new(&context, PrimitiveType::TrianglesList, &eight_indices).unwrap();

            let ten_index_buffer =
                IndexBuffer::new(&context, PrimitiveType::TrianglesList, &ten_indices).unwrap();

            let twelve_index_buffer =
                IndexBuffer::new(&context, PrimitiveType::TrianglesList, &twelve_indices).unwrap();

            let twenty_index_buffer =
                IndexBuffer::new(&context, PrimitiveType::TrianglesList, &twenty_indices).unwrap();


            // TODO get the GResource state
            let four_per_instance: VertexBuffer<Attr> = VertexBuffer::empty_dynamic(&context, 0).unwrap();
            let six_per_instance: VertexBuffer<Attr> = VertexBuffer::empty_dynamic(&context, 0).unwrap();
            let eight_per_instance: VertexBuffer<Attr> = VertexBuffer::empty_dynamic(&context, 0).unwrap();
            let ten_per_instance: VertexBuffer<Attr> = VertexBuffer::empty_dynamic(&context, 0).unwrap();
            let twelve_per_instance: VertexBuffer<Attr> = VertexBuffer::empty_dynamic(&context, 0).unwrap();
            let twenty_per_instance: VertexBuffer<Attr> = VertexBuffer::empty_dynamic(&context, 0).unwrap();

            let program = program!(&context,
                // This example includes a shader that requires GLSL 1.40 or above.
                //
                // GLSL 1.40 is supported by GL 3.1 or above, but not by GLES 2.0 which only supports
                // GLSL 1.00. GLES 3.0 supports GLSL 3.00 ES, which also supports the shader below but
                // requires the floating point precision to be specified.
                //
                // GL < 3.1 and GLES < 3.0 are not supported by the example.
                //
                // In practice, the version of GLSL for the shaders inside your application depends on
                // the GL context you're either creating or using — i.e. if you support multiple versions
                // of GL then you should load different shaders.
                //
                // If you only care about recent GL, as you should, then going for GLSL 1.50 is
                // perfectly fine; anything else will error out, and you can catch that error and fall
                // back to something else. This example simply unwrap()s on error and does not
                // implement a fallback or error reporting.
                300 es => {
                    vertex: "
                        #version 300 es

                        in mat4 world_matrix;
                        uniform mat4 perspective;

                        in vec3 position;
                        in vec3 color;
                        out vec3 vColor;
                        out vec3 vPosition;

                        void main() {
                            vec4 worldPos = vec4(position, 1.0) * world_matrix;
                            gl_Position = worldPos * perspective;
                            vColor = color;
                            vPosition = worldPos.xyz;
                        }
                    ",

                    fragment: "
                        #version 300 es
                        precision mediump float;
                        in vec3 vColor;
                        in vec3 vPosition;

                        out vec4 f_color;
                        void main() {
                            vec3 normal = normalize(cross(dFdx(vPosition), dFdy(vPosition)));
                            vec3 lightDir = normalize(vec3(0.3, 0.5, 1.0));
                            float diffuse = abs(dot(normal, lightDir));
                            float ambient = 0.3;
                            float lighting = ambient + (1.0 - ambient) * diffuse;
                            f_color = vec4(vColor * lighting, 1.0);
                        }
                    "
                },
                150 => {
                    vertex: "
                        #version 150
                        in mat4 world_matrix;
                        uniform mat4 perspective;

                        in vec3 position;
                        in vec3 color;

                        out vec3 vColor;
                        out vec3 vPosition;

                        void main() {
                            vec4 worldPos = vec4(position, 1.0) * world_matrix;
                            gl_Position = worldPos * perspective;
                            vColor = color;
                            vPosition = worldPos.xyz;
                        }
                    ",

                    fragment: "
                        #version 150
                        in vec3 vColor;
                        in vec3 vPosition;
                        out vec4 f_color;
                        void main() {
                            vec3 normal = normalize(cross(dFdx(vPosition), dFdy(vPosition)));
                            vec3 lightDir = normalize(vec3(0.3, 0.5, 1.0));
                            float diffuse = abs(dot(normal, lightDir));
                            float ambient = 0.3;
                            float lighting = ambient + (1.0 - ambient) * diffuse;
                            f_color = vec4(vColor * lighting, 1.0);
                        }
                    "
                },
            )
            .unwrap();

            let dice = Vec::new();
            let prev_size = 0usize;

            Renderer {
                context,
                program,
                four_vertex_buffer,
                four_index_buffer,
                four_per_instance,
                six_vertex_buffer,
                six_index_buffer,
                six_per_instance,
                eight_vertex_buffer,
                eight_index_buffer,
                eight_per_instance,
                ten_vertex_buffer,
                ten_index_buffer,
                ten_per_instance,
                twelve_vertex_buffer,
                twelve_index_buffer,
                twelve_per_instance,
                twenty_vertex_buffer,
                twenty_index_buffer,
                twenty_per_instance,
                dice,
                prev_size,
                die_screen_positions: Vec::new(),
            }
        }

        fn draw(&mut self) {
            let mut frame = Frame::new(
                self.context.clone(),
                self.context.get_framebuffer_dimensions(),
            );

            let perspective = {
                let (width, height) = self.context.get_framebuffer_dimensions();
                let aspect_ratio = height as f32 / width as f32;

                //let fov: f32 = 3.141592 / 3.0;
                //let zfar = 1024.0;
                //let znear = 0.1;

                //let f = 1.0 / (fov / 2.0).tan();

                [
                    [1. *   aspect_ratio,    0.0 ,     0.0,   0.0],
                    [         0.0       ,    1.  ,     0.0,   0.0],
                    [         0.0       ,    0.0 ,     1.0,   0.0],
                    [         0.0       ,    0.0 ,     0.0,   1.0],
                ]
            };

            let uniforms = uniform! {
                perspective: perspective,
            };

            let size: &usize = &self.dice.len();

            /* Updating
                let mut mapping = self.six_per_instance.map();


                for instance in mapping.iter_mut() {
                    instance.world_matrix = translate;
                }
            */

            let any_animating = self.dice.iter().any(|die| {
                if let Some(t) = die.time.get() {
                    t.elapsed().as_secs_f32() < SPIN_DURATION
                } else {
                    false
                }
            });

            if size != &self.prev_size || any_animating {
                let n = *size;
                let viewport_width = 1.8f32;
                let viewport_height = 1.6f32;
                let base_scale = 0.4f32;
                const MAX_PER_ROW: usize = 5;

                // Grid layout: distribute dice across rows
                let rows = if n == 0 { 1 } else { (n + MAX_PER_ROW - 1) / MAX_PER_ROW };
                let cols = if n == 0 { 0 } else { (n + rows - 1) / rows }; // distribute evenly

                // Scale to fit both dimensions
                let scale = if n <= 1 {
                    base_scale
                } else {
                    base_scale
                        .min(viewport_width / (cols as f32 * 2.0))
                        .min(viewport_height / (rows as f32 * 2.0))
                };

                let slot_width = if cols <= 1 { 0.0 } else { viewport_width / cols as f32 };
                let slot_height = if rows <= 1 { 0.0 } else { viewport_height / rows as f32 };

                let mut four_instances: Vec<Attr> = Vec::new();
                let mut six_instances: Vec<Attr> = Vec::new();
                let mut eight_instances: Vec<Attr> = Vec::new();
                let mut ten_instances: Vec<Attr> = Vec::new();
                let mut twelve_instances: Vec<Attr> = Vec::new();
                let mut twenty_instances: Vec<Attr> = Vec::new();

                let (width, height) = self.context.get_framebuffer_dimensions();
                let aspect_ratio = height as f32 / width as f32;

                self.die_screen_positions.clear();

                for (i, die) in self.dice.iter().enumerate() {
                    let row = i / cols;
                    let col = i % cols;
                    let cols_this_row = (n - row * cols).min(cols);

                    let x = -(cols_this_row as f32 - 1.0) * slot_width / 2.0 + col as f32 * slot_width;
                    let y = (rows as f32 - 1.0) * slot_height / 2.0 - row as f32 * slot_height;

                    let elapsed = die.time.get()
                        .map(|t| t.elapsed().as_secs_f32())
                        .unwrap_or(SPIN_DURATION);
                    let t = (elapsed / SPIN_DURATION).min(1.0);
                    let eased = 1.0 - (1.0 - t).powi(3); // ease-out cubic

                    let (settled_x, settled_y, settled_z) = settled_rotation(die.kind, die.val.get());
                    let seed = die.spin_seed.get();

                    let angle_x = eased * (settled_x + seed[0] as f32 * 2.0 * PI);
                    let angle_y = eased * (settled_y + seed[1] as f32 * 2.0 * PI);
                    let angle_z = eased * (settled_z + seed[2] as f32 * 2.0 * PI);

                    let world = build_world_matrix(scale * die_scale(die.kind), x, y, angle_x, angle_y, angle_z);

                    let screen_x = (x * aspect_ratio + 1.0) / 2.0 * width as f32;
                    let screen_y = (1.0 - y) / 2.0 * height as f32;
                    self.die_screen_positions.push((screen_x, screen_y, i));

                    let attr = Attr { world_matrix: world };
                    match die.kind {
                        DieKind::Four => four_instances.push(attr),
                        DieKind::Six => six_instances.push(attr),
                        DieKind::Eight => eight_instances.push(attr),
                        DieKind::Ten => ten_instances.push(attr),
                        DieKind::Twelve => twelve_instances.push(attr),
                        DieKind::Twenty => twenty_instances.push(attr),
                    }
                }

                self.four_per_instance = if four_instances.is_empty() {
                    VertexBuffer::empty_dynamic(&self.context, 0).unwrap()
                } else {
                    VertexBuffer::dynamic(&self.context, &four_instances).unwrap()
                };
                self.six_per_instance = if six_instances.is_empty() {
                    VertexBuffer::empty_dynamic(&self.context, 0).unwrap()
                } else {
                    VertexBuffer::dynamic(&self.context, &six_instances).unwrap()
                };
                self.eight_per_instance = if eight_instances.is_empty() {
                    VertexBuffer::empty_dynamic(&self.context, 0).unwrap()
                } else {
                    VertexBuffer::dynamic(&self.context, &eight_instances).unwrap()
                };
                self.ten_per_instance = if ten_instances.is_empty() {
                    VertexBuffer::empty_dynamic(&self.context, 0).unwrap()
                } else {
                    VertexBuffer::dynamic(&self.context, &ten_instances).unwrap()
                };
                self.twelve_per_instance = if twelve_instances.is_empty() {
                    VertexBuffer::empty_dynamic(&self.context, 0).unwrap()
                } else {
                    VertexBuffer::dynamic(&self.context, &twelve_instances).unwrap()
                };
                self.twenty_per_instance = if twenty_instances.is_empty() {
                    VertexBuffer::empty_dynamic(&self.context, 0).unwrap()
                } else {
                    VertexBuffer::dynamic(&self.context, &twenty_instances).unwrap()
                };

                self.prev_size = *size;
            }

            let params = glium::DrawParameters::default();

            frame.clear_color(0., 0., 0., 0.);

            // GTK manages the depth buffer externally, so glium can't detect it.
            // Enable depth testing via raw GL calls through epoxy.
            unsafe {
                const GL_DEPTH_TEST: u32 = 0x0B71;
                const GL_LESS: u32 = 0x0201;
                const GL_DEPTH_BUFFER_BIT: u32 = 0x00000100;

                type GlEnable = unsafe extern "C" fn(u32);
                type GlDepthFunc = unsafe extern "C" fn(u32);
                type GlDepthMask = unsafe extern "C" fn(u8);
                type GlClear = unsafe extern "C" fn(u32);

                let enable: GlEnable = std::mem::transmute(epoxy::get_proc_addr("glEnable"));
                let depth_func: GlDepthFunc = std::mem::transmute(epoxy::get_proc_addr("glDepthFunc"));
                let depth_mask: GlDepthMask = std::mem::transmute(epoxy::get_proc_addr("glDepthMask"));
                let clear: GlClear = std::mem::transmute(epoxy::get_proc_addr("glClear"));

                enable(GL_DEPTH_TEST);
                depth_func(GL_LESS);
                depth_mask(1); // GL_TRUE
                clear(GL_DEPTH_BUFFER_BIT);
            }

            if self.four_per_instance.len() > 0 {
                frame
                    .draw(
                       (&self.four_vertex_buffer,
                        self.four_per_instance.per_instance().unwrap()),
                        &self.four_index_buffer,
                        &self.program,
                        &uniforms,
                        &params,
                    )
                    .unwrap();
            }

            if self.six_per_instance.len() > 0 {
                frame
                    .draw(
                       (&self.six_vertex_buffer,
                        self.six_per_instance.per_instance().unwrap()),
                        &self.six_index_buffer,
                        &self.program,
                        &uniforms,
                        &params,
                    )
                    .unwrap();
            }

            if self.eight_per_instance.len() > 0 {
                frame
                    .draw(
                       (&self.eight_vertex_buffer,
                        self.eight_per_instance.per_instance().unwrap()),
                        &self.eight_index_buffer,
                        &self.program,
                        &uniforms,
                        &params,
                    )
                    .unwrap();
            }

            if self.ten_per_instance.len() > 0 {
                frame
                    .draw(
                       (&self.ten_vertex_buffer,
                        self.ten_per_instance.per_instance().unwrap()),
                        &self.ten_index_buffer,
                        &self.program,
                        &uniforms,
                        &params,
                    )
                    .unwrap();
            }

            if self.twelve_per_instance.len() > 0 {
                frame
                    .draw(
                       (&self.twelve_vertex_buffer,
                        self.twelve_per_instance.per_instance().unwrap()),
                        &self.twelve_index_buffer,
                        &self.program,
                        &uniforms,
                        &params,
                    )
                    .unwrap();
            }

            if self.twenty_per_instance.len() > 0 {
                frame
                    .draw(
                       (&self.twenty_vertex_buffer,
                        self.twenty_per_instance.per_instance().unwrap()),
                        &self.twenty_index_buffer,
                        &self.program,
                        &uniforms,
                        &params,
                    )
                    .unwrap();
            }
            frame.finish().unwrap();
        }
    }

    #[derive(Default)]
    pub struct DiceArea {
        pub renderer: RefCell<Option<Renderer>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DiceArea {
        const NAME: &'static str = "DiceArea";
        type Type = super::DiceArea;
        type ParentType = gtk::GLArea;
    }

    impl ObjectImpl for DiceArea {
        fn constructed(&self) {
            self.parent_constructed();

            self.obj().add_tick_callback(|widget, _clock| {
                widget.queue_draw();
                glib::ControlFlow::Continue
            });

            let click = gtk::GestureClick::new();
            click.connect_pressed(glib::clone!(@weak self as this => move |_gesture, _n, x, y| {
                let widget = this.obj();
                let scale = widget.scale_factor() as f32;
                let click_x = x as f32 * scale;
                let click_y = y as f32 * scale;

                let mut binding = this.renderer.borrow_mut();
                if let Some(renderer) = binding.as_mut() {
                    let threshold = 80.0f32;
                    let mut closest: Option<(f32, usize)> = None;

                    for &(sx, sy, idx) in &renderer.die_screen_positions {
                        let dist = ((click_x - sx).powi(2) + (click_y - sy).powi(2)).sqrt();
                        if dist < threshold {
                            if closest.is_none() || dist < closest.unwrap().0 {
                                closest = Some((dist, idx));
                            }
                        }
                    }

                    if let Some((_, idx)) = closest {
                        if idx < renderer.dice.len() {
                            renderer.dice.remove(idx);
                        }
                    }
                }
            }));
            self.obj().add_controller(click);
        }
    }

    impl WidgetImpl for DiceArea {
        fn realize(&self) {
            self.parent_realize();

            let widget = self.obj();
            if widget.error().is_some() {
                return;
            }

            // SAFETY: we know the GdkGLContext exists as we checked for errors above, and
            // we haven't done any operations on it which could lead to glium's
            // state mismatch. (In theory, GTK doesn't do any state-breaking
            // operations on the context either.)
            //
            // We will also ensure glium's context does not outlive the GdkGLContext by
            // destroying it in `unrealize()`.
            let context =
                unsafe { glium::backend::Context::new(widget.clone(), true, Default::default()) }
                    .unwrap();

            println!("{:?}", context.get_opengl_version_string());

            *self.renderer.borrow_mut() = Some(Renderer::new(context));
        }

        fn unrealize(&self) {
            *self.renderer.borrow_mut() = None;
            self.parent_unrealize();
        }
    }

    impl GLAreaImpl for DiceArea {
        // Is a glib::Propagation in post 0.7 gtk, need to figure out how to update
        fn render(&self, _context: &gtk::gdk::GLContext) -> bool {
            self.renderer.borrow_mut().as_mut().unwrap().draw();
            false
        }

    }
}


glib::wrapper! {
    pub struct DiceArea(ObjectSubclass<imp::DiceArea>)
        @extends gtk::GLArea, gtk::Widget;
}

impl Default for DiceArea {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl glium::backend::Backend for DiceArea {
    fn swap_buffers(&self) -> Result<(), glium::SwapBuffersError> {
        // We're supposed to draw (and hence swap buffers) only inside the `render()`
        // vfunc or signal, which means that GLArea will handle buffer swaps for
        // us.
        Ok(())
    }

    unsafe fn get_proc_address(&self, symbol: &str) -> *const std::ffi::c_void {
        epoxy::get_proc_addr(symbol)
    }

    fn get_framebuffer_dimensions(&self) -> (u32, u32) {
        let scale = self.scale_factor();
        let width = self.width();
        let height = self.height();
        ((width * scale) as u32, (height * scale) as u32)
    }

    fn is_current(&self) -> bool {
        match self.context() {
            Some(context) => gdk::GLContext::current() == Some(context),
            None => false,
        }
    }

    unsafe fn make_current(&self) {
        GLAreaExt::make_current(self);
    }

    fn resize(&self, size: (u32, u32)) {
        self.set_size_request(size.0 as i32, size.1 as i32);
    }
}

impl DiceArea {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn start_roll(&self) {

    }

    pub fn add_four(&self) {
        let imp = self.imp();

        let mut binding = imp.renderer.borrow_mut();
        if let Some(renderer) = binding.as_mut() {
            if renderer.dice.len() >= imp::MAX_DICE { return; }
            renderer.dice.push(Die::new(DieKind::Four));
        } else {
            println!("Renderer doesn't exist");
        }
    }

    pub fn add_six(&self) {
        let imp = self.imp();

        let mut binding = imp.renderer.borrow_mut();
        if let Some(renderer) = binding.as_mut() {
            if renderer.dice.len() >= imp::MAX_DICE { return; }
            renderer.dice.push(Die::new(DieKind::Six));
        } else {
            println!("Renderer doesn't exist");
        }
    }

    pub fn add_eight(&self) {
        let imp = self.imp();

        let mut binding = imp.renderer.borrow_mut();
        if let Some(renderer) = binding.as_mut() {
            if renderer.dice.len() >= imp::MAX_DICE { return; }
            renderer.dice.push(Die::new(DieKind::Eight));
        } else {
            println!("Renderer doesn't exist");
        }
    }

    pub fn add_ten(&self) {
        let imp = self.imp();

        let mut binding = imp.renderer.borrow_mut();
        if let Some(renderer) = binding.as_mut() {
            if renderer.dice.len() >= imp::MAX_DICE { return; }
            renderer.dice.push(Die::new(DieKind::Ten));
        } else {
            println!("Renderer doesn't exist");
        }
    }
    pub fn add_twelve(&self) {
        let imp = self.imp();

        let mut binding = imp.renderer.borrow_mut();
        if let Some(renderer) = binding.as_mut() {
            if renderer.dice.len() >= imp::MAX_DICE { return; }
            renderer.dice.push(Die::new(DieKind::Twelve));
        } else {
            println!("Renderer doesn't exist");
        }
    }

    pub fn add_twenty(&self) {
        let imp = self.imp();

        let mut binding = imp.renderer.borrow_mut();
        if let Some(renderer) = binding.as_mut() {
            if renderer.dice.len() >= imp::MAX_DICE { return; }
            renderer.dice.push(Die::new(DieKind::Twenty));
        } else {
            println!("Renderer doesn't exist");
        }
    }

    pub fn roll(&self) {
        let imp = self.imp();

        let mut binding = imp.renderer.borrow_mut();
        if let Some(renderer) = binding.as_mut() {
            for die in renderer.dice.iter_mut() {
                die.roll();
            }
        } else {
            println!("Renderer doesn't exist");
        }
    }

    pub fn settled_dice_info(&self) -> Vec<(f32, f32, u32)> {
        let imp_ref = self.imp();
        let binding = imp_ref.renderer.borrow();
        let scale_factor = self.scale_factor() as f32;
        if let Some(renderer) = binding.as_ref() {
            let positions = &renderer.die_screen_positions;
            renderer.dice.iter().enumerate().filter_map(|(i, die)| {
                let elapsed = die.time.get()
                    .map(|t| t.elapsed().as_secs_f32())
                    .unwrap_or(imp::SPIN_DURATION);
                if elapsed >= imp::SPIN_DURATION {
                    positions.iter()
                        .find(|&&(_, _, idx)| idx == i)
                        .map(|&(sx, sy, _)| (sx / scale_factor, sy / scale_factor, die.val.get()))
                } else {
                    None
                }
            }).collect()
        } else {
            Vec::new()
        }
    }

    pub fn start_tick(&self) {
        self.add_tick_callback(|s, _| {
            s.queue_draw();
            glib::ControlFlow::Continue
        });
    }
}

