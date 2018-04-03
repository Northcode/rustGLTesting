#[macro_use]
extern crate glium;

extern crate image;

extern crate tobj;

mod linalg;

use glium::{glutin,Surface};
use std::io::Cursor;

use linalg::*;
use linalg::matrix4::*;

use std::path::Path;

#[derive(Copy,Clone)]
struct Vertex {
    position: Vector3,
    uvs: Vector2,
    normal: Vector3,
}

impl std::fmt::Debug for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vertex {{ position: {:?}, uvs: {:?}, normal: {:?}", self.position, self.uvs, self.normal)
    }
}

// * Vertex making functions

fn make_square(x: f32, y: f32) -> Vec<Vertex> {
    let nz = 1.0;
    let mut square = vec![
        Vertex { position: [0.0,0.0,0.0], uvs: [0.0,0.0], normal: [0.0,0.0,nz] },
        Vertex { position: [x,0.0,0.0],   uvs: [1.0,0.0], normal: [0.0,0.0,nz] },
        Vertex { position: [0.0,y,0.0],   uvs: [0.0,1.0], normal: [0.0,0.0,nz] },
        Vertex { position: [0.0,y,0.0],   uvs: [0.0,1.0], normal: [0.0,0.0,nz] },
        Vertex { position: [x,0.0,0.0],   uvs: [0.0,0.0], normal: [0.0,0.0,nz] },
        Vertex { position: [x,y,0.0],     uvs: [1.0,1.0], normal: [0.0,0.0,nz] },
    ];
    transform_vertecies(mat4_translate([-x/2.0,-y/2.0,0.0]), &mut square);
    square
}


