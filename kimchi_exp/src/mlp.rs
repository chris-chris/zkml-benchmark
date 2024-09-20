use ark_ff::FftField;
use kimchi::circuits::wires::COLUMNS;
use kimchi::{
    circuits::{gate::CircuitGate, polynomials::generic::GenericGateSpec, wires::Wire},
    mina_curves::pasta::Fp,
};

/// Create a generic circuit
///
/// # Panics
///
/// Will panic if `gates_row` is None.
    /// 간단한 MLP 서킷 생성 (depth = 2, 입력 크기 고정)
pub fn create_simple_mlp_circuit(input_size: usize) -> Vec<CircuitGate<Fp>> {
    let mut gates = vec![];
    let mut gates_row = (0..).into_iter(); // 행 번호 생성기

    // 입력 레이어 처리
    for _ in 0..input_size {
        let r = gates_row.next().unwrap();
        gates.push(CircuitGate::create_generic_gadget(
            Wire::for_row(r),
            GenericGateSpec::Pub, // 공개 입력
            None,
        ));
    }

    // 은닉 레이어 1
    for _ in 0..input_size {
        let r = gates_row.next().unwrap();
        let weight = Fp::from(3u32); // 가중치 값 (임의 설정)
        let bias = Fp::from(1u32);   // 바이어스 값 (임의 설정)

        // 선형 변환: output = input * weight + bias
        let g1 = GenericGateSpec::Mul {
            mul_coeff: Some(weight),
            output_coeff: Some(bias),
        };

        gates.push(CircuitGate::create_generic_gadget(
            Wire::for_row(r),
            g1,
            None,
        ));

        // ReLU 활성화 함수
        let r_relu = gates_row.next().unwrap();
        gates.push(CircuitGate::create_generic_gadget(
            Wire::for_row(r_relu),
            GenericGateSpec::Mul {
                mul_coeff: Some(Fp::from(1u32)),  // 입력값 그대로 통과
                output_coeff: Some(Fp::from(0u32)),   // 음수일 경우 0으로 처리
            },
            None,
        ));
    }

    // 출력 레이어
    for _ in 0..input_size {
        let r = gates_row.next().unwrap();
        gates.push(CircuitGate::create_generic_gadget(
            Wire::for_row(r),
            GenericGateSpec::Pub, // 출력값 처리
            None,
        ));
    }

    gates
}

/// Witness 생성 (depth 2 고정)
pub fn fill_in_simple_mlp_witness<F: FftField>(
    start_row: usize,
    witness: &mut [Vec<F>; COLUMNS],
    public: &[F],         // 공개 입력 (MLP의 입력 데이터)
    input_size: usize,    // 입력 크기
) {
    let mut witness_row = (start_row..).into_iter(); // 행 번호 생성기

    // 입력 레이어: 공개 입력 처리
    for p in public.iter() {
        let r = witness_row.next().unwrap();
        witness[0][r] = *p;  // 입력 데이터를 witness에 채움
    }

    // 은닉 레이어 1: 선형 변환과 ReLU 적용
    for _ in 0..input_size {
        let r = witness_row.next().unwrap();

        let input_val = witness[0][r]; // 입력값
        let weight = F::from(3u32);    // 가중치 값
        let bias = F::from(1u32);      // 바이어스 값

        // 선형 변환: output = input * weight + bias
        let linear_output = input_val * weight + bias;

        // ReLU 적용: 음수일 경우 0으로 변환
        let relu_output = if linear_output > F::zero() {
            linear_output
        } else {
            F::zero()
        };

        witness[1][r] = relu_output;    // ReLU 결과를 witness에 저장
    }

    // 출력 레이어: 마지막 은닉층 결과를 witness에 저장
    for _ in 0..input_size {
        let r = witness_row.next().unwrap();
        let final_output = witness[2][r]; // 은닉 레이어 2의 출력값
        witness[0][r] = final_output;     // 최종 출력값을 witness에 저장
    }
}

pub fn create_mlp_circuit(input_size: usize, depth: usize) -> Vec<CircuitGate<Fp>> {
    let mut gates = vec![];
    let mut gates_row = (0..).into_iter(); // 행 번호 생성기

    // 입력 레이어 처리
    for _ in 0..input_size {
        let r = gates_row.next().unwrap();
        gates.push(CircuitGate::create_generic_gadget(
            Wire::for_row(r),
            GenericGateSpec::Pub, // 공개 입력
            None,
        ));
    }

    // 은닉 레이어: depth만큼 반복
    for _ in 0..depth {
        for _ in 0..input_size {
            let r = gates_row.next().unwrap();
            let weight = Fp::from(0u32); // 가중치 값 (임의 설정)
            let bias = Fp::from(0u32);   // 바이어스 값 (임의 설정)

            // 선형 변환: output = input * weight + bias
            let g1 = GenericGateSpec::Mul {
                mul_coeff: Some(weight),
                output_coeff: None,
            };
            let g2 = GenericGateSpec::Add {
                left_coeff: None,
                right_coeff: Some(bias),
                output_coeff: None,
            };

            gates.push(CircuitGate::create_generic_gadget(
                Wire::for_row(r),
                g1,
                Some(g2),
            ));

            // ReLU 활성화 함수
            // let r_relu = gates_row.next().unwrap();
            // gates.push(CircuitGate::create_generic_gadget(
            //     Wire::for_row(r_relu),
            //     GenericGateSpec::Mul {
            //         mul_coeff: Some(Fp::from(1u32)),  // 입력값 그대로 통과
            //         output_coeff: Some(Fp::from(0u32)),   // 음수일 경우 0으로 처리
            //     },
            //     None,
            // ));
        }
    }

    // 출력 레이어
    let r = gates_row.next().unwrap();
    gates.push(CircuitGate::create_generic_gadget(
        Wire::for_row(r),
        GenericGateSpec::Pub, // 출력값 처리
        None,
    ));

    gates
}

/// Witness 생성 (depth를 받음)
pub fn fill_in_mlp_witness<F: FftField>(
    start_row: usize,
    witness: &mut [Vec<F>; COLUMNS],
    public: &[F],         // 공개 입력 (MLP의 입력 데이터)
    input_size: usize,    // 입력 크기
    depth: usize,         // 은닉층의 깊이
) {
    let mut witness_row = (start_row..).into_iter(); // 행 번호 생성기

    // 입력 레이어: 공개 입력 처리
    for p in public.iter() {
        let r = witness_row.next().unwrap();
        witness[0][r] = *p;  // 입력 데이터를 witness에 채움
    }

    // 은닉 레이어: depth만큼 반복
    for _ in 0..depth {
        for _ in 0..input_size {
            let r = witness_row.next().unwrap();

            let input_val = witness[0][r]; // 이전 레이어 출력값
            let weight = F::from(0u32);    // 가중치 값
            let bias = F::from(0u32);      // 바이어스 값

            // 선형 변환: output = input * weight + bias
            let linear_output = input_val * weight + bias;

            // ReLU 적용: 음수일 경우 0으로 변환
            // let relu_output = if linear_output > F::zero() {
            //     linear_output
            // } else {
            //     F::zero()
            // };

            witness[1][r] = linear_output;    // ReLU 결과를 witness에 저장
        }
    }

    // 출력 레이어: 마지막 은닉층 결과 중 하나를 witness에 저장
    // let final_output = witness[1][witness_row.next().unwrap()]; // 은닉 레이어 결과값 중 하나 선택
    // println!("final_output: {:?}", final_output);
    // witness[0][witness[0]] = final_output;     // 최종 출력값을 witness에 저장
    
}