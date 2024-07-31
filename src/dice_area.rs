use gtk::{gdk, glib, prelude::*, subclass::prelude::*};
use std::{time::Instant};

use crate::glib::clone;

use crate::die::Die;

mod imp {

    use std::{cell::RefCell, time::Instant, f32::consts::PI, rc::Rc};
    use glium::{
        implement_vertex, index::PrimitiveType, program, uniform, Frame, IndexBuffer, Surface,
        VertexBuffer
    };
    use gtk::{glib, prelude::*, subclass::prelude::*};

    use crate::die::Die;

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

    implement_vertex!(Attr, world_position);

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

        dice: RefCell<Vec<Die>>,
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
                        color: [0., 1., 0.],
                    },
                    Vertex {
                        position: [0.5, -0.5, -0.5],
                        color: [0., 0., 1.],
                    },
                    Vertex {
                        position: [-0.5, 0.5, -0.5],
                        color: [1., 0., 0.],
                    },
                    Vertex {
                        position: [-0.5, -0.5, 0.5],
                        color: [0., 1., 0.],
                    },
                ],
            )
            .unwrap();


            let six_vertex_buffer = VertexBuffer::new(
                &context,
                &[
                    Vertex {
                        position: [-0.5, -0.5, -0.5],
                        color: [0., 1., 0.],
                    },
                    Vertex {
                        position: [0.5, -0.5, -0.5],
                        color: [0., 0., 1.],
                    },
                    Vertex {
                        position: [0.5, 0.5, -0.5],
                        color: [1., 0., 0.],
                    },
                    Vertex {
                        position: [-0.5, 0.5, -0.5],
                        color: [0., 1., 0.],
                    },
                    Vertex {
                        position: [-0.5, -0.5, 0.5],
                        color: [0., 0., 1.],
                    },
                    Vertex {
                        position: [0.5, -0.5, 0.5],
                        color: [0., 1., 0.],
                    },
                    Vertex {
                        position: [0.5, 0.5, 0.5],
                        color: [1., 0., 0.],
                    },
                    Vertex {
                        position: [-0.5, 0.5, 0.5],
                        color: [0., 1., 0.],
                    },
                ],
            )
            .unwrap();

            let eight_vertex_buffer = VertexBuffer::new(
                &context,
                &[
                    Vertex {
                        position: [0.5, 0., 0.],
                        color: [0., 1., 0.],
                    },
                    Vertex {
                        position: [-0.5, 0., 0.],
                        color: [0., 0., 1.],
                    },
                    Vertex {
                        position: [0., 0.5, 0.],
                        color: [1., 0., 0.],
                    },
                    Vertex {
                        position: [0., -0.5, 0.],
                        color: [0., 1., 0.],
                    },
                    Vertex {
                        position: [0., 0., 0.5],
                        color: [0., 0., 1.],
                    },
                    Vertex {
                        position: [0., 0., -0.5],
                        color: [0., 1., 0.],
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
                        color: [0., 1., 0.],
                    },
                    Vertex {
                        position: [0.5, -0.5, -0.5],
                        color: [0., 0., 1.],
                    },
                    Vertex {
                        position: [-0.5, 0.5, -0.5],
                        color: [1., 0., 0.],
                    },
                    Vertex {
                        position: [-0.5, -0.5, 0.5],
                        color: [0., 1., 0.],
                    },
                ],
            )
            .unwrap();

            let twelve_vertex_buffer = VertexBuffer::new(
                &context,
                &[
                    Vertex {
                        position: [0.5, 0.5, 0.5],
                        color: [0., 1., 0.],
                    },
                    Vertex {
                        position: [-0.5, 0.5, 0.5],
                        color: [0., 0., 1.],
                    },
                    Vertex {
                        position: [0.5, -0.5, 0.5],
                        color: [1., 0., 0.],
                    },
                    Vertex {
                        position: [0.5, 0.5, -0.5],
                        color: [0., 1., 0.],
                    },
                    Vertex {
                        position: [-0.5, -0.5, 0.5],
                        color: [0., 0., 1.],
                    },
                    Vertex {
                        position: [0.5, -0.5, -0.5],
                        color: [0., 1., 0.],
                    },
                    Vertex {
                        position: [-0.5, 0.5, -0.5],
                        color: [0., 1., 0.],
                    },
                    Vertex {
                        position: [-0.5, -0.5, -0.5],
                        color: [0., 1., 0.],
                    },
                    Vertex {
                        position: [0., 0.5 / PHI, PHI / 2.],
                        color: [0., 1., 0.],
                    },
                    Vertex {
                        position: [0., -0.5 / PHI, PHI / 2.],
                        color: [0., 1., 0.],
                    },
                    Vertex {
                        position: [0., 0.5 / PHI, -PHI / 2.],
                        color: [0., 1., 0.],
                    },
                    Vertex {
                        position: [0., -0.5 / PHI, -PHI / 2.],
                        color: [0., 1., 0.],
                    },
                    Vertex {
                        position: [0.5 / PHI, PHI / 2., 0.],
                        color: [0., 1., 0.],
                    },
                    Vertex {
                        position: [-0.5 / PHI, PHI / 2., 0.],
                        color: [0., 1., 0.],
                    },
                    Vertex {
                        position: [0.5 / PHI, -PHI / 2., 0.],
                        color: [0., 1., 0.],
                    },
                    Vertex {
                        position: [-0.5 / PHI, -PHI / 2., 0.],
                        color: [0., 1., 0.],
                    },
                    Vertex {
                        position: [PHI / 2., 0., 0.5 / PHI],
                        color: [0., 1., 0.],
                    },
                    Vertex {
                        position: [-PHI / 2., 0., 0.5 / PHI],
                        color: [0., 1., 0.],
                    },
                    Vertex {
                        position: [PHI / 2., 0., -0.5 / PHI],
                        color: [0., 1., 0.],
                    },
                    Vertex {
                        position: [-PHI / 2., 0., -0.5 / PHI],
                        color: [0., 1., 0.],
                    },
                ],
            )
            .unwrap();

            let twenty_vertex_buffer = VertexBuffer::new(
                &context,
                &[
                    Vertex {
                        position: [0., 0.5, PHI / 2.],
                        color: [0., 1., 0.],
                    },
                    Vertex {
                        position: [0., -0.5, PHI / 2.],
                        color: [0., 0., 1.],
                    },
                    Vertex {
                        position: [0., 0.5, -PHI / 2.],
                        color: [0., 0., 1.],
                    },
                    Vertex {
                        position: [0., -0.5, -PHI / 2.],
                        color: [0., 0., 1.],
                    },
                    Vertex {
                        position: [0.5, PHI / 2., 0.],
                        color: [1., 0., 0.],
                    },
                    Vertex {
                        position: [0.5, -PHI / 2., 0.],
                        color: [0., 1., 0.],
                    },
                    Vertex {
                        position: [-0.5, PHI / 2., 0.],
                        color: [0., 0., 1.],
                    },
                    Vertex {
                        position: [-0.5, -PHI / 2., 0.],
                        color: [0., 1., 0.],
                    },
                    Vertex {
                        position: [PHI / 2., 0., 0.5],
                        color: [1., 0., 0.],
                    },
                    Vertex {
                        position: [PHI / 2., 0., -0.5],
                        color: [1., 0., 0.],
                    },
                    Vertex {
                        position: [-PHI / 2., 0., 0.5],
                        color: [1., 0., 0.],
                    },
                    Vertex {
                        position: [-PHI / 2., 0., -0.5],
                        color: [1., 0., 0.],
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

            // Pentagonal faces, 3 triangles per face
            let twelve_indices: [u16; 108] = [
                // Face 1
                0, 1, 9,
                0, 1, 14,
                0, 14, 13,

                // Face 2
                0, 13, 3,
                0, 3,  19,
                0, 19, 18,

                // Face 3
                0, 17, 2,
                0, 2, 10,
                0, 10, 9,

                // Face 4
                8, 7, 20,
                8, 7, 11,
                8, 11, 12,

                // Face 5
                8, 12, 6,
                8, 6, 15,
                8, 15, 16,

                // Face 6
                8, 16, 5,
                8, 5, 18,
                8, 18, 20,

                // Face 7
                20, 18, 4,
                20, 4, 14,
                20, 14, 7,

                // Face 8
                11, 7, 14,
                11, 14, 13,
                11, 13, 3,

                // Face 9
                11, 3, 19,
                11, 19, 6,
                11, 6, 12,

                // Face 10
                15, 6, 19,
                15, 19, 17,
                15, 17, 2,

                // Face 11
                15, 2, 10,
                15, 10, 5,
                15, 5, 16,

                // Face 12
                5, 10, 9,
                5, 9, 4,
                5, 4, 18

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
            let four_per_instance = {
                let data = Vec::new();
                VertexBuffer::dynamic(&context, &data).unwrap()
            };

            let six_per_instance = {
                let data = Vec::new();
                VertexBuffer::dynamic(&context, &data).unwrap()
            };

            let eight_per_instance = {
                let data = Vec::new();
                VertexBuffer::dynamic(&context, &data).unwrap()
            };

            let ten_per_instance = {
                let data = Vec::new();
                VertexBuffer::dynamic(&context, &data).unwrap()
            };

            let twelve_per_instance = {
                let data = Vec::new();
                VertexBuffer::dynamic(&context, &data).unwrap()
            };

            let twenty_per_instance = {
                let data = Vec::new();
                VertexBuffer::dynamic(&context, &data).unwrap()
            };


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
                140 => {
                    vertex: "
                        #version 140
                        uniform mat4 matrix;
                        uniform mat4 perspective;

                        in mat4 world_matrix
                        in vec3 position;
                        in vec3 color;
                        out vec3 vColor;
                        void main() {
                            gl_Position = vec4(position, 1.0) * world_matrix * perspective;
                            vColor = color;
                        }
                    ",

                    fragment: "
                        #version 140
                        in vec3 vColor;
                        out vec4 f_color;
                        void main() {
                            f_color = vec4(vColor, 1.0);
                        }
                    "
                },
                300 es => {
                    vertex: "
                        #version 300 es

                        uniform mat4 matrix;
                        uniform mat4 perspective;

                        in vec3 position;
                        in vec3 color;
                        out vec3 vColor;

                        void main() {
                            gl_Position = vec4(position, 1.0) * matrix * perspective;
                            vColor = color;
                        }
                    ",

                    fragment: "
                        #version 300 es
                        precision mediump float;
                        in vec3 vColor;

                        out vec4 f_color;
                        void main() {
                            f_color = vec4(vColor, 1.0);
                        }
                    "
                },
            )
            .unwrap();




            let dice = RefCell::new(Vec::new());

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
            }
        }

        fn draw(&self) {
            let mut frame = Frame::new(
                self.context.clone(),
                self.context.get_framebuffer_dimensions(),
            );

            /* TODO Needs to be in world_matrix
            let matrix = {
                let mut theta: f32 = 0.0;
                let mut phi: f32 = 0.0;
                let mut psi: f32 = 0.0;

                if let Some(start) = elapsed {
                    let duration = start.elapsed().as_secs_f32();
                    println!("{:?}", duration);

                    if duration > 4.0 {
                        let mut borrow = *self.roll.borrow_mut();
                        borrow.take();
                    } else {
                        theta = duration / 4.0 * PI;
                        phi = theta * 2.0;
                        psi = theta * 4.0;
                    }
                }

                [
                    [                                      theta.cos() * psi.cos(),                                      theta.cos() * psi.sin(),            -theta.sin(),    0.],
                    [ -phi.cos() * psi.sin() + phi.sin() * theta.sin() * psi.cos(),  phi.cos() * psi.cos() + phi.sin() * theta.sin() * psi.sin(), phi.sin() * theta.cos(),    0.],
                    [  phi.sin() * psi.sin() + phi.cos() * theta.sin() * psi.cos(), -phi.sin() * psi.cos() + phi.cos() * theta.sin() * psi.sin(), phi.cos() * theta.cos(),    0.],
                    [                                                           0.,                                                           0.,                      0.,  1f32],
                ]
            }; */

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
                matrix: matrix,
                perspective: perspective,
            };

            frame.clear_color(0., 0., 0., 0.);
            frame
                .draw(
                   (&self.four_vertex_buffer,
                    &self.four_per_instance.per_instance().unwrap()),
                    &self.four_index_buffer,
                    &self.program,
                    &uniforms,
                    &Default::default(),
                )
                .unwrap();
            frame
                .draw(
                   (&self.six_vertex_buffer,
                    &self.six_per_instance.per_instance().unwrap()),
                    &self.six_index_buffer,
                    &self.program,
                    &uniforms,
                    &Default::default(),
                )
                .unwrap();
            frame
                .draw(
                   (&self.eight_vertex_buffer,
                    &self.eight_per_instance.per_instance().unwrap()),
                    &self.eight_index_buffer,
                    &self.program,
                    &uniforms,
                    &Default::default(),
                )
                .unwrap();
            frame
                .draw(
                   (&self.ten_vertex_buffer,
                    &self.ten_per_instance.per_instance().unwrap()),
                    &self.ten_index_buffer,
                    &self.program,
                    &uniforms,
                    &Default::default(),
                )
                .unwrap();
            frame
                .draw(
                   (&self.twelve_vertex_buffer,
                    &self.twelve_per_instance.per_instance().unwrap()),
                    &self.twelve_index_buffer,
                    &self.program,
                    &uniforms,
                    &Default::default(),
                )
                .unwrap();
            frame
                .draw(
                   (&self.twenty_vertex_buffer,
                    &self.twenty_per_instance.per_instance().unwrap()),
                    &self.twenty_index_buffer,
                    &self.program,
                    &uniforms,
                    &Default::default(),
                )
                .unwrap();
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

            self.setup_click();
            self.setup_flicking();
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
            self.renderer.borrow().as_ref().unwrap().draw();
            false
        }

        /*
        fn setup_click(&self) {
            let click_controller = gtk::GestureClick::new();

            click_controller.connect_pressed(|click_controller, _, x, y| {
                let dice_area = click_controller
                    .widget()
                    .dynamic_cast::<super::DiceArea>()
                    .expect("pressed event is not on the Dice Area");

                dice_area.remove()
            });

            self.obj().add_controller(click_controller);
        }

        */


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
        let imp = self.imp();
        let obj = self.obj();

        let binding = imp.renderer.borrow();
        let mut roll = binding.as_ref().unwrap().roll.borrow_mut();
        *roll = Some(Instant::now());
    }

    pub fn add_four(&self) {
        let imp = self.imp();
        let renderer = imp.renderer.borrow();
        let dice = renderer.as_ref().unwrap().roll.borrow_mut();
    }

    pub fn add_six(&self) {}
    pub fn add_ten(&self) {}
    pub fn add_twelve(&self) {}
    pub fn add_twenty(&self) {}

    pub fn roll(&self) {

    }

    pub fn remove_die(&self, x: f64, y: f64) -> AnimatedRemove {
        let remove = AnimatedRemove::new();

        // Shamelessly stolen from confetti snapshot
        let frame_clock = self.frame_clock().unwrap();

        frame_clock.connect_update(clone!(@weak self as this, @weak exp => move |clock| {
            match remove.update(clock) {
                ControlFlow::Continue => {
                    this.queue_draw();
                },
                ControlFlow::Break => {
                    this.imp().dice.borrow_mut().remove();
                    clock.end_updating();
                }
            }
        }));

        self.imp().dice.borrow_mut().insert(remove.clone());

        frame_clock.begin_updating();
    }

    pub fn start_tick(&self) {
        self.add_tick_callback(|s, _| {
            s.queue_draw();
            glib::ControlFlow::Continue
        });
    }
}

