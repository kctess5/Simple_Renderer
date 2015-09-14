#[macro_use]
#[allow(dead_code)]
extern crate glium;

use glium::{Surface, glutin, DisplayBuild};

mod obj_loader;
mod support;
mod color;
mod light;
mod camera;

fn main() {
    // building the display, ie. the main object
    let display = glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .build_glium()
        .unwrap();

    // load vertex buffer from std in
    let vertex_buffer = obj_loader::ObjBuffer::load_std().to_vertex_buffer(&display);

    // // the program
    let program = program!(&display,
        140 => {
            vertex: "
                #version 140
                uniform mat4 persp_matrix;
                uniform mat4 view_matrix;
                in vec3 position;
                in vec3 normal;
                out vec3 v_position;
                out vec3 v_normal;
                void main() {
                    v_position = position;
                    v_normal = normal;
                    gl_Position = persp_matrix * view_matrix * vec4(v_position, 1.0);
                }
            ",

            fragment: "
                #version 140
                uniform vec3 color_matrix;
                uniform vec3 light;
                in vec3 v_normal;
                out vec4 f_color;
                void main() {
                    float lum = max(dot(normalize(v_normal), normalize(light)), 0.0);
                    vec3 color = (0.2 + 0.8 * lum) * color_matrix;
                    f_color = vec4(color, 1.0);
                }
            ",
        },
    ).unwrap();

    //
    let mut camera = camera::CameraState::new();
    let mut color = color::Color::new();
    let mut light = light::Light::new();
    let (mut t, mut i) = (0.0f32, 0);

    // the main loop
    support::start_loop(|| {
        t += 0.002;
        i += 1;

        if i % 100 == 0 {
            // occasionally print t
            println!("t: {}",t);
        }

        camera.update();

        // building the uniforms
        let uniforms = uniform! {
            persp_matrix: camera.get_perspective(),
            view_matrix: camera.get_view(),
            color_matrix: color.serialize(),
            light: light.serialize(),
        };

        // draw parameters
        let params = glium::DrawParameters {
            depth_test: glium::DepthTest::IfLess,
            depth_write: true,
            .. Default::default()
        }; 

        // drawing a frame
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        target.draw(&vertex_buffer,
                    &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &program, &uniforms, &params).unwrap();
        target.finish().unwrap();

        // polling and handling the events received by the window
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return support::Action::Stop,
                ev => {
                	camera.process_input(&ev);
	                color.process_input(&ev);
	                light.process_input(&ev);
	            },
            }
        }
        support::Action::Continue
    });
}