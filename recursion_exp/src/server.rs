use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;
use serde::{Deserialize, Serialize};
use bincode;
use tokio::sync::Notify;
use std::env;
use std::error::Error;
use std::sync::{Arc, Mutex};
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
    public_output: u128,
    msm_call_count: u64,
    msm_accumulated_time: f32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let client_count: u32 = if args.len() > 1 {
        args[1].to_string().parse::<u32>().unwrap_or(1)
    } else {
        "1".to_string().parse::<u32>().unwrap_or(1)
    };

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
    
    let listener = TcpListener::bind("0.0.0.0:4000").await?;
    info!("Server listening on port 4000");

    let connection_start_time = Arc::new(Mutex::new(None::<chrono::NaiveDateTime>));
    let public_output_count = Arc::new(Mutex::new(0));
    let notify = Arc::new(Notify::new());
    
    loop {
        let (mut socket, addr) = listener.accept().await?;
        info!("New connection from {}", addr);
        
        // Record the time of the first connection
        let connection_start_time = Arc::clone(&connection_start_time);
        if connection_start_time.lock().unwrap().is_none() {
            *connection_start_time.lock().unwrap() = Some(Local::now().naive_utc());
        }

        let public_output_count = Arc::clone(&public_output_count);
        let notify = Arc::clone(&notify);

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
            
            // public_output 데이터 길이 읽기
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
            info!("Received public_output: {:?}", number_response.public_output);
            info!("Received msm_accumulated_time: {:?}", number_response.msm_accumulated_time);
            info!("Received msm_call_count: {:?}", number_response.msm_call_count);

            // Increase the count and check if it is the 32nd client
            let mut count = public_output_count.lock().unwrap();
            *count += 1;

            if *count == client_count {
                let end_time = Local::now().naive_utc();
                let start_time = connection_start_time.lock().unwrap().clone().unwrap();
                let duration = end_time - start_time;
                info!("Time taken for {} public_output: {:?}", client_count, duration);
                std::process::exit(0);
            }

            // Notify that public output has been received
            notify.notify_one();

            // // 응답 생성
            // let response_data = bincode::serialize(&number_response).unwrap();

            // // 응답 전송
            // if let Err(e) = socket.write_all(&response_data).await {
            //     eprintln!("Failed to write to socket; err = {:?}", e);
            //     return;
            // }

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
