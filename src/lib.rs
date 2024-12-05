pub mod group;
pub mod scalar;

#[cfg(test)]
mod tests;

use constants::*;
pub use group::ArkGroupWrapper;
pub use scalar::ArkScalarWrapper;

pub mod constants {
    include!(concat!(env!("OUT_DIR"), "/field_constants.rs"));
    include!(concat!(env!("OUT_DIR"), "/curve_constants.rs"));
}

// Create modules for each implementation
macro_rules! impl_field {
    ($field:ty, $field_constants:ident) => {
        paste::paste! {
            mod [<__private_impl_field_ $field_constants>] {
                use super::*;
                impl_wrapped_field!($field, $field_constants);
                impl_prime_field!($field, $field_constants);
            }
        }
    };
}

macro_rules! impl_group {
    ($curve:ty, $curve_constants:ident) => {
        paste::paste! {
            mod [<__private_impl_group_ $curve_constants>] {
                use super::*;
                impl_group_for_curve!($curve, $curve_constants);
            }
        }
    };
}

// Implement field traits once per scalar field type
impl_field!(ark_bls12_381::Fr, bls12_381_fr_constants);
// impl_field!(ark_secp256k1::Fr, secp256k1_constants);
// impl_field!(ark_secp256r1::Fr, secp256r1_constants);
// impl_field!(ark_secp384r1::Fr, secp384r1_constants);
// impl_field!(ark_secq256k1::Fr, secq256k1_constants);
// impl_field!(ark_curve25519::Fr, curve25519_constants);
// impl_field!(ark_bw6_767::Fr, bw6_767_fr_constants);
// impl_field!(ark_bls12_377::Fr, bls12_377_fr_constants);
// impl_field!(ark_bn254::Fr, bn254_fr_constants);
// impl_field!(ark_bw6_761::Fr, bw6_761_fr_constants);

// Implement group traits for each curve
impl_group!(ark_bls12_381::G1Projective, bls12_381_g1_constants);
// impl_group!(ark_secp256k1::Projective, secp256k1_projective_constants);
// impl_group!(ark_secp256r1::Projective, secp256r1_projective_constants);
// impl_group!(ark_secp384r1::Projective, secp384r1_projective_constants);
// impl_group!(ark_secq256k1::Projective, secq256k1_projective_constants);
// impl_group!(
//     ark_curve25519::EdwardsProjective,
//     curve25519_projective_constants
// );
// impl_group!(ark_ed25519::EdwardsProjective, ed25519_projective_constants);
// impl_group!(ark_bw6_767::G1Projective, bw6_767_g1_constants);
// impl_group!(ark_bw6_767::G2Projective, bw6_767_g2_constants);
// impl_group!(ark_cp6_782::G1Projective, cp6_782_g1_constants);
// impl_group!(ark_cp6_782::G2Projective, cp6_782_g2_constants);
// impl_group!(ark_bls12_377::G1Projective, bls12_377_g1_constants);
// impl_group!(ark_bls12_377::G2Projective, bls12_377_g2_constants);
// impl_group!(ark_bls12_381::G2Projective, bls12_381_g2_constants);
// impl_group!(ark_bn254::G1Projective, bn254_g1_constants);
// impl_group!(ark_bn254::G2Projective, bn254_g2_constants);
// impl_group!(ark_bw6_761::G1Projective, bw6_761_g1_constants);
// impl_group!(ark_bw6_761::G2Projective, bw6_761_g2_constants);
// impl_group!(
//     ark_ed_on_bls12_377::EdwardsProjective,
//     ed_on_bls12_377_constants
// );
// impl_group!(
//     ark_ed_on_bls12_381::EdwardsProjective,
//     ed_on_bls12_381_constants
// );
// impl_group!(
//     ark_ed_on_bw6_761::EdwardsProjective,
//     ed_on_bw6_761_constants
// );
// impl_group!(
//     ark_ed_on_cp6_782::EdwardsProjective,
//     ed_on_cp6_782_constants
// );
// impl_group!(
//     ark_ed_on_mnt4_298::EdwardsProjective,
//     ed_on_mnt4_298_constants
// );
// impl_group!(
//     ark_ed_on_mnt4_753::EdwardsProjective,
//     ed_on_mnt4_753_constants
// );
