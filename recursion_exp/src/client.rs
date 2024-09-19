use ark_ff::{BigInteger, PrimeField};
use kimchi::mina_curves::pasta::Fp;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Deserialize, Serialize};
use bincode;
use std::env;
use std::error::Error;

mod mlp;

use mlp::mlp_by_depth;

#[derive(Serialize, Deserialize)]
struct NumberRequest {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct NumberResponse {
    value: u128,
}

pub fn fp_to_integer(fp: Fp) -> u128 {
    // Fp의 내부 표현을 BigInteger256으로 가져옵니다.
    let big_integer = fp.into_repr();  // Fp에서 BigInteger256로 변환
    
    // BigInteger256은 4개의 u64 배열로 구성되어 있으므로, 이를 바이트로 변환
    let mut result = 0u128;
    for &part in big_integer.0.iter().rev() {
        result = (result << 64) | part as u128;
    }

    result // 결과를 문자열로 변환하여 반환
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 명령줄 인수에서 서버 주소를 가져옵니다.
    let args: Vec<String> = env::args().collect();
    let server_addr = if args.len() > 1 {
        args[1].clone()
    } else {
        "127.0.0.1:4000".to_string()
    };

    let mut socket = TcpStream::connect(server_addr).await?;
    println!("Connected to server");

    let args: Vec<String> = env::args().collect();
    let exp: usize = if args.len() > 2 {
        args[2].parse().unwrap_or(1)  // 명령줄 인수가 있으면 그 값을 사용, 없으면 기본값 1
    } else {
        1
    };

    let result: (kimchi::proof::ProverProof<ark_ec::short_weierstrass_jacobian::GroupAffine<kimchi::mina_curves::pasta::VestaParameters>, kimchi::poly_commitment::evaluation_proof::OpeningProof<ark_ec::short_weierstrass_jacobian::GroupAffine<kimchi::mina_curves::pasta::VestaParameters>>>, Vec<Fp>) = mlp_by_depth(exp);

    let _proof: kimchi::poly_commitment::evaluation_proof::OpeningProof<ark_ec::short_weierstrass_jacobian::GroupAffine<kimchi::mina_curves::pasta::VestaParameters>> = result.0.proof;
    let public_output: Vec<Fp> = result.1;

    println!("_proof: {:?}", _proof);
    println!("public_output: {:?}", public_output);
    let public_output_u = fp_to_integer(public_output[0]);
    println!("public_output_u: {:?}", public_output_u);

    // 요청 생성
    // let number_request = NumberRequest {
    //     message: "Please send your number".to_string(),
    // };
    let request_data = bincode::serialize(&_proof).unwrap();
    let data_length = request_data.len() as u32;

    // 데이터 길이 전송
    socket.write_all(&data_length.to_be_bytes()).await?;

    // 데이터 전송
    socket.write_all(&request_data).await?;

    let number_response = NumberResponse { value: public_output_u };
    let response_data = bincode::serialize(&number_response).unwrap();
    let data_length = response_data.len() as u32;

    socket.write_all(&data_length.to_be_bytes()).await?;
    socket.write_all(&response_data).await?;

    // let request_data = bincode::serialize(&_proof).unwrap();

    // // 요청 전송
    // socket.write_all(&request_data).await?;

    // 응답 읽기
    // let mut buffer = [0u8; 1024];
    // let n = socket.read(&mut buffer).await?;

    // // 응답 역직렬화
    // let number_response: NumberResponse = bincode::deserialize(&buffer[..n]).unwrap();
    // println!("Received number: {}", number_response.value);

    Ok(())
}