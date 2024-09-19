use ark_ff::{BigInteger, BigInteger256, PrimeField};
use kimchi::mina_curves::pasta::Fp;
use kimchi::proof::ProverProof;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Deserialize, Serialize};
use bincode;
use std::error::Error;
use env_logger;
use chrono::Local;
use log::*;
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct NumberRequest {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct NumberResponse {
    value: u128,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let target = Box::new(File::create("log/server.log").expect("Can't create file"));

    env_logger::Builder::new()
        .target(env_logger::Target::Pipe(target))
        .filter(None, LevelFilter::Debug)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
    
    let listener = TcpListener::bind("127.0.0.1:4000").await?;
    info!("Server listening on port 4000");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        info!("New connection from {}", addr);

        tokio::spawn(async move {
            // 데이터 길이 읽기 (4바이트)
            let mut length_buffer = [0u8; 4];
            if let Err(e) = socket.read_exact(&mut length_buffer).await {
                eprintln!("Failed to read data length from socket; err = {:?}", e);
                return;
            }
            let data_length = u32::from_be_bytes(length_buffer) as usize;
            info!("data_length: {:?}", data_length);
        
            // 데이터 읽기
            let mut buffer = vec![0u8; data_length];
            if let Err(e) = socket.read_exact(&mut buffer).await {
                eprintln!("Failed to read data from socket; err = {:?}", e);
                return;
            }
        
            // 메시지 역직렬화
            let proof: kimchi::poly_commitment::evaluation_proof::OpeningProof<
                ark_ec::short_weierstrass_jacobian::GroupAffine<kimchi::mina_curves::pasta::VestaParameters>
            > = bincode::deserialize(&buffer[..]).unwrap();
            info!("Received message: {:?}", proof);
            
            // // public_output 데이터 길이 읽기
            // let mut length_buffer = [0u8; 4];
            // if let Err(e) = socket.read_exact(&mut length_buffer).await {
            //     eprintln!("Failed to read public_output length; err = {:?}", e);
            //     return;
            // }

            // let public_output_length = u32::from_be_bytes(length_buffer) as usize;
            // info!("public_output data_length: {:?}", public_output_length);

            // // public_output 데이터 읽기
            // let mut public_output_buffer = vec![0u8; public_output_length];
            // socket.read_exact(&mut public_output_buffer).await;

            // // 바이트 데이터를 Fp로 변환
            // let public_output: Vec<Fp> = public_output_buffer.chunks(32)  // 32바이트 청크로 분할
            //     .map(|chunk| {
            //         // BigInteger256로 변환
            //         let mut repr = BigInteger256::default();
            //         for (i, chunk_8bytes) in chunk.chunks(8).enumerate() {
            //             repr.0[i] = u64::from_be_bytes(chunk_8bytes.try_into().unwrap());
            //         }
            //         // Fp로 변환
            //         Fp::new(repr)
            //     })
            //     .collect();

            // info!("public_output: {:?}", public_output);

            // 데이터 길이 읽기 (4바이트)
            let mut length_buffer = [0u8; 4];
            if let Err(e) = socket.read_exact(&mut length_buffer).await {
                eprintln!("Failed to read data length from socket; err = {:?}", e);
                return;
            }
            let data_length = u32::from_be_bytes(length_buffer) as usize;
            info!("data_length: {:?}", data_length);

            // 데이터 읽기
            let mut buffer = vec![0u8; data_length];
            if let Err(e) = socket.read_exact(&mut buffer).await {
                eprintln!("Failed to read data from socket; err = {:?}", e);
                return;
            }

            // 메시지 역직렬화
            let number_response: NumberResponse = bincode::deserialize(&buffer[..]).unwrap();
            info!("Received public_output as u128: {:?}", number_response.value);

            // 응답 생성
            let response_data = bincode::serialize(&number_response).unwrap();

            // 응답 전송
            if let Err(e) = socket.write_all(&response_data).await {
                eprintln!("Failed to write to socket; err = {:?}", e);
                return;
            }

            // // 응답 생성
            // let number_response = NumberResponse { value: 42 };
            // let response_data = bincode::serialize(&number_response).unwrap();
        
            // // 응답 전송
            // if let Err(e) = socket.write_all(&response_data).await {
            //     eprintln!("Failed to write to socket; err = {:?}", e);
            //     return;
            // }
        });
    }
}