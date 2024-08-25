use ndarray::array;
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
    // 입력 데이터 준비 (여기서는 간단히 2차원 데이터를 사용)
    let x = DenseMatrix::from_2d_array(&[&[1.0, 1.0], &[1.0, 2.0], &[2.0, 2.0], &[2.0, 3.0]]);

    // 레이블 (목표값)
    let y = array![6.0, 8.0, 9.0, 11.0];

    // 첫 번째 층: 선형 회귀 모델 사용
    let model1 = LinearRegression::fit(&x, &y.to_vec(), Default::default()).unwrap();
    let layer1_weights = model1.coefficients().clone(); // coefficients는 DenseMatrix입니다.
    let layer1_biases = vec![model1.intercept().to_owned()]; // intercept는 단일 f64 값입니다.

    // 두 번째 층: 수동으로 가중치 설정
    let layer2_weights = DenseMatrix::from_2d_array(&[&[0.3], &[0.7]]);
    let layer2_biases = vec![0.0];

    // 다층 퍼셉트론 (MLP) 구조 설정
    let layers: Vec<(DenseMatrix<f64>, Vec<f64>)> = vec![
        (layer1_weights, layer1_biases),
        (layer2_weights, layer2_biases),
    ];

    // MLP 예측 수행
    let prediction = mlp_forward(x.clone(), &layers);

    // 예측 결과 출력
    for (i, pred) in prediction.row_iter().enumerate() {
        println!("Prediction for input {}: {:?}", i, pred);
    }

    // 모델과 데이터를 파일로 저장
    let model_bytes = rmp_serde::to_vec(&layers).unwrap();
    let data_bytes = rmp_serde::to_vec(&x).unwrap();

    let model_json = serde_json::to_string(&model_bytes).unwrap();
    let x_json = serde_json::to_string(&data_bytes).unwrap();

    let mut f = File::create("model_data/mlp_model_bytes.json").expect("unable to create file");
    f.write_all(model_json.as_bytes())
        .expect("Unable to write data");

    let mut f1 =
        File::create("model_data/mlp_input_data_bytes.json").expect("unable to create file");
    f1.write_all(x_json.as_bytes())
        .expect("Unable to write data");
}
