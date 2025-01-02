use crate::tensor::{BroadcastLayer, Tensor};

impl<T> BroadcastLayer<T> for Tensor<T> {
    fn can_broadcast(&self, other: &Self) -> bool {
        if self.shape.len() != other.shape.len() {
            return false;
        }
        // 각 차원을 뒤에서부터 비교
        self.shape.iter().zip(other.shape.iter()).all(|(&a, &right)| {
            a == right || a == 1 || right == 1
        })
    }

    fn broadcast_shape(&self, other: &Self) -> Vec<usize> {
        self.shape
            .iter()
            .zip(&other.shape)
            .map(|(&left, &right)| std::cmp::max(left, right))
            .collect()
    }

    fn broadcast_op<F>(&self, other: &Self, op: F) -> Option<Self>
    where
        F: Fn(&T, &T) -> T,
    {
        let shape: Vec<usize> = self.broadcast_shape(other);

        let size = shape.iter().product();
        let mut data = Vec::with_capacity(size);

        for idx in 0..size {
            let (self_idx, other_idx) = self.calculate_broadcast_indices(other, idx, &shape)?;
            data.push(op(&self.data[self_idx], &other.data[other_idx]));
        }

        Some(Self { data, shape })
    }

    fn into_broadcast_op<F>(self, other: Self, op: F) -> Option<Self>
    where
        F: Fn(&T, &T) -> T
    {
        let shape: Vec<usize> = self.broadcast_shape(&other);
        let size = shape.iter().product();
        let mut data = Vec::with_capacity(size);

        for idx in 0..size {
            let (self_idx, other_idx) = self.calculate_broadcast_indices(&other, idx, &shape)?;
            data.push(op(&self.data[self_idx], &other.data[other_idx]));
        }

        Some(Self { data, shape })
    }

    fn calculate_broadcast_indices(&self, other: &Self, idx: usize, shape: &[usize]) -> Option<(usize, usize)> {
        let mut remaining = idx;
        let mut self_idx = 0;
        let mut other_idx = 0;

        for (i, (_, (self_dim, other_dim))) in shape.into_iter()
            .zip(self.shape.iter().zip(&other.shape))
            .enumerate()
        {
            let pos = remaining / shape[i+1..].iter().product::<usize>().max(1);
            remaining %= shape[i+1..].iter().product::<usize>().max(1);

            self_idx = self_idx * self_dim + if self_dim == &1 { 0 } else { pos };
            other_idx = other_idx * other_dim + if other_dim == &1 { 0 } else { pos };
        }

        Some((self_idx, other_idx))
    }
}