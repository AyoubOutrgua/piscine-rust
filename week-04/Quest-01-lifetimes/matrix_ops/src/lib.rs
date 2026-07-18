use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

use lalgebra_scalar::*;
use matrix::*;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Wrapper<const W: usize, const H: usize, T>(pub Matrix<W, H, T>);

impl<const W: usize, const H: usize, T> From<[[T; W]; H]> for Wrapper<W, H, T> {
    fn from(input_data: [[T; W]; H]) -> Self {
        Wrapper(Matrix(input_data))
    }
}

impl<const W: usize, const H: usize, T: Add<Output = T> + Debug + Scalar<Item = T> + Copy> Add
    for Wrapper<W, H, T>
{
    type Output = Wrapper<W, H, T>;
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = [[T::zero(); W]; H];
        for i in 0..W {
            for j in 0..H {
                result[i][j] = self.0 .0[i][j] + rhs.0 .0[i][j]
            }
        }
        Wrapper(Matrix(result))
    }
}

impl<const W: usize, const H: usize, T: Sub<Output = T> + Debug + Scalar<Item = T> + Copy> Sub
    for Wrapper<W, H, T>
{
    type Output = Wrapper<W, H, T>;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = [[T::zero(); W]; H];
        for i in 0..W {
            for j in 0..H {
                result[i][j] = self.0 .0[i][j] - rhs.0 .0[i][j]
            }
        }
        Wrapper(Matrix(result))
    }
}

impl<const S: usize, T: Mul<Output = T>  + Add<Output = T> + Debug + Scalar<Item = T> + Copy> Mul
    for Wrapper<S, S, T>
{
    type Output = Wrapper<S, S, T>;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = [[T::zero(); S]; S];
        result[0][0] = self.0.0[0][0] * rhs.0.0[0][0] + self.0.0[0][1] * rhs.0.0[1][0];
        result[0][1] = self.0.0[0][0] * rhs.0.0[0][1] + self.0.0[0][1] * rhs.0.0[1][1];
        result[1][0] = self.0.0[1][0] * rhs.0.0[0][0] + self.0.0[1][1] * rhs.0.0[1][0];
        result[1][1] = self.0.0[1][0] * rhs.0.0[0][1] + self.0.0[1][1] * rhs.0.0[1][1];

        Wrapper(Matrix(result))
    }
}