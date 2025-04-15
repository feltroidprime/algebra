use ark_ec::{
    models::CurveConfig,
    short_weierstrass::{self as sw, SWCurveConfig},
};
use ark_ff::{AdditiveGroup, Field, MontFp, Zero};

use crate::{fq::Fq, fr::Fr};

#[cfg(test)]
mod tests;

pub type Affine = sw::Affine<Config>;
pub type Projective = sw::Projective<Config>;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Config;

impl CurveConfig for Config {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR = 1
    const COFACTOR: &'static [u64] = &[0x1];

    /// COFACTOR_INV = COFACTOR^{-1} mod r = 1
    #[rustfmt::skip]
    const COFACTOR_INV: Fr =  Fr::ONE;
}

impl SWCurveConfig for Config {
    /// COEFF_A = 1
    const COEFF_A: Fq = Fq::ONE;

    /// COEFF_B = 3141592653589793238462643383279502884197169399375105820974944592307816406665
    const COEFF_B: Fq =
        MontFp!("3141592653589793238462643383279502884197169399375105820974944592307816406665");

    /// GENERATOR = (G_GENERATOR_X, G_GENERATOR_Y)
    const GENERATOR: Affine = Affine::new_unchecked(G_GENERATOR_X, G_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }
}

/// G_GENERATOR_X =
/// 874739451078007766457464989774322083649278607533249481151382481072868806602
pub const G_GENERATOR_X: Fq =
    MontFp!("874739451078007766457464989774322083649278607533249481151382481072868806602");

/// G_GENERATOR_Y =
/// 152666792071518830868575557812948353041420400780739481342941381225525861407
pub const G_GENERATOR_Y: Fq =
    MontFp!("152666792071518830868575557812948353041420400780739481342941381225525861407");
