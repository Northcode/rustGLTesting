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

// * Vertex making functions

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

fn make_square_centered(x: f32, y: f32) -> Vec<Vertex> {
    let mut square = vec![
        Vertex { position: [0.0,0.0,0.0], uvs: [0.0,0.0] },
        Vertex { position: [x,0.0,0.0], uvs: [1.0,0.0] },
        Vertex { position: [0.0,y,0.0], uvs: [0.0,1.0] },
        Vertex { position: [0.0,y,0.0], uvs: [0.0,1.0] },
        Vertex { position: [x,0.0,0.0], uvs: [0.0,0.0] },
        Vertex { position: [x,y,0.0], uvs: [1.0,1.0] },
    ];
    transform_vertecies(mat4_translate([-x/2.0,-y/2.0,0.0]), &mut square);
    square
}


fn transform_vertecies(mat: Matrix4, vertecies: &mut Vec<Vertex>) {
    for mut tex in vertecies.iter_mut() {
        tex.position = mat4_mul_vec3(mat, tex.position);
    }
}

fn make_cube_face(angle: Angle, x: f32, y: f32, z: f32, dir: Vector3) -> Vec<Vertex> {
    let mut square = make_square(x,y,0.0);
    transform_vertecies(mat4_rotate(angle, dir), &mut square);
    transform_vertecies(mat4_translate([x,0.0,0.0]), &mut square);
    square
}

fn make_cube(x: f32, y: f32, z: f32) -> Vec<Vertex> {
    let mut cube = vec![];

    let mut front = {
        let mut square = make_square_centered(x,y);
        transform_vertecies(mat4_translate([0.0,0.0,z/2.0]), &mut square);
        square
    };

    let mut left = {
        let mut square = make_square_centered(z,y);
        transform_vertecies(mat4_translate([0.0,0.0,z/2.0]), &mut square);
        transform_vertecies(mat4_rotate(Angle::Deg(90.0),[0.0,1.0,0.0]), &mut square);
        square
    };

    let mut right = {
        let mut square = make_square_centered(z,y);
        transform_vertecies(mat4_translate([0.0,0.0,z/2.0]), &mut square);
        transform_vertecies(mat4_rotate(Angle::Deg(-90.0),[0.0,1.0,0.0]), &mut square);
        square
    };
    

    cube.append(&mut front);
    cube.append(&mut left);
    cube.append(&mut right);

    cube
}

fn transpose_vertex(vert: &mut Vertex, amnt: Vector3) {
    let mat = mat4_translate(amnt);

    vert.position = mat4_mul_vec3(mat, vert.position);
}

// * Main function

fn main() {

// ** Init window and gl
    implement_vertex!(Vertex, position, uvs);

    let mut event_loop = glium::glutin::EventsLoop::new();

    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title("Hello world!");

    let context = glium::glutin::ContextBuilder::new()
        .with_depth_buffer(24);

    let display = glium::Display::new(window, context, &event_loop).unwrap();



// ** Make a shape
    let mut shape = make_cube(1.0,1.0,1.0);

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);



    let image = image::load(Cursor::new(&include_bytes!("../textures/skin.jpg")[..]), image::JPEG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    let texture = glium::texture::Texture2d::new(&display, image).unwrap();


// ** Shaders
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
    color = texture(tex, v_uvs);
}

"#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();


    let mut closed = false;

    let mut t : f32 = -0.5;

// ** Render loop
    while ! closed {
        t += 0.02;
        if t > 3.14159 * 2.0 {
            t = 0.0;
        }

        // let mat = mat4_rotate(Angle::Rad(t),[0.0,0.0,1.0]);
        let mat = mat4_vec_mul(vec![
            mat4_translate([-0.5,-0.5,0.0]),
            mat4_scale([0.5,0.5,0.5]),
            mat4_rotate(Angle::Rad(t), [1.0,0.0,0.0]),
            mat4_rotate(Angle::Rad(t), [0.0,1.0,0.0]),
            mat4_rotate(Angle::Rad(t), [0.0,0.0,1.0])]);

        let uniforms = uniform! {
            matrix: mat,
            tex: &texture
        };
        
        let mut target = display.draw();
        target.clear_color_and_depth((0.0,0.0,0.0,1.0), 1.0);

        let draw_params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };
                    
        target.draw(&vertex_buffer, &indices, &program, &uniforms, &draw_params).unwrap();

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
