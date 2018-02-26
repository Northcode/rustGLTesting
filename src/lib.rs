#[cfg(test)]
#[macro_use]
extern crate quickcheck;

mod linalg;

#[cfg(test)]
mod tests {

    use linalg::*;
    use linalg::matrix4::*;

    use std::f64::consts::PI;

    #[test]
    fn test_mat4_ident() {
        assert_eq!(mat4_ident(),[
            [1.0,0.0,0.0,0.0],
            [0.0,1.0,0.0,0.0],
            [0.0,0.0,1.0,0.0],
            [0.0,0.0,0.0,1.0]
        ]);
    }

    #[test]
    fn test_mat4_mul() {
        let a = mat4_ident();
        let b = mat4_ident();

        let prod = mat4_mul(a,b);

        assert_eq!(a,prod);

        let a = [
           [0.0, 1.0, 0.0, 0.0],
           [1.0, 0.0, 0.0, 0.0],
           [0.0, 0.0, 1.0, 0.0],
           [0.0, 0.0, 0.0, 1.0]
        ];

        let b = [
            [1.0, 0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 1.0],
            [0.0, 0.0, 1.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        let prod = mat4_mul(a,b);

        let expected = [
            [0.0, 1.0, 0.0, 1.0],
            [1.0, 0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        assert_eq!(prod,expected);
    }

    fn round_vec(v: Vector3) -> Vector3 {
        let precision = 10000.0;
        let mut res = [0.0; 3];
        res[0] = (v[0] * precision).round() / precision;
        res[1] = (v[1] * precision).round() / precision;
        res[2] = (v[2] * precision).round() / precision;
        return res;
    }

    #[test]
    fn test_mat4_mul_vec3() {
        let mat = mat4_rotate(Angle::Deg(90.0), [0.0,1.0,0.0]);

        let cases = vec![
            [[1.0,0.0,0.0], [0.0,0.0,1.0]],
            [[0.0,0.0,1.0], [-1.0,0.0,0.0]],
            [[-1.0,0.0,0.0], [0.0,0.0,-1.0]],
            [[0.0,0.0,-1.0], [1.0,0.0,0.0]],
            [[0.5,0.0,0.0], [0.0,0.0,0.5]],
        ];

        for case in cases.iter() {
            let prod = round_vec(mat4_mul_vec3(mat,case[0]));
            assert_eq!(prod,case[1]);
        }

        let mat = mat4_rotate(Angle::Deg(90.0), [1.0,0.0,0.0]);

        let cases = vec![
            [[0.0,1.0,0.0], [0.0,0.0,-1.0]],
            [[0.0,-1.0,0.0], [0.0,0.0,1.0]],
        ];

        for case in cases.iter() {
            let prod = round_vec(mat4_mul_vec3(mat,case[0]));
            assert_eq!(prod,case[1]);
        }

    }


    quickcheck! {
        fn test_mat4_translate(x: f32, y: f32, z: f32) -> bool {
            mat4_translate([x,y,z]) == [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [  x,   y,   z, 1.0],
            ]
        }
    }
}
