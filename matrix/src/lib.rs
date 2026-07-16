use lalgebra_scalar::Scalar;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Matrix<const W: usize, const H: usize, T>(pub [[T; W]; H]);

impl<const W: usize, const H: usize, T: Scalar> Matrix<W, H, T> 
where
    T: Scalar<Item = T> + Copy,
{
    pub fn zero() -> Self {
        Matrix([[T::zero(); W]; H])
    }
}

impl<const S: usize, T: Scalar> Matrix<S, S, T> {
    pub fn identity() -> Self 
    where
    T: Scalar<Item = T> + Copy,
{
        let mut mat = [[T::zero(); S]; S];
        for i in 0..S {
            mat[i][i] = T::one();
        }
        Matrix(mat)
    }
}