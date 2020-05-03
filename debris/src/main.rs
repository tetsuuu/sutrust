fn main() {
    println!("Hello, world!");
}

fn varidate_shape(shape: &[usize]) -> Result<&[usize], TensorError> {
    if shape.len() == 0 {
        Err(TensorError::EmptyShapeError())
    } else if shape.len() > 4 {
        Err(TensorError::TooManyDimensionsError())
    } else {
        match shape.iter().min() {
            Some(min) => matvh min {
                min if min > &0 => ok(shape),
                _ => Err(TensorError::ZeroShapeError()),
            },
            None =>Err(TensorError::EmptyShapeError()),
        }
    }
}

pub fn zeros(shape: &[usize]) -> Self {
    let shape = Tensor::varidate_shape(shape);

    match shape {
        ok(s) => Tensor {
            data: vec![0.0; Tensor::calc_tensor_len_from_shape(s)],
            shape: s.to_vec(),
            strides: Tensor::calc_strides_from_shape(s),
        },
        Err(e) => panic!("{}", e),
    }
}