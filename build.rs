use ark_ec::CurveGroup;
use ark_ff::{BigInteger, FftField, PrimeField};
use ark_serialize::CanonicalSerialize;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn generate_curve_constants<G: CurveGroup>(f: &mut File, prefix: &str) {
    writeln!(f, "#[allow(dead_code)]").unwrap();
    writeln!(f, "pub mod {} {{", prefix).unwrap();

    let generator = G::generator();
    let mut buf = Vec::new();
    generator.serialize_compressed(&mut buf).unwrap();
    let compressed_size = <G as CanonicalSerialize>::compressed_size(&generator);

    writeln!(f, "    #[allow(dead_code)]").unwrap();
    writeln!(
        f,
        "    pub const COMPRESSED_POINT_SIZE: usize = {};",
        compressed_size
    )
    .unwrap();
    writeln!(f, "    #[allow(dead_code)]").unwrap();
    writeln!(f, "    pub const GENERATOR_COMPRESSED: &[u8] = &{:?};", buf).unwrap();

    writeln!(f, "}}").unwrap();
}

fn generate_field_constants<F: PrimeField + FftField>(f: &mut File, prefix: &str) {
    writeln!(f, "#[allow(dead_code)]").unwrap();
    writeln!(f, "pub mod {} {{", prefix).unwrap();

    // Write modulus and basic field parameters
    writeln!(f, "    #[allow(dead_code)]").unwrap();
    writeln!(f, "    pub const MODULUS: &str = \"{}\";", F::MODULUS).unwrap();
    writeln!(f, "    #[allow(dead_code)]").unwrap();
    writeln!(f, "    pub const NUM_BITS: u32 = {};", F::MODULUS_BIT_SIZE).unwrap();
    writeln!(f, "    #[allow(dead_code)]").unwrap();
    writeln!(
        f,
        "    pub const CAPACITY: u32 = {};",
        F::MODULUS_BIT_SIZE - 1
    )
    .unwrap();
    writeln!(f, "    #[allow(dead_code)]").unwrap();
    writeln!(f, "    pub const TWO_ADICITY: u32 = {};", F::TWO_ADICITY).unwrap();

    // Write field element constants
    let zero = F::zero();
    let one = F::one();

    writeln!(f, "    #[allow(dead_code)]").unwrap();
    writeln!(
        f,
        "    pub const ZERO: [u8; {}] = {:?};",
        std::mem::size_of::<F>(),
        zero.into_bigint().to_bytes_be()
    )
    .unwrap();
    writeln!(f, "    #[allow(dead_code)]").unwrap();
    writeln!(
        f,
        "    pub const ONE: [u8; {}] = {:?};",
        std::mem::size_of::<F>(),
        one.into_bigint().to_bytes_be()
    )
    .unwrap();

    // Write other constants as byte arrays
    let two_inv = F::from(2u64).inverse().unwrap();
    let generator = F::GENERATOR;
    let root = F::TWO_ADIC_ROOT_OF_UNITY;
    let root_inv = root.inverse().unwrap();

    writeln!(f, "    #[allow(dead_code)]").unwrap();
    writeln!(
        f,
        "    pub const TWO_INV: [u8; {}] = {:?};",
        std::mem::size_of::<F::BigInt>(),
        two_inv.into_bigint().to_bytes_be()
    )
    .unwrap();
    writeln!(f, "    #[allow(dead_code)]").unwrap();
    writeln!(
        f,
        "    pub const GENERATOR: [u8; {}] = {:?};",
        std::mem::size_of::<F::BigInt>(),
        generator.into_bigint().to_bytes_be()
    )
    .unwrap();
    writeln!(f, "    #[allow(dead_code)]").unwrap();
    writeln!(
        f,
        "    pub const ROOT_OF_UNITY: [u8; {}] = {:?};",
        std::mem::size_of::<F::BigInt>(),
        root.into_bigint().to_bytes_be()
    )
    .unwrap();
    writeln!(f, "    #[allow(dead_code)]").unwrap();
    writeln!(
        f,
        "    pub const ROOT_OF_UNITY_INV: [u8; {}] = {:?};",
        std::mem::size_of::<F::BigInt>(),
        root_inv.into_bigint().to_bytes_be()
    )
    .unwrap();

    // Calculate DELTA = MULTIPLICATIVE_GENERATOR^(2^S)
    let two_pow_s = F::from(2u64).pow([F::TWO_ADICITY as u64]);
    let delta = F::GENERATOR.pow(two_pow_s.into_bigint().as_ref());

    writeln!(f, "    #[allow(dead_code)]").unwrap();
    writeln!(
        f,
        "    pub const DELTA: [u8; {}] = {:?};",
        std::mem::size_of::<F::BigInt>(),
        delta.into_bigint().to_bytes_be()
    )
    .unwrap();

    writeln!(f, "}}").unwrap();
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    // Generate curve constants
    let curve_path = Path::new(&out_dir).join("curve_constants.rs");
    let mut curve_file = File::create(&curve_path).unwrap();

    generate_curve_constants::<ark_secp256k1::Projective>(
        &mut curve_file,
        "secp256k1_projective_constants",
    );
    generate_curve_constants::<ark_secp256r1::Projective>(
        &mut curve_file,
        "secp256r1_projective_constants",
    );
    generate_curve_constants::<ark_secp384r1::Projective>(
        &mut curve_file,
        "secp384r1_projective_constants",
    );
    generate_curve_constants::<ark_secq256k1::Projective>(
        &mut curve_file,
        "secq256k1_projective_constants",
    );
    generate_curve_constants::<ark_curve25519::EdwardsProjective>(
        &mut curve_file,
        "curve25519_projective_constants",
    );
    generate_curve_constants::<ark_ed25519::EdwardsProjective>(
        &mut curve_file,
        "ed25519_projective_constants",
    );
    generate_curve_constants::<ark_bw6_767::G1Projective>(&mut curve_file, "bw6_767_g1_constants");
    generate_curve_constants::<ark_bw6_767::G2Projective>(&mut curve_file, "bw6_767_g2_constants");
    generate_curve_constants::<ark_cp6_782::G1Projective>(&mut curve_file, "cp6_782_g1_constants");
    generate_curve_constants::<ark_cp6_782::G2Projective>(&mut curve_file, "cp6_782_g2_constants");
    generate_curve_constants::<ark_bls12_377::G1Projective>(
        &mut curve_file,
        "bls12_377_g1_constants",
    );
    generate_curve_constants::<ark_bls12_377::G2Projective>(
        &mut curve_file,
        "bls12_377_g2_constants",
    );
    generate_curve_constants::<ark_bls12_381::G1Projective>(
        &mut curve_file,
        "bls12_381_g1_constants",
    );
    generate_curve_constants::<ark_bls12_381::G2Projective>(
        &mut curve_file,
        "bls12_381_g2_constants",
    );
    generate_curve_constants::<ark_bn254::G1Projective>(&mut curve_file, "bn254_g1_constants");
    generate_curve_constants::<ark_bn254::G2Projective>(&mut curve_file, "bn254_g2_constants");
    generate_curve_constants::<ark_bw6_761::G1Projective>(&mut curve_file, "bw6_761_g1_constants");
    generate_curve_constants::<ark_bw6_761::G2Projective>(&mut curve_file, "bw6_761_g2_constants");
    generate_curve_constants::<ark_ed_on_bls12_377::EdwardsProjective>(
        &mut curve_file,
        "ed_on_bls12_377_constants",
    );
    generate_curve_constants::<ark_ed_on_bls12_381::EdwardsProjective>(
        &mut curve_file,
        "ed_on_bls12_381_constants",
    );
    generate_curve_constants::<ark_ed_on_bw6_761::EdwardsProjective>(
        &mut curve_file,
        "ed_on_bw6_761_constants",
    );
    generate_curve_constants::<ark_ed_on_cp6_782::EdwardsProjective>(
        &mut curve_file,
        "ed_on_cp6_782_constants",
    );
    generate_curve_constants::<ark_ed_on_mnt4_298::EdwardsProjective>(
        &mut curve_file,
        "ed_on_mnt4_298_constants",
    );
    generate_curve_constants::<ark_ed_on_mnt4_753::EdwardsProjective>(
        &mut curve_file,
        "ed_on_mnt4_753_constants",
    );

    // Generate field constants
    let field_path = Path::new(&out_dir).join("field_constants.rs");
    let mut field_file = File::create(&field_path).unwrap();

    generate_field_constants::<ark_secp256k1::Fr>(&mut field_file, "secp256k1_constants");
    generate_field_constants::<ark_secp256r1::Fr>(&mut field_file, "secp256r1_constants");
    generate_field_constants::<ark_secp384r1::Fr>(&mut field_file, "secp384r1_constants");
    generate_field_constants::<ark_secq256k1::Fr>(&mut field_file, "secq256k1_constants");
    generate_field_constants::<ark_curve25519::Fr>(&mut field_file, "curve25519_constants");
    generate_field_constants::<ark_ed25519::Fr>(&mut field_file, "ed25519_constants");

    generate_field_constants::<ark_bls12_377::Fr>(&mut field_file, "bls12_377_fr_constants");
    generate_field_constants::<ark_ed_on_bls12_377::Fr>(
        &mut field_file,
        "ed_on_bls12_377_fr_constants",
    );
    generate_field_constants::<ark_bw6_761::Fr>(&mut field_file, "bw6_761_fr_constants");
    generate_field_constants::<ark_ed_on_bw6_761::Fr>(
        &mut field_file,
        "ed_on_bw6_761_fr_constants",
    );
    generate_field_constants::<ark_bw6_767::Fr>(&mut field_file, "bw6_767_fr_constants");
    generate_field_constants::<ark_cp6_782::Fr>(&mut field_file, "cp6_782_fr_constants");
    generate_field_constants::<ark_ed_on_cp6_782::Fr>(
        &mut field_file,
        "ed_on_cp6_782_fr_constants",
    );
    generate_field_constants::<ark_bls12_381::Fr>(&mut field_file, "bls12_381_fr_constants");
    generate_field_constants::<ark_ed_on_bls12_381::Fr>(
        &mut field_file,
        "ed_on_bls12_381_fr_constants",
    );
    generate_field_constants::<ark_ed_on_bls12_381_bandersnatch::Fr>(
        &mut field_file,
        "bandersnatch_fr_constants",
    );
    generate_field_constants::<ark_bn254::Fr>(&mut field_file, "bn254_fr_constants");
    generate_field_constants::<ark_ed_on_bn254::Fr>(&mut field_file, "ed_on_bn254_fr_constants");
    generate_field_constants::<ark_grumpkin::Fr>(&mut field_file, "grumpkin_fr_constants");
    generate_field_constants::<ark_mnt4_298::Fr>(&mut field_file, "mnt4_298_fr_constants");
    generate_field_constants::<ark_mnt6_298::Fr>(&mut field_file, "mnt6_298_fr_constants");
    generate_field_constants::<ark_ed_on_mnt4_298::Fr>(
        &mut field_file,
        "ed_on_mnt4_298_fr_constants",
    );
    generate_field_constants::<ark_mnt4_753::Fr>(&mut field_file, "mnt4_753_fr_constants");
    generate_field_constants::<ark_mnt6_753::Fr>(&mut field_file, "mnt6_753_fr_constants");
    generate_field_constants::<ark_ed_on_mnt4_753::Fr>(
        &mut field_file,
        "ed_on_mnt4_753_fr_constants",
    );
    generate_field_constants::<ark_pallas::Fr>(&mut field_file, "pallas_fr_constants");
    generate_field_constants::<ark_vesta::Fr>(&mut field_file, "vesta_fr_constants");
}
