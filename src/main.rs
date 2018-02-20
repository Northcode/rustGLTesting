#[macro_use]
extern crate glium;

extern crate image;

mod linalg;

use glium::{glutin,Surface};
use std::io::Cursor;

use linalg::*;
use linalg::matrix4::*;

#[derive(Copy,Clone)]
struct Vertex {
    position: Vector3,
    uvs: Vector2,
}


fn make_square(x: f32, y: f32, z: f32) -> Vec<Vertex> {
    let square = vec![
        Vertex { position: [0.0,0.0,z], uvs: [0.0,0.0] },
        Vertex { position: [x,0.0,z], uvs: [1.0,0.0] },
        Vertex { position: [0.0,y,z], uvs: [0.0,1.0] },
        Vertex { position: [0.0,y,z], uvs: [0.0,1.0] },
        Vertex { position: [x,0.0,z], uvs: [0.0,0.0] },
        Vertex { position: [x,y,z], uvs: [1.0,1.0] },
    ];
    square
}

fn transpose_vertex(vert: &mut Vertex, amnt: Vector3) {
    let x = vert.position[0];
    let y = vert.position[1];
    let z = vert.position[2];

    let ax = amnt[0];
    let ay = amnt[1];
    let az = amnt[2];

    vert.position = [x + ax, y + ay, z + az];
}

fn main() {

    implement_vertex!(Vertex, position, uvs);

    let mut event_loop = glium::glutin::EventsLoop::new();

    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title("Hello world!");

    let context = glium::glutin::ContextBuilder::new();

    let display = glium::Display::new(window, context, &event_loop).unwrap();



    let mut shape = make_square(1.0,1.0,0.0);
    let mut square2 = make_square(0.5,0.5,1.0);
    for mut tex in &mut square2 {
        transpose_vertex(tex, [-0.5, -0.5, 0.0]);
    }
    shape.append(&mut square2);

    

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);



    let image = image::load(Cursor::new(&include_bytes!("/home/andreas/Pictures/skin.jpg")[..]), image::JPEG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    let texture = glium::texture::Texture2d::new(&display, image).unwrap();


    let vertex_shader_src = r#"
#version 140

in vec3 position;
in vec2 uvs;

out vec2 v_uvs;

out vec3 pos;

uniform mat4 matrix;

void main() {
    v_uvs = uvs;
    pos = position;
    gl_Position = matrix * vec4(position, 1.0);
}
"#;
    let fragment_shader_src = r#"
#version 140

in vec2 v_uvs;

in vec3 pos;

out vec4 color;

uniform sampler2D tex;

void main() {
    color = texture(tex, v_uvs) * vec4(pos,0);
}

"#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();


    let mut closed = false;

    let mut t : f32 = -0.5;

    while ! closed {
        t += 0.02;
        if t > 3.14159 * 2.0 {
            t = 0.0;
        }

        // let mat = mat4_rotate(Angle::Rad(t),[0.0,0.0,1.0]);
        let mat = mat4_vec_mul(vec![
            mat4_translate([-0.5,-0.5,0.0]),
            mat4_scale([0.5,0.5,0.5]),
            mat4_rotate(Angle::Rad(t), [0.0,0.0,1.0])]);

        let uniforms = uniform! {
            matrix: mat,
            tex: &texture
        };
        
        let mut target = display.draw();
        target.clear_color(0.0,0.0,0.0,1.0);

        target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();

        target.finish().unwrap();

        event_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => closed = true,
                    _ => ()
                },
                _ => ()
            }
        });
    }
}
