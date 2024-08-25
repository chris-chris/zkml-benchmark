use ndarray::array;
use risc0_zkvm::guest::env;
use smartcore::linalg::basic::arrays::{Array, Array2, MutArray};
use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::linear::linear_regression::LinearRegression;
use std::fs::File;
use std::io::Write;

// 활성화 함수 (ReLU)
fn relu(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        0.0
    }
}

// 퍼셉트론 함수: 선형 변환 + 활성화 함수
fn perceptron(
    input: &DenseMatrix<f64>,
    weights: &DenseMatrix<f64>,
    biases: &Vec<f64>,
) -> DenseMatrix<f64> {
    let mut output = DenseMatrix::from_2d_vec(&vec![vec![0.0; biases.len()]; input.shape().0]);

    for (i, row) in input.row_iter().enumerate() {
        let row_vec: Vec<f64> = row.iterator(0).map(|&x| x).collect(); // 슬라이스 대신 Vec<f64>로 변환
        for j in 0..biases.len() {
            // 선형 변환: 가중치 * 입력 + 편향
            let mut sum = biases[j];
            for (k, &input_val) in row_vec.iter().enumerate() {
                sum += input_val * weights.get((k, j));
            }
            // 활성화 함수 (ReLU 적용)
            output.set((i, j), relu(sum));
        }
    }

    output
}

// MLP 구조에서 forward propagation을 수행
fn mlp_forward(
    input: DenseMatrix<f64>,
    layers: &[(DenseMatrix<f64>, Vec<f64>)],
) -> DenseMatrix<f64> {
    let mut output = input;

    for (weights, biases) in layers.iter() {
        output = perceptron(&output, weights, biases);
    }

    output
}

fn main() {
    // Host로부터 직렬화된 입력 데이터를 읽습니다.
    let model_data_bytes: Vec<u8> = env::read();
    let input_data_bytes: Vec<u8> = env::read();

    // 역직렬화하여 MLPInput 구조체로 변환합니다.
    let layers: Vec<(DenseMatrix<f64>, Vec<f64>)> =
        rmp_serde::from_slice(&model_data_bytes).unwrap();
    let x: DenseMatrix<f64> = rmp_serde::from_slice(&input_data_bytes).unwrap();

    // MLP 예측 수행
    let prediction = mlp_forward(x.clone(), &layers);

    // 예측 결과를 journal에 커밋하여 공개합니다.
    env::commit(&prediction);
}
