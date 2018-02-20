#[cfg(test)]
#[macro_use]
extern crate quickcheck;

mod linalg;

#[cfg(test)]
mod tests {

    use linalg::*;
    use linalg::matrix4::*;

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