fn transform_vertecies(mat: Matrix4, vertecies: &mut Vec<Vertex>) {
    let notransmat = mat4_mul(mat, [
        [1.0, 1.0, 1.0, 1.0],
        [1.0, 1.0, 1.0, 1.0],
        [1.0, 1.0, 1.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    for mut tex in vertecies.iter_mut() {
        tex.position = mat4_mul_vec3(mat, tex.position);
        tex.normal = mat4_mul_vec3(notransmat, tex.normal);
    }
}

fn make_cube(x: f32, y: f32, z: f32) -> Vec<Vertex> {
    let mut cube = vec![];

    let mut front = {
        let mut square = make_square(x,y);
        transform_vertecies(mat4_translate([0.0,0.0,z/2.0]), &mut square);
        square
    };


    let mut left = {
        let mut square = make_square(z,y);
        transform_vertecies(mat4_translate([0.0,0.0,z/2.0]), &mut square);
        transform_vertecies(mat4_rotate(Angle::Deg(90.0),[0.0,1.0,0.0]), &mut square);
        square
    };

    let mut right = {
        let mut square = make_square(z,y);
        transform_vertecies(mat4_translate([0.0,0.0,z/2.0]), &mut square);
        transform_vertecies(mat4_rotate(Angle::Deg(-90.0),[0.0,1.0,0.0]), &mut square);
        square
    };

    let mut back = {
        let mut square = make_square(z,y);
        transform_vertecies(mat4_translate([0.0,0.0,z/2.0]), &mut square);
        transform_vertecies(mat4_rotate(Angle::Deg(180.0),[0.0,1.0,0.0]), &mut square);
        square
    };


    let mut top = {
        let mut square = make_square(z,y);
        transform_vertecies(mat4_translate([0.0,0.0,z/2.0]), &mut square);
        transform_vertecies(mat4_rotate(Angle::Deg(90.0),[1.0,0.0,0.0]), &mut square);
        square
    };

    let mut bottom = {
        let mut square = make_square(z,y);
        transform_vertecies(mat4_translate([0.0,0.0,z/2.0]), &mut square);
        transform_vertecies(mat4_rotate(Angle::Deg(-90.0),[1.0,0.0,0.0]), &mut square);
        square
    };

    cube.append(&mut front);
    cube.append(&mut left);
    cube.append(&mut right);
    cube.append(&mut back);
    cube.append(&mut top);
    cube.append(&mut bottom);

    cube
}

// * Main function

fn main() {

// ** Init window and gl
    implement_vertex!(Vertex, position, uvs, normal);

    let mut event_loop = glium::glutin::EventsLoop::new();

    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title("Hello world!");

    let context = glium::glutin::ContextBuilder::new().with_depth_buffer(24);

    let display = glium::Display::new(window, context, &event_loop).unwrap();

// ** Load a cube
    let mut vertex_data = Vec::new();

    match tobj::load_obj(Path::new("cube.obj")) {
        Ok((models, mats)) => {
            for model in &models {
                let mesh = &model.mesh;
                for idx in &mesh.indices {
                    let i = *idx as usize;
                    let pos = [
                        mesh.positions[3 * i],
                        mesh.positions[3 * i + 1],
                        mesh.positions[3 * i + 2],
                    ];
                    let normal = if !mesh.normals.is_empty() {
                        [
                            mesh.normals[3 * i],
                            mesh.normals[3 * i + 1],
                            mesh.normals[3 * i + 2],
                        ]
                    } else {
                        [0.0, 0.0, 0.0]
                    };
                    let uvs = if !mesh.texcoords.is_empty() {
                        [
                            mesh.texcoords[2 * i],
                            mesh.texcoords[2 * i + 1],
                        ]
                    } else {
                        [0.0,0.0]
                    };
                    vertex_data.push(Vertex {
                        position: pos,
                        uvs: uvs,
                        normal: normal
                    });
                }
            }
        },
        Err(e) => panic!("Failed to load model!"),
    };

    // for ver in &vertex_data {
    //     println!("{:?}", ver);
    // }

// ** Make a shape
    // let shape = {
    //     let mut shape = make_cube(1.0,1.0,1.0);
    //     let mut shape2 = make_cube(0.5,0.5,0.5);
    //     transform_vertecies(mat4_translate([-1.0,0.0,0.0]), &mut shape2);
    //     shape.append(&mut shape2);
    //     shape
    // };

    // for ver in &shape {
    //     println!("{:?}", ver);
    // }
    

    let vertex_buffer = glium::VertexBuffer::new(&display, &vertex_data).unwrap();
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
in vec3 normal;

out vec2 v_uvs;

out vec3 pos;
out vec4 norm;

uniform mat4 matrix;

out mat4 o_mat;

void main() {
    v_uvs = uvs;
    pos = position;
    norm = normalize(matrix * vec4(normal,0.0));
    gl_Position = matrix * vec4(position, 1.0);
}
"#;
    let fragment_shader_src = r#"
#version 140

in vec2 v_uvs;

in vec3 pos;
in vec4 norm;

in mat4 o_mat;

out vec4 color;

uniform sampler2D tex;
uniform vec3 light_pos;

uniform vec4 diffuse_light_direction;
uniform vec3 diffuse_light_color;
uniform vec4 ambient_light;

void main() {
    vec4 diffuse_light = max(dot(norm, diffuse_light_direction), 0.0) * vec4(diffuse_light_color,1.0);
    color = texture(tex, v_uvs) * max(diffuse_light, ambient_light);
}

"#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();



    let mut closed = false;

    let mut t : f32 = 0.0;
    let mut w : f32 = 0.0;

    let speed = 0.05;

    #[derive(Debug)]
    struct KeyState {
        w : bool,
        a : bool,
        s : bool,
        d : bool
    };

    let mut keystate = KeyState { w: false, a: false, s: false, d: false };

// ** Render loop
    while ! closed {
        // t += 0.01;
        // if t > 3.14159 * 2.0 {
        //     t = 0.0;
        // }

        // t = 2.5;

        // let mat = mat4_rotate(Angle::Rad(t),[0.0,0.0,1.0]);
        let mat = mat4_vec_mul(vec![
            mat4_translate([-0.5,-0.5,-0.5]),
            mat4_rotate(Angle::Rad(t), [0.0,1.0,0.0]),
            mat4_rotate(Angle::Rad(w), [1.0,0.0,0.0]),
            mat4_scale([0.5,0.5,0.5]),
        ]);

        let light_ambient : [f32; 4] = [0.5, 0.3, 0.3, 1.0];
        let light_diffuse_color : [f32; 3] = [1.0, 1.0, 1.0];
        let light_diffuse_direction : [f32; 4] = [-1.0, 1.0, 0.0, 0.0];

        let uniforms = uniform! {
            matrix: mat,
            tex: &texture,
            diffuse_light_direction: light_diffuse_direction,
            diffuse_light_color: light_diffuse_color,
            ambient_light: light_ambient,
        };
        
        let mut target = display.draw();
        target.clear_color_and_depth((0.0,0.0,0.0,1.0), 1.0);

        let draw_params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };
        
        target.draw(&vertex_buffer, &indices, &program, &uniforms, &draw_params).unwrap();

        target.finish().unwrap();

        event_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => closed = true,
                    glutin::WindowEvent::KeyboardInput { device_id, input } => {
                        use glutin::VirtualKeyCode;
                        use glutin::ElementState;
                        let val = input.state == ElementState::Pressed;
                        match input.virtual_keycode {
                            Some(VirtualKeyCode::W) => keystate.w = val,
                            Some(VirtualKeyCode::S) => keystate.s = val,
                            Some(VirtualKeyCode::A) => keystate.a = val,
                            Some(VirtualKeyCode::D) => keystate.d = val,
                            _ => ()
                        };
                    }
                    _ => ()
                },
                _ => ()
            }

        });

        if keystate.w {
            w += speed;
        } else if keystate.s {
            w -= speed;
        }
        
        if keystate.d {
            t += speed;
        } else if keystate.a {
            t -= speed;
        }
    }
}
