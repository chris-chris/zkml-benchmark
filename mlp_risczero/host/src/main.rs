use methods::MLP_GUEST_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::fs::File;
use std::io::Read;

fn main() {
    // JSON 파일에서 모델과 입력 데이터를 읽습니다.
    let mut model_file =
        File::open("./model_data/mlp_model_bytes.json").expect("unable to open model file");
    let mut model_json = String::new();
    model_file
        .read_to_string(&mut model_json)
        .expect("unable to read model data");

    let mut input_file = File::open("./model_data/mlp_input_data_bytes.json")
        .expect("unable to open input data file");
    let mut input_json = String::new();
    input_file
        .read_to_string(&mut input_json)
        .expect("unable to read input data");

    // JSON 데이터를 바이트 배열로 변환
    let model_bytes: Vec<u8> = serde_json::from_str(&model_json).unwrap();
    let input_bytes: Vec<u8> = serde_json::from_str(&input_json).unwrap();

    // Executor 환경을 설정합니다.
    let env = ExecutorEnv::builder()
        .write(&model_bytes)
        .unwrap()
        .write(&input_bytes)
        .unwrap()
        .build()
        .unwrap();

    // 기본 prover를 가져옵니다.
    let prover = default_prover();

    // 지정된 ELF 바이너리를 증명하여 영수증(receipt)을 생성합니다.
    let receipt = prover.prove(env, MLP_GUEST_ELF).unwrap().receipt;

    // 영수증의 journal에서 출력 값을 추출합니다.
    let output: Vec<f64> = receipt.journal.decode().unwrap();

    // 출력 결과를 출력합니다. 이 결과는 journal에 커밋된 후 공개된 값입니다.
    println!(
        "Hello, world! I generated a proof of guest execution! The model prediction is: {:?}",
        output
    );
}
