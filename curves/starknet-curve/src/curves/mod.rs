use crate::{fq::Fq, fr::Fr};
use ark_ec::hashing::curve_maps::swu::SWUConfig;
use ark_ec::{
    models::CurveConfig,
    short_weierstrass::{self as sw, SWCurveConfig},
};
use ark_ff::{Field, MontFp, Zero};

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

impl SWUConfig for Config {
    // def find_z_sswu(F, A, B):
    //     R.<xx> = F[]                       # Polynomial ring over F
    //     g = xx^3 + F(A) * xx + F(B)        # y^2 = g(x) = x^3 + A * x + B
    //     ctr = F.gen()
    //     while True:
    //         for Z_cand in (F(ctr), F(-ctr)):
    //             # Criterion 1: Z is non-square in F.
    //             if is_square(Z_cand):
    //                 continue
    //             # Criterion 2: Z != -1 in F.
    //             if Z_cand == F(-1):
    //                 continue
    //             # Criterion 3: g(x) - Z is irreducible over F.
    //             if not (g - Z_cand).is_irreducible():
    //                 continue
    //             # Criterion 4: g(B / (Z * A)) is square in F.
    //             if is_square(g(B / (Z_cand * A))):
    //                 return Z_cand
    //         ctr += 1
    // F = GF(3618502788666131213697322783095070105623107215331596699973092056135872020481)
    // a = 1
    // b = 3141592653589793238462643383279502884197169399375105820974944592307816406665
    // find_z_sswu(F, a, b) == 19
    const ZETA: Fq = MontFp!("19");
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
    fn mul_by_a(x: Self::BaseField) -> Self::BaseField {
        x
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
