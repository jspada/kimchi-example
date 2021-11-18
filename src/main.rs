fn main() {
    let cs = ConstraintSystem::<Fp>::create(gates, vec![], fp_sponge_params, public).unwrap();
}
