use kimchi::{
    circuits::{gate::CircuitGate, polynomials::generic::GenericGateSpec, wires::Wire},
    mina_curves::pasta::Fp,
};

pub(crate) fn create_complex_circuit(depth: usize) -> Vec<CircuitGate<Fp>> {
    let mut gates = vec![];

    for i in 0..depth {
        let wires_mul = Wire::for_row(i * 2);
        let mul_gate = CircuitGate::create_generic_gadget(
            wires_mul.clone(),
            GenericGateSpec::Mul {
                output_coeff: None,
                mul_coeff: Some(Fp::from(3 + i as u64)),
            },
            None,
        );
        gates.push(mul_gate);

        let wires_add = Wire::for_row(i * 2 + 1);
        let add_gate = CircuitGate::create_generic_gadget(
            wires_add.clone(),
            GenericGateSpec::Add {
                left_coeff: None,
                right_coeff: Some(Fp::from(5 + i as u64)),
                output_coeff: None,
            },
            None,
        );
        gates.push(add_gate);
    }

    gates
}