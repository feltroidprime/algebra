use ark_algebra_bench_templates::*;
use ark_starknet_curve::{fq::Fq, fr::Fr, Projective as G};

bench!(
    Name = "StarknetCurve",
    Group = G,
    ScalarField = Fr,
    PrimeBaseField = Fq,
);
