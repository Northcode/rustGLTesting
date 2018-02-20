#[macro_use]
extern crate glium;

extern crate image;

use glium::{glutin,Surface};
use std::io::Cursor;
use std::f64::consts::PI;

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

fn mat4_mul(a: [[f32; 4]; 4], b: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
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

fn mat4_translate(amount: [f32; 3]) -> [[f32; 4]; 4] {
    let x = amount[0];
    let y = amount[1];
    let z = amount[2];
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [  x,   y,   z, 1.0],
    ]
}

enum Angle {
    Deg(f32),
    Rad(f32)
}

fn mat4_rotate(angle: Angle, dir: [f32; 3]) -> [[f32; 4]; 4] {
    let angle_rad = match angle {
        Angle::Rad(r) => r,
        Angle::Deg(d) => PI as f32 * d / 180.0
    };

    let mut s = angle_rad.sin();
    let c = angle_rad.cos();


    let mut x = dir[0];
    let mut y = dir[1];
    let mut z = dir[2];


    if x == 1.0 && y == 0.0 && z == 0.0 {
        if x < 0.0 {
            s = -s;
        }

        [
            [1.0,0.0,0.0,0.0],
            [0.0,  c, -s,0.0],
            [0.0,  s,  c,0.0],
            [0.0,0.0,0.0,1.0],
        ]
    } else if x == 0.0 && y == 1.0 && z == 0.0 {
        if y < 0.0 {
            s = -s;
        }

        [
            [  c,0.0,  s,0.0],
            [0.0,1.0,0.0,0.0],
            [ -s,0.0,  c,0.0],
            [0.0,0.0,0.0,1.0],
        ]
    } else if x == 0.0 && y == 0.0 && z == 1.0 {
        if z < 0.0 {
            s = -s;
        }

        [
            [  c, -s,0.0,0.0],
            [  s,  c,0.0,0.0],
            [0.0,0.0,1.0,0.0],
            [0.0,0.0,0.0,1.0],
        ]
    } else {
        let len = (x*x + y*y + z*z).sqrt();

        if len != 1.0 {
            let rlen = 1.0 / len;
            x *= rlen;
            y *= rlen;
            z *= rlen;
        }

        let nc = 1.0 - c;
        let xy = x*y;
        let yz = y*z;
        let zx = z*x;

        let xs = x * s;
        let ys = y * s;
        let zs = z * s;

        [
            [x*x*nc +  c, xy  * nc + zs, zx * nc - ys, 0.0],
            [xy *nc - zs, y*y * nc +  c, yz * nc + xs, 0.0],
            [zx *nc + ys, yz  * nc + xs, z*z* nc -  c, 0.0],
            [0.0,0.0,0.0,1.0],
        ]
    }
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

        // let mat = mat4_rotate(Angle::Rad(t),[0.0,0.0,1.0]);
        let mat = mat4_mul(
            mat4_translate(-0.5,-0.5,0.0), 
            mat4_rotate(Angle::Rad(t), [0.0,0.0,1.0]));

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
