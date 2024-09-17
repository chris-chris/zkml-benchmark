use kimchi::proof::ProverProof;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Deserialize, Serialize};
use bincode;
use std::error::Error;

#[derive(Serialize, Deserialize)]
struct NumberRequest {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct NumberResponse {
    value: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:4000").await?;
    println!("Server listening on port 4000");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

        tokio::spawn(async move {
            // 데이터 길이 읽기 (4바이트)
            let mut length_buffer = [0u8; 4];
            if let Err(e) = socket.read_exact(&mut length_buffer).await {
                eprintln!("Failed to read data length from socket; err = {:?}", e);
                return;
            }
            let data_length = u32::from_be_bytes(length_buffer) as usize;
        
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
            println!("Received message: {:?}", proof);
        
            // 응답 생성
            let number_response = NumberResponse { value: 42 };
            let response_data = bincode::serialize(&number_response).unwrap();
        
            // 응답 전송
            if let Err(e) = socket.write_all(&response_data).await {
                eprintln!("Failed to write to socket; err = {:?}", e);
                return;
            }
        });
    }
}