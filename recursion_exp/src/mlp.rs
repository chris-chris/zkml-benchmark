use std::array;
use std::time::Instant;
use kimchi::{
    circuits::{
        polynomials::generic::testing::{create_circuit, fill_in_witness},
        wires::COLUMNS,
    },
    proof::ProverProof,
    prover_index::testing::new_index_for_test,
};
use ark_ff::Zero;
use kimchi::groupmap::GroupMap;
use kimchi::mina_curves::pasta::{Fp, Vesta, VestaParameters};
use kimchi::mina_poseidon::{
    constants::PlonkSpongeConstantsKimchi,
    sponge::{DefaultFqSponge, DefaultFrSponge},
};
use kimchi::poly_commitment::{commitment::CommitmentCurve, evaluation_proof::OpeningProof, srs::SRS};

type SpongeParams = PlonkSpongeConstantsKimchi;
type BaseSponge = DefaultFqSponge<VestaParameters, SpongeParams>;
type ScalarSponge = DefaultFrSponge<Fp, SpongeParams>;


use ark_ff::FftField;
use kimchi::{
    circuits::{gate::CircuitGate, polynomials::generic::GenericGateSpec, wires::Wire},
};


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
    }

    // 출력 레이어: 마지막 은닉층 결과를 witness에 저장
    for _ in 0..input_size {
        let r = witness_row.next().unwrap();
        let final_output = witness[1][r]; // 은닉 레이어 결과값
        witness[0][r] = final_output;     // 최종 출력값을 witness에 저장
    }
}

pub fn mlp_by_depth(exp: usize) 
-> ProverProof<ark_ec::short_weierstrass_jacobian::GroupAffine<kimchi::mina_curves::pasta::VestaParameters>, 
kimchi::poly_commitment::evaluation_proof::OpeningProof<ark_ec::short_weierstrass_jacobian::GroupAffine<kimchi::mina_curves::pasta::VestaParameters>>> {

    println!("run exp {}", exp);

    let depth = 2usize.pow(exp.try_into().unwrap());
    println!("Creating proof for depth: {}", depth);

    // Input size: 4
    let public = vec![Fp::from(3u8); 4];
    
    println!("public.len() {}", public.len());
    let gates = create_mlp_circuit(public.len(), depth);

    // create witness
    let mut witness: [Vec<Fp>; COLUMNS] = array::from_fn(|_| vec![Fp::zero(); gates.len()]);
    fill_in_mlp_witness(0, &mut witness, &public, public.len(), depth);

    let index = new_index_for_test(gates, public.len());
    let verifier_index = index.verifier_index();

    let verifier_index_serialize =
        serde_json::to_string(&verifier_index).expect("couldn't serialize index");

    // verify the circuit satisfiability by the computed witness
    index.verify(&witness, &public).unwrap();

    // add the proof to the batch
    let group_map = <Vesta as CommitmentCurve>::Map::setup();

    let start_proof = Instant::now();
    let proof =
        ProverProof::create::<BaseSponge, ScalarSponge>(&group_map, witness, &[], &index)
            .unwrap();
    println!("- time to prove: {}ms", start_proof.elapsed().as_millis());
    proof
}
