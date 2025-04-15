#[cfg(test)]
mod tests {
    use crate::Fq;
    use core::fmt;
    use std::println;
    // use ark_ec::hashing::HashToCurve;
    use ark_ff::field_hashers::{DefaultFieldHasher, HashToField};
    use num_bigint::BigInt;
    use sha2::Sha256;

    #[test]
    fn test_hash_to_field() {
        let hasher = <DefaultFieldHasher<Sha256> as HashToField<Fq>>::new(&[1, 2, 3]);
        let field_elements: Vec<Fq> = hasher.hash_to_field(b"Hello, World!", 2);

        assert_eq!(field_elements.len(), 2);

        for elem in field_elements {
            println!("{:}", elem);
        }

        let x = Fq::from(14);
        println!("{:}", x);
    }
}
