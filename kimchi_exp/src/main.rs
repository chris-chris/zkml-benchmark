use kimchi::{
    bench::BenchmarkCtx,
    circuits::{
        polynomials::generic::testing::{create_circuit, fill_in_witness},
        wires::COLUMNS,
    },
    proof::ProverProof,
    prover_index::testing::new_index_for_test,
    verifier::verify,
    verifier_index::VerifierIndex,
};
use ark_ec::short_weierstrass_jacobian::GroupAffine;
use ark_ff::Zero;
use kimchi::groupmap::GroupMap;
use kimchi::mina_curves::pasta::{Fp, Vesta, VestaParameters};
use kimchi::mina_poseidon::{
    constants::PlonkSpongeConstantsKimchi,
    sponge::{DefaultFqSponge, DefaultFrSponge},
};
use kimchi::poly_commitment::{commitment::CommitmentCurve, evaluation_proof::OpeningProof, srs::SRS};
use std::array;
use std::time::Instant;

mod mlp;

use std::env;

use mlp::create_mlp_circuit;
use mlp::fill_in_mlp_witness;

type SpongeParams = PlonkSpongeConstantsKimchi;
type BaseSponge = DefaultFqSponge<VestaParameters, SpongeParams>;
type ScalarSponge = DefaultFrSponge<Fp, SpongeParams>;

fn main() {
    // 명령줄 인수에서 exp 값을 받음, 기본값은 1
    let args: Vec<String> = env::args().collect();
    let exp: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(1)  // 명령줄 인수가 있으면 그 값을 사용, 없으면 기본값 1
    } else {
        1
    };
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

    // // deserialize the verifier index
    // let mut verifier_index_deserialize: VerifierIndex<GroupAffine<VestaParameters>, _> =
    //     serde_json::from_str(&verifier_index_serialize).unwrap();

    // // add srs with lagrange bases
    // let mut srs = SRS::<GroupAffine<VestaParameters>>::create(verifier_index.max_poly_size);
    // srs.add_lagrange_basis(verifier_index.domain);
    // verifier_index_deserialize.powers_of_alpha = index.powers_of_alpha;
    // verifier_index_deserialize.linearization = index.linearization;
    // verifier_index_deserialize.srs = std::sync::Arc::new(srs);

    // // verify the proof
    // let start = Instant::now();
    // verify::<Vesta, BaseSponge, ScalarSponge, OpeningProof<Vesta>>(
    //     &group_map,
    //     &verifier_index_deserialize,
    //     &proof,
    //     &public,
    // )
    // .unwrap();
    // println!("- time to verify: {}ms", start.elapsed().as_millis());

}
