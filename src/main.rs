#[macro_use]
extern crate glium;

extern crate image;

use glium::{glutin,Surface};
use std::io::Cursor;

#[derive(Copy,Clone)]
struct Vertex {
    position: [ f32; 3 ],
    uvs: [ f32; 2 ],
}

fn mat4_ident() -> [[f32 ; 4]; 4] {
    [
        [1.0,0.0,0.0,0.0],
        [0.0,1.0,0.0,0.0],
        [0.0,0.0,1.0,0.0],
        [0.0,0.0,0.0,1.0]
    ]
}

fn mat4_add(a: [[f32; 4]; 4], b: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut newmat = [[0.0; 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            newmat[i][j] = 0.0;
            for k in 0..4 {
                newmat[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    newmat
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

fn main() {

    // let tst1 = mat4_ident();
    let tst1 = [
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0, 1.0],
        [0.0, 0.0, 1.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let tst2 = [
        [0.0, 1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let tst3 = mat4_add(tst1,tst2);

    println!("Test add two ident mat4: {:?}", tst3);

    return;

    implement_vertex!(Vertex, position, uvs);

    let mut event_loop = glium::glutin::EventsLoop::new();

    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title("Hello world!");

    let context = glium::glutin::ContextBuilder::new();

    let display = glium::Display::new(window, context, &event_loop).unwrap();

    let shape = make_square(1.0,1.0,0.0);

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

        let mat = mat4_ident();

        let uniforms = uniform! {
            matrix: [
                [t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ 0.0 , 0.0, 0.0, 1.0f32],
            ],
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
