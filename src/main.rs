use kimchi_circuits::{
    gate::{CircuitGate, GateType},
    nolookup::constraints::ConstraintSystem,
    wires::Wire,
};

use mina_curves::pasta::fp::Fp;

fn main() {
    let gates = vec![
        CircuitGate {
            row: 0,
            typ: GateType::Generic, // ql*L + qr*R + qo*O + qm*L*R + qc*C = 0
            wires: Wire::new(0),
            c: vec![
                // selector coefficients for V - E + F = 2
                Fp::from(1),  // ql
                Fp::from(-1), // qr
                Fp::from(-2), // qo
                Fp::from(0),  // qm
                Fp::from(1),  // qc
            ],
        },
        CircuitGate {
            row: 1,
            typ: GateType::Generic, // ql*L + qr*R + qo*O + qm*L*R + qc*C = 0
            wires: Wire::new(1),
            c: vec![
                // selector coefficients for Y = 3X + B
                Fp::from(3),  // ql
                Fp::from(1),  // qr
                Fp::from(-1), // qo
                Fp::from(0),  // qm
                Fp::from(0),  // qc
            ],
        },
    ];

    let cs = ConstraintSystem::create(gates, vec![], oracle::pasta::fp::params(), 0).unwrap();

    println!("cs = {:?}", cs);
}
