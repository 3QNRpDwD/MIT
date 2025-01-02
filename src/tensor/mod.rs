use std::ops::{Add, Div, Mul, Sub};
mod ops;
mod broadcast;

#[derive(Debug, Default)]
pub struct Tensor<T> {
    pub(crate) data: Vec<T>,
    pub(crate) shape: Vec<usize>,
}

pub trait DefaultLayer<T> where T: Sized{
    fn new(data: T) -> Self;
    fn get(&self, indices: &[usize]) -> Option<&T>;
    fn index(&self, indices: &[usize]) -> Option<usize>;
}

pub trait OpsLayer<T> {
    // 사칙연산
    fn add(&self, other: &Tensor<T>)
           -> Option<Self> where T: Add<Output = T>, Self: Sized;
    fn sub(&self, other: &Tensor<T>)
           -> Option<Self> where T: Sub<Output = T>, Self: Sized;
    fn div(&self, other: &Tensor<T>)
           -> Option<Self> where T: Div<Output = T>, Self: Sized;
    fn mul(&self, other: &Tensor<T>)
           -> Option<Self> where T: Mul<Output = T>, Self: Sized;

    // 텐서 & 스칼라 연산
    fn add_scalar(&self, other: &Tensor<T>)
                  -> Option<Self> where T: Add<Output = T>, Self: Sized;
    fn mul_scalar(&self, other: &Tensor<T>)
                  -> Option<Self> where T: Mul<Output = T>, Self: Sized;
    fn sub_scalar(&self, other: &Tensor<T>)
                  -> Option<Self> where T: Sub<Output = T>, Self: Sized;
    fn div_scalar(&self, other: &Tensor<T>)
                  -> Option<Self> where T: Div<Output = T>, Self: Sized;

    // 스칼라 & 텐서 연산
    fn scalar_sub(&self, other: &Tensor<T>)
                  -> Option<Self> where T: Sub<Output = T>, Self: Sized;
    fn scalar_div(&self, other: &Tensor<T>)
                  -> Option<Self> where T: Div<Output = T>, Self: Sized;
}

pub trait BroadcastLayer<T> {
    fn can_broadcast(&self, other: &Self) -> bool;
    fn broadcast_shape(&self, other: &Self) -> Vec<usize>;
    fn broadcast_op<F>(&self, other: &Self, op: F) -> Option<Self>
    where
        F: Fn(&T, &T) -> T,
        Self: Sized;
    fn into_broadcast_op<F>(self, other: Self, op: F) -> Option<Self>
    where
        F: Fn(&T, &T) -> T,
        Self: Sized;
    fn calculate_broadcast_indices(&self, other: &Self, idx: usize, shape: &[usize]) -> Option<(usize, usize)>;
}
#[cfg(test)]
mod tests {
    use crate::tensor::{DefaultLayer, Tensor};
    use super::*;

    // Option<T>의 결과를 테스트하는 헬퍼 함수
    pub fn assert_tensor_eq<T: PartialEq + std::fmt::Debug>(
        result: Option<Tensor<T>>,
        expected_data: Vec<T>,
        expected_shape: Vec<usize>
    ) {
        let tensor = result.unwrap();
        debug_assert_eq!(tensor.data, expected_data);
        debug_assert_eq!(tensor.shape, expected_shape);
    }

    #[test]
    fn tensor_arithmetic_operations() {
        let t1: Tensor<Vec<f32>> = DefaultLayer::new(vec![1.0, 2.0]);
        let t2: Tensor<Vec<f32>> = DefaultLayer::new(vec![3.0, 4.0]);

        assert_tensor_eq(t1.add(&t2), vec![6.0, 6.0, 6.0, 6.0], vec![2, 2]);
        assert_tensor_eq(t1.sub(&t2), vec![2.0, 2.0, 2.0, 2.0], vec![2, 2]);
        assert_tensor_eq(t1.mul(&t2), vec![8.0, 8.0, 8.0, 8.0], vec![2, 2]);
        assert_tensor_eq(t1.div(&t2), vec![2.0, 2.0, 2.0, 2.0], vec![2, 2]);
    }
}