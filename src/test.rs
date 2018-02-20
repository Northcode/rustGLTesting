#[cfg(test)]
mod tests {

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
    }

}
