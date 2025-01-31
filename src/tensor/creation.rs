use crate::{MlError, MlResult};
use crate::tensor::{TensorBase, Tensor, TensorError};

impl<Type: std::fmt::Debug> TensorBase<Type> for Tensor<Type> {
    fn new(data: Vec<Vec<Type>>) -> MlResult<Box<dyn TensorBase<Type>>>  {
        let shape = vec![data.len(), data[0].len()];
        let data: Vec<Type> = data.into_iter().flatten().collect();

        Ok(Box::new(Self {
            data,
            shape,
            grad: None,
            grad_fn: None,
            requires_grad: false,
            power: None,
            topk: None,
            matmax: None,
        }))
    }

    fn from_vec(data: Vec<Type>, shape: &[usize]) -> MlResult<Box<dyn TensorBase<Type>>> {
        let expected_len: usize = shape.iter().product();
        if data.len() != expected_len {
            return Err(MlError::TensorError(TensorError::InvalidDataLength {
                expected: expected_len,
                got: data.len(),
            }));
        }

        Ok(Box::new(Self {
            data,
            shape: shape.to_vec(),
            grad: None,
            grad_fn: None,
            requires_grad: false,
            power: None,
            topk: None,
            matmax: None,
        }))
    }

    fn shape(&self) -> &[usize] {
        &self.shape
    }

    fn data(&self) -> &[Type] {
        &self.data
    }

    fn power(&self) -> f32 {
        self.power.unwrap()
    }

    fn topk(&self) -> (usize, bool) {
        self.topk.unwrap()
    }

    fn matmax(&self) -> (Option<i32>, bool) {
        self.matmax.unwrap()
    }

    fn set_power(&mut self, exponent: f32) {
        self.power = Some(exponent);
    }

    fn set_topk(&mut self, k: usize, sorted: bool) {
        self.topk = Some((k, sorted));
    }

    fn set_matmax(&mut self, dim: Option<i32>, keepdim: bool) {
        self.matmax = Some((dim, keepdim));
    }

    fn get(&self, indices: &[usize]) -> Option<&Type> {
        self.data.get(self.index(indices)?)
    }

    fn index(&self, indices: &[usize]) -> Option<usize> {
        if indices.len() != self.shape.len() {
            return None;
        }
        Some(
            indices
                .iter()
                .zip(&self.shape)
                .fold(0, |acc, (&i, &dim)| acc * dim + i),
        )
    }

    /// Verifies if two tensors can perform element-wise operations
    ///
    /// # Arguments
    /// * `other` - The tensor to compare shapes with
    ///
    /// # Returns
    /// * `Ok(())` if the shapes match
    /// * `Err(MlError::TensorError)` if shapes don't match
    fn chk_shape(&self, other: &Box<dyn TensorBase<Type>>) -> MlResult<()> {
        if self.shape != other.as_ref().shape() {
            return Err(MlError::TensorError(TensorError::InvalidShape {
                expected: self.shape.to_vec(),
                got: other.shape().to_vec(),
            }));
        }
        Ok(())
    }

    fn requires_grad(&mut self, requires: bool) {
        self.requires_grad = requires;
    }

    // fn set_grad_fn<F>(&mut self, grad_fn: F)
    // where
    //     F: Fn(&Tensor<Type>) -> MlResult<()> + 'static {
    //     self.grad_fn = Some(GradFn(Arc::new(grad_fn)));
    // }

    fn grad(&self) -> Option<&Tensor<Type>> {
        self.grad.as_ref().map(|g| g.as_ref())
    }
}

// impl<T: TensorBase Function<T> for Functions {
//     type Output = MlResult<T>;
//     type Gradient = f64;
//
//     fn forward(&self) -> Self::Output {
//         match self {
//             Functions::Abs      (F) => F.forward(),
//             Functions::Exp      (F) => F.forward(),
//             Functions::Log      (F) => F.forward(),
//             Functions::Neg      (F) => F.forward(),
//             Functions::Sqrt     (F) => F.forward(),
//             Functions::Square   (F) => F.forward(),
//
//             Functions::Add      (F) => F.forward(),
//             Functions::Sub      (F) => F.forward(),
//             Functions::Mul      (F) => F.forward(),
//             Functions::Div      (F) => F.forward(),
//             Functions::Pow      (F) => F.forward(),
//             Functions::Matmul   (F) => F.forward(),
//
//             Functions::Topk     (F) => F.forward(),
//             Functions::Matmax   (F) => F.forward(),
//         }
//     }
//     fn backward(&self, grad: Self::Gradient) -> Self::Output {
//         match self {
//             Functions::Abs      (F) =>  F.backward(grad),
//             Functions::Exp      (F) =>  F.backward(grad),
//             Functions::Log      (F) =>  F.backward(grad),
//             Functions::Neg      (F) =>  F.backward(grad),
//             Functions::Sqrt     (F) =>  F.backward(grad),
//             Functions::Square   (F) =>  F.backward(grad),
//
//             Functions::Add      (F) =>  F.backward(grad),
//             Functions::Sub      (F) =>  F.backward(grad),
//             Functions::Mul      (F) =>  F.backward(grad),
//             Functions::Div      (F) =>  F.backward(grad),
//             Functions::Pow      (F) =>  F.backward(grad),
//             Functions::Matmul   (F) =>  F.backward(grad),
//
//             Functions::Topk     (F) =>  F.backward(grad),
//             Functions::Matmax   (F) =>  F.backward(grad),
//         }
//         todo!("역전파 구현하기")
//     }
// }