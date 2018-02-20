pub type Vector3 = [f32; 3];
pub type Vector2 = [f32; 2];

pub enum Angle {
    Deg(f32),
    Rad(f32)
}

pub mod matrix4 {

    use std::f64::consts::PI;
    use linalg::*;

    pub type Matrix4 = [[f32; 4]; 4];

    pub fn mat4_ident() -> Matrix4 {
        [
            [1.0,0.0,0.0,0.0],
            [0.0,1.0,0.0,0.0],
            [0.0,0.0,1.0,0.0],
            [0.0,0.0,0.0,1.0]
        ]
    }
    
    pub fn mat4_mul(a: Matrix4, b: Matrix4) -> Matrix4 {
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

    pub fn mat4_vec_mul(mats: Vec<Matrix4>) -> Matrix4 {
        let mut result = mat4_ident();
        for m in mats {
            result = mat4_mul(result, m);
        }
        result
            
    }

    pub fn mat4_translate(amount: Vector3) -> Matrix4 {
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

    pub fn mat4_rotate(angle: Angle, dir: Vector3) -> Matrix4 {
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

    pub fn mat4_scale(amount: Vector3) -> Matrix4 {
        let x = amount[0];
        let y = amount[1];
        let z = amount[2];
        [
            [  x, 0.0, 0.0, 0.0],
            [0.0,   y, 0.0, 0.0],
            [0.0, 0.0,   z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }
}
