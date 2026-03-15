use gtk::{gdk, glib, prelude::*, subclass::prelude::*};
// use std::{time::Instant};

// TODO Use Die Crate
use crate::die::{Die, DieKind};

mod imp {

    use std::{cell::RefCell, /* time::Instant, f32::consts::PI, */ rc::Rc};
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
        die_screen_positions: Vec<(f32, f32, usize)>,
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

            // TODO Add correct vertex buffer, currently regular tetrahedron
            let ten_vertex_buffer = VertexBuffer::new(
                &context,
                &[
                    Vertex {
                        position: [0.5, 0.5, 0.5],
                        color: [0.753, 0.110, 0.157]
                    },
                    Vertex {
                        position: [0.5, -0.5, -0.5],
                        color: [0.753, 0.110, 0.157]
                    },
                    Vertex {
                        position: [-0.5, 0.5, -0.5],
                        color: [0.753, 0.110, 0.157]
                    },
                    Vertex {
                        position: [-0.5, -0.5, 0.5],
                        color: [0.753, 0.110, 0.157]
                    },
                ],
            )
            .unwrap();

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

            // TODO add correct vertices, currently a tetrahedron
            // Each triangle is a kite, so 2 triangles per face
            let ten_indices: [u16; 12] = [
                0, 1, 2,
                0, 2, 3,
                0, 1, 3,
                1, 2, 3,
            ];

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

            if size != &self.prev_size {
                let n = *size;
                let viewport_width = 1.8f32;
                let base_scale = 0.4f32;

                // Flexbox-style: scale down as more dice are added
                let scale = if n <= 1 {
                    base_scale
                } else {
                    base_scale.min(viewport_width / (n as f32 * 2.0))
                };

                let slot_width = if n <= 1 { 0.0 } else { viewport_width / n as f32 };
                let start_x = -(n as f32 - 1.0) * slot_width / 2.0;

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
                    let x = start_x + i as f32 * slot_width;

                    // Each Rust sub-array = one GLSL column (vec * mat convention)
                    // Translation goes in index 3 of the relevant column
                    let world: [[f32; 4]; 4] = [
                        [scale, 0.0,   0.0,   x  ],
                        [0.0,   scale, 0.0,   0.0],
                        [0.0,   0.0,   scale, 0.0],
                        [0.0,   0.0,   0.0,   1.0],
                    ];

                    let screen_x = (x * aspect_ratio + 1.0) / 2.0 * width as f32;
                    let screen_y = height as f32 / 2.0;
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

            // TODO Implement when glium can detect a Depth Buffer
            /*
            let params = glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::IfLess,
                    write: true,
                    .. Default::default()
                },
                .. Default::default()
            };
            */
            let params = glium::DrawParameters::default();

            // TODO Switch to this when with Depth Buffer
            frame.clear_color_and_depth((0., 0., 0., 0.), 1.0);

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
            renderer.dice.push(Die::new(DieKind::Four));
        } else {
            println!("Renderer doesn't exist");
        }
    }

    pub fn add_six(&self) {
        let imp = self.imp();

        let mut binding = imp.renderer.borrow_mut();
        if let Some(renderer) = binding.as_mut() {
            renderer.dice.push(Die::new(DieKind::Six));
        } else {
            println!("Renderer doesn't exist");
        }
    }

    pub fn add_eight(&self) {
        let imp = self.imp();

        let mut binding = imp.renderer.borrow_mut();
        if let Some(renderer) = binding.as_mut() {
            renderer.dice.push(Die::new(DieKind::Eight));
        } else {
            println!("Renderer doesn't exist");
        }
    }

    pub fn add_ten(&self) {
        let imp = self.imp();

        let mut binding = imp.renderer.borrow_mut();
        if let Some(renderer) = binding.as_mut() {
            renderer.dice.push(Die::new(DieKind::Ten));
        } else {
            println!("Renderer doesn't exist");
        }
    }
    pub fn add_twelve(&self) {
        let imp = self.imp();

        let mut binding = imp.renderer.borrow_mut();
        if let Some(renderer) = binding.as_mut() {
            renderer.dice.push(Die::new(DieKind::Twelve));
        } else {
            println!("Renderer doesn't exist");
        }
    }

    pub fn add_twenty(&self) {
        let imp = self.imp();

        let mut binding = imp.renderer.borrow_mut();
        if let Some(renderer) = binding.as_mut() {
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

    pub fn start_tick(&self) {
        self.add_tick_callback(|s, _| {
            s.queue_draw();
            glib::ControlFlow::Continue
        });
    }
}

