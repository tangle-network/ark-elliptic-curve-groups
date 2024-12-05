use ark_ec::{AdditiveGroup, PrimeGroup};
use ark_ff::{UniformRand, Zero};
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

mod group;
mod scalar;

pub use group::ArkGroupWrapper;
pub use scalar::ArkScalarWrapper;

// impl_wrapped_field!(ark_secp256k1::Fr, secp256k1_constants);
// impl_prime_field!(ark_secp256k1::Fr, secp256k1_constants);
// impl_group_for_curve!(ark_secp256k1::Projective);

// impl_wrapped_field!(ark_secp256r1::Fr, secp256r1_constants);
// impl_prime_field!(ark_secp256r1::Fr, secp256r1_constants);
// impl_group_for_curve!(ark_secp256r1::Projective);

// impl_wrapped_field!(ark_secp384r1::Fr, secp384r1_constants);
// impl_prime_field!(ark_secp384r1::Fr, secp384r1_constants);
// impl_group_for_curve!(ark_secp384r1::Projective);

// impl_wrapped_field!(ark_secq256k1::Fr, secq256k1_constants);
// impl_prime_field!(ark_secq256k1::Fr, secq256k1_constants);
// impl_group_for_curve!(ark_secq256k1::Projective);

// impl_wrapped_field!(ark_curve25519::Fr, curve25519_constants);
// impl_prime_field!(ark_curve25519::Fr, curve25519_constants);
// impl_group_for_curve!(ark_curve25519::EdwardsProjective);
// impl_group_for_curve!(ark_ed25519::EdwardsProjective);

// impl_wrapped_field!(ark_bls12_377::Fr, bls12_377_constants);
// impl_prime_field!(ark_bls12_377::Fr, bls12_377_constants);
// impl_group_for_curve!(ark_bls12_377::G2Projective);

// impl_wrapped_field!(ark_ed_on_bls12_377::Fr, ed_on_bls12_377_constants);
// impl_prime_field!(ark_ed_on_bls12_377::Fr, ed_on_bls12_377_constants);
// impl_group_for_curve!(ark_ed_on_bls12_377::EdwardsProjective);

// impl_wrapped_field!(ark_bw6_761::Fr, bw6_761_constants);
// impl_prime_field!(ark_bw6_761::Fr, bw6_761_constants);
// impl_group_for_curve!(ark_bw6_761::G1Projective);

// impl_wrapped_field!(ark_ed_on_bw6_761::Fr, ed_on_bw6_761_constants);
// impl_prime_field!(ark_ed_on_bw6_761::Fr, ed_on_bw6_761_constants);
// impl_group_for_curve!(ark_ed_on_bw6_761::EdwardsProjective);

// impl_wrapped_field!(ark_bw6_767::Fr, bw6_767_constants);
// impl_prime_field!(ark_bw6_767::Fr, bw6_767_constants);
// impl_group_for_curve!(ark_bw6_767::G1Projective);

// impl_wrapped_field!(ark_cp6_782::Fr, cp6_782_constants);
// impl_prime_field!(ark_cp6_782::Fr, cp6_782_constants);
// impl_group_for_curve!(ark_cp6_782::G1Projective);

// impl_wrapped_field!(ark_ed_on_cp6_782::Fr, ed_on_cp6_782_constants);
// impl_prime_field!(ark_ed_on_cp6_782::Fr, ed_on_cp6_782_constants);
// impl_group_for_curve!(ark_ed_on_cp6_782::EdwardsProjective);

// impl_wrapped_field!(ark_bls12_381::Fr, bls12_381_constants);
// impl_prime_field!(ark_bls12_381::Fr, bls12_381_constants);
// impl_group_for_curve!(ark_bls12_381::G1Projective);

// impl_wrapped_field!(ark_ed_on_bls12_381::Fr, ed_on_bls12_381_constants);
// impl_prime_field!(ark_ed_on_bls12_381::Fr, ed_on_bls12_381_constants);
// impl_group_for_curve!(ark_ed_on_bls12_381::EdwardsProjective);

// impl_wrapped_field!(ark_ed_on_bls12_381_bandersnatch::Fr, bandersnatch_constants);
// impl_prime_field!(ark_ed_on_bls12_381_bandersnatch::Fr, bandersnatch_constants);
// impl_group_for_curve!(ark_ed_on_bls12_381_bandersnatch::EdwardsProjective);

// impl_wrapped_field!(ark_bn254::Fr, bn254_constants);
// impl_prime_field!(ark_bn254::Fr, bn254_constants);
// impl_group_for_curve!(ark_bn254::G1Projective);

// impl_wrapped_field!(ark_ed_on_bn254::Fr, ed_on_bn254_constants);
// impl_prime_field!(ark_ed_on_bn254::Fr, ed_on_bn254_constants);
// impl_group_for_curve!(ark_ed_on_bn254::EdwardsProjective);

// impl_wrapped_field!(ark_grumpkin::Fr, grumpkin_constants);
// impl_prime_field!(ark_grumpkin::Fr, grumpkin_constants);
// impl_group_for_curve!(ark_grumpkin::Projective);

// impl_wrapped_field!(ark_mnt4_298::Fr, mnt4_298_constants);
// impl_prime_field!(ark_mnt4_298::Fr, mnt4_298_constants);
// impl_group_for_curve!(ark_mnt4_298::G1Projective);

// impl_wrapped_field!(ark_mnt6_298::Fr, mnt6_298_constants);
// impl_prime_field!(ark_mnt6_298::Fr, mnt6_298_constants);
// impl_group_for_curve!(ark_mnt6_298::G1Projective);

// impl_wrapped_field!(ark_ed_on_mnt4_298::Fr, ed_on_mnt4_298_constants);
// impl_prime_field!(ark_ed_on_mnt4_298::Fr, ed_on_mnt4_298_constants);
// impl_group_for_curve!(ark_ed_on_mnt4_298::EdwardsProjective);

// impl_wrapped_field!(ark_mnt4_753::Fr, mnt4_753_constants);
// impl_prime_field!(ark_mnt4_753::Fr, mnt4_753_constants);
// impl_group_for_curve!(ark_mnt4_753::G1Projective);

// impl_wrapped_field!(ark_mnt6_753::Fr, mnt6_753_constants);
// impl_prime_field!(ark_mnt6_753::Fr, mnt6_753_constants);
// impl_group_for_curve!(ark_mnt6_753::G1Projective);

// impl_wrapped_field!(ark_ed_on_mnt4_753::Fr, ed_on_mnt4_753_constants);
// impl_prime_field!(ark_ed_on_mnt4_753::Fr, ed_on_mnt4_753_constants);
// impl_group_for_curve!(ark_ed_on_mnt4_753::EdwardsProjective);

// impl_wrapped_field!(ark_pallas::Fr, pallas_constants);
// impl_prime_field!(ark_pallas::Fr, pallas_constants);
// impl_group_for_curve!(ark_pallas::Projective);

// impl_wrapped_field!(ark_vesta::Fr, vesta_constants);
// impl_prime_field!(ark_vesta::Fr, vesta_constants);
// impl_group_for_curve!(ark_vesta::Projective);
