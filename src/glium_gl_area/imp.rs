use std::{cell::RefCell, rc::Rc};

use glium::{
    implement_vertex, index::PrimitiveType, program, uniform, Frame, IndexBuffer, Surface,
    VertexBuffer,
};
use gtk::{glib, prelude::*, subclass::prelude::*};
use std::time::{Duration, Instant};
use std::f32::consts::PI;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

pub struct Renderer {
    context: Rc<glium::backend::Context>,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
    program: glium::Program,
}

impl Renderer {
    fn new(context: Rc<glium::backend::Context>) -> Self {
        // The following code is based on glium's triangle example:
        // https://github.com/glium/glium/blob/2ff5a35f6b097889c154b42ad0233c6cdc6942f4/examples/triangle.rs
        let vertex_buffer = VertexBuffer::new(
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

        let indices: [u16; 36] = [
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


        let index_buffer =
            IndexBuffer::new(&context, PrimitiveType::TrianglesList, &indices).unwrap();

        // let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

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

                    in vec3 position;
                    in vec3 color;
                    out vec3 vColor;
                    void main() {
                        gl_Position = vec4(position, 1.0) * matrix * perspective;
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

        Renderer {
            context,
            vertex_buffer,
            index_buffer,
            program,
        }
    }

    fn draw(&self) {
        let mut frame = Frame::new(
            self.context.clone(),
            self.context.get_framebuffer_dimensions(),
        );

        println!("Surface has_depth_buffer: {}",frame.has_depth_buffer());

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

        let theta = PI / 12.;
        let psi = PI / 6.;
        let phi = PI / 3.;

        let uniforms = uniform! {
            matrix: [
                [theta.cos() * psi.cos(), theta.cos() * psi.sin(), -theta.sin(), 0.],
                [-phi.cos() * psi.sin() + phi.sin() * theta.sin() * psi.cos(), phi.cos() * psi.cos() + phi.sin() * theta.sin() * psi.sin(), phi.sin() * theta.cos(), 0.],
                [phi.sin() * psi.sin() + phi.cos() * theta.sin() * psi.cos(), -phi.sin() * psi.cos() + phi.cos() * theta.sin() * psi.sin(), phi.cos() * theta.cos(), 0.],
                [0., 0., 0., 1f32]
            ],
            /* matrix: [
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1f32]
            ], */
            perspective: perspective,
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            ..Default::default()
        };

        frame.clear_color_and_depth((0., 0., 0., 0.), 1.0);
        frame
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
                &self.program,
                &uniforms,
                &params,
            )
            .unwrap();
        frame.finish().unwrap();
    }

    pub fn six_spin(&self) {
        let start_time = Instant::now();

        loop {
            let elapsed = start_time.elapsed();
            // println!("{}", elapsed.as_secs_f32());

            if elapsed >= Duration::new(4, 0) {
                break;
            }

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

            let elapsed_f32 = elapsed.as_secs_f32();
            let secs_to_rads = PI / 4.0;
            let theta = elapsed_f32 * secs_to_rads;
            println!("{}",theta);
            let psi = theta * 2.0;
            let phi = theta / 2.0;

            let uniforms = uniform! {
                matrix: [
                    [theta.cos() * psi.cos(), theta.cos() * psi.sin(), -theta.sin(), 0.],
                    [-phi.cos() * psi.sin() + phi.sin() * theta.sin() * psi.cos(), phi.cos() * psi.cos() + phi.sin() * theta.sin() * psi.sin(), phi.sin() * theta.cos(), 0.],
                    [phi.sin() * psi.sin() + phi.cos() * theta.sin() * psi.cos(), -phi.sin() * psi.cos() + phi.cos() * theta.sin() * psi.sin(), phi.cos() * theta.cos(), 0.],
                    [0., 0., 0., 1f32]
                ],
                perspective: perspective,
            };

            frame.clear_color(0., 0., 0., 0.);
            frame
                .draw(
                    &self.vertex_buffer,
                    &self.index_buffer,
                    &self.program,
                    &uniforms,
                    &Default::default(),
                )
                .unwrap();
            frame.finish().unwrap();
        }
    }
}

#[derive(Default)]
pub struct GliumGLArea {
    pub renderer: RefCell<Option<Renderer>>,
}

#[glib::object_subclass]
impl ObjectSubclass for GliumGLArea {
    const NAME: &'static str = "GliumGLArea";
    type Type = super::GliumGLArea;
    type ParentType = gtk::GLArea;
}

impl ObjectImpl for GliumGLArea {}

impl WidgetImpl for GliumGLArea {
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

impl GLAreaImpl for GliumGLArea {
    // Is a glib::Propagation in post 0.7 gtk, need to figure out how to update
    fn render(&self, _context: &gtk::gdk::GLContext) -> bool {
        self.renderer.borrow().as_ref().unwrap().draw();
        false
    }
}
