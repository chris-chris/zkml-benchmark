use ark_ff::{BigInteger, PrimeField};
use kimchi::mina_curves::pasta::Fp;

fn fp_to_integer(fp: Fp) -> u128 {
    // Fp의 내부 표현을 BigInteger256으로 가져옵니다.
    let big_integer = fp.into_repr();  // Fp에서 BigInteger256로 변환
    
    // BigInteger256은 4개의 u64 배열로 구성되어 있으므로, 이를 바이트로 변환
    let mut result = 0u128;
    for &part in big_integer.0.iter().rev() {
        result = (result << 64) | part as u128;
    }

    result // 결과를 문자열로 변환하여 반환
}

fn main() {
    // 임의의 Fp 값을 생성
    let fp = Fp::from(123456789u64);
    
    // Fp 값을 원래 정수로 변환
    let integer = fp_to_integer(fp);
    
    println!("Fp as integer: {}", integer);
}