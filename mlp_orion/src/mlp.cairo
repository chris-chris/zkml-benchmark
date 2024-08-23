use orion::operators::tensor::{Tensor, TensorTrait, FP16x16Tensor};
use orion::numbers::{FP16x16, FixedTrait};
use core::array::ArrayTrait;

// ReLU activation function
pub fn relu(x: FP16x16) -> FP16x16 {
    if x > FixedTrait::new(0, false) {
        return x;
    } else {
        return FixedTrait::new(0, false);
    }
}

// Perceptron function
pub fn perceptron(inputs: Tensor<FP16x16>, weights: Tensor<FP16x16>, bias: Tensor<FP16x16>) -> Tensor<FP16x16> {
    let mut output_shape = ArrayTrait::<u32>::new();
    output_shape.append(*weights.shape.at(1));

    let mut output_data = ArrayTrait::<FP16x16>::new();  // output_data 타입 수정

    let mut i: u32 = 0;
    loop {
        if i >= *output_shape.at(0) {
            break ();
        }

        // weighted_sum의 초기화를 FP16x16으로 변경
        let mut weighted_sum = FixedTrait::new(0, false);  // FP16x16 타입으로 초기화

        let mut j: u32 = 0;
        loop {
            if j >= *inputs.shape.at(0) {
                break ();
            }
            weighted_sum = *inputs.data.at(j) * *weights.data.at(j * *output_shape.at(0) + i) + weighted_sum;
            j += 1;
        };
        weighted_sum = weighted_sum + *bias.data.at(i);
        output_data.append(relu(weighted_sum));
        i += 1;
    };

    let output_tensor = TensorTrait::<FP16x16>::new(output_shape.span(), output_data.span());
    return output_tensor;
}

// MLP forward function
pub fn mlp_forward(num_layers: u32, nodes_per_layer: ArrayTrait<u32>) -> Tensor<FP16x16> {
    let x_values = X_values();
    let mut inputs = x_values;

    let mut layer_idx: u32 = 0;
    loop {
        if layer_idx >= num_layers {
            break ();
        }

        let mut weights_shape = ArrayTrait::new();
        weights_shape.append(*inputs.shape.at(0));
        weights_shape.append(nodes_per_layer.at(layer_idx));

        let mut weights_data = ArrayTrait::<FP16x16>::new();  // weights_data 타입 수정
        let mut bias_shape = ArrayTrait::new();
        let mut bias_data = ArrayTrait::<FP16x16>::new();  // bias_data 타입 수정

        let mut i: u32 = 0;
        loop {
            if i >= inputs.shape.at(0) * nodes_per_layer.at(layer_idx) {
                break ();
            }
            weights_data.append(FixedTrait::new(1, false));  // 예시 값: 모두 1로 설정
            i += 1;
        }

        let mut j: u32 = 0;
        loop {
            if j >= nodes_per_layer.at(layer_idx) {
                break ();
            }
            bias_data.append(FixedTrait::new(0, false));  // 예시 값: 모두 0으로 설정
            j += 1;
        }

        let weights = TensorTrait::<FP16x16>::new(weights_shape.span(), weights_data.span());
        let bias = TensorTrait::<FP16x16>::new(bias_shape.span(), bias_data.span());

        inputs = perceptron(inputs, weights, bias);
        layer_idx += 1;
    }

    return inputs;
}

