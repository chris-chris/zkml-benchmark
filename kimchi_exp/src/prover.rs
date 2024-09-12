use kimchi::{
    circuits::{gate::CircuitGate},
    proof::ProverProof,
    prover_index::ProverIndex,
    groupmap::GroupMap,
    mina_curves::pasta::{Fp, Vesta},
    mina_poseidon::sponge::{DefaultFqSponge, DefaultFrSponge},
    poly_commitment::commitment::CommitmentCurve,
    proof::OpeningProof,
};
use crate::circuit::create_complex_circuit;

type BaseSponge = DefaultFqSponge<Vesta, kimchi::mina_poseidon::constants::PlonkSpongeConstantsKimchi>;
type ScalarSponge = DefaultFrSponge<Fp, kimchi::mina_poseidon::constants::PlonkSpongeConstantsKimchi>;

pub fn create_linear_proof(depth: usize) -> (
    ProverProof<Vesta, ScalarSponge>, 
    Vec<Fp>, 
    ProverIndex<Vesta, OpeningProof<Vesta>>
) {
    // 서킷을 생성합니다.
    let circuit = create_complex_circuit(depth);

    // ProverIndex 생성
    let prover_index = ProverIndex::<Vesta, OpeningProof<Vesta>>::create(circuit.clone(), None);

    // GroupMap 생성
    let group_map = <Vesta as CommitmentCurve>::Map::setup();

    // 증명 생성
    let prover_proof = ProverProof::create::<BaseSponge, ScalarSponge>(
        &group_map,
        &circuit,
        &prover_index,
        &[], // Public input
    )
    .unwrap();

    // VerifierIndex는 ProverIndex에서 가져옵니다.
    let verifier_index = prover_index.verifier_index();

    (prover_proof, vec![], verifier_index)
}