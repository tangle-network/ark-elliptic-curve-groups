use ark_ff::{BigInteger, FftField, PrimeField};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("field_constants.rs");
    let mut f = File::create(dest_path).unwrap();

    // Generate constants for secp256k1
    generate_field_constants::<ark_secp256k1::Fr>(&mut f, "secp256k1_constants");

    // Generate constants for secp256r1
    generate_field_constants::<ark_secp256r1::Fr>(&mut f, "secp256r1_constants");

    // Generate constants for secp384r1
    generate_field_constants::<ark_secp384r1::Fr>(&mut f, "secp384r1_constants");

    // Generate constants for secq256k1
    generate_field_constants::<ark_secq256k1::Fr>(&mut f, "secq256k1_constants");

    // Generate constants for curve25519
    generate_field_constants::<ark_curve25519::Fr>(&mut f, "curve25519_constants");

    // Generate constants for ed25519
    generate_field_constants::<ark_ed25519::Fr>(&mut f, "ed25519_constants");

    // Generate constants for BLS12-377
    generate_field_constants::<ark_bls12_377::Fr>(&mut f, "bls12_377_constants");
    generate_field_constants::<ark_ed_on_bls12_377::Fr>(&mut f, "ed_on_bls12_377_constants");

    // Generate constants for BW6-761
    generate_field_constants::<ark_bw6_761::Fr>(&mut f, "bw6_761_constants");
    generate_field_constants::<ark_ed_on_bw6_761::Fr>(&mut f, "ed_on_bw6_761_constants");

    // Generate constants for BW6-767
    generate_field_constants::<ark_bw6_767::Fr>(&mut f, "bw6_767_constants");

    // Generate constants for CP6-782
    generate_field_constants::<ark_cp6_782::Fr>(&mut f, "cp6_782_constants");
    generate_field_constants::<ark_ed_on_cp6_782::Fr>(&mut f, "ed_on_cp6_782_constants");

    // Generate constants for BLS12-381
    generate_field_constants::<ark_bls12_381::Fr>(&mut f, "bls12_381_constants");
    generate_field_constants::<ark_ed_on_bls12_381::Fr>(&mut f, "ed_on_bls12_381_constants");
    generate_field_constants::<ark_ed_on_bls12_381_bandersnatch::Fr>(
        &mut f,
        "bandersnatch_constants",
    );

    // Generate constants for BN254
    generate_field_constants::<ark_bn254::Fr>(&mut f, "bn254_constants");
    generate_field_constants::<ark_ed_on_bn254::Fr>(&mut f, "ed_on_bn254_constants");
    generate_field_constants::<ark_grumpkin::Fr>(&mut f, "grumpkin_constants");

    // Generate constants for MNT4/6-298
    generate_field_constants::<ark_mnt4_298::Fr>(&mut f, "mnt4_298_constants");
    generate_field_constants::<ark_mnt6_298::Fr>(&mut f, "mnt6_298_constants");
    generate_field_constants::<ark_ed_on_mnt4_298::Fr>(&mut f, "ed_on_mnt4_298_constants");

    // Generate constants for MNT4/6-753
    generate_field_constants::<ark_mnt4_753::Fr>(&mut f, "mnt4_753_constants");
    generate_field_constants::<ark_mnt6_753::Fr>(&mut f, "mnt6_753_constants");
    generate_field_constants::<ark_ed_on_mnt4_753::Fr>(&mut f, "ed_on_mnt4_753_constants");

    // Generate constants for Pallas/Vesta
    generate_field_constants::<ark_pallas::Fr>(&mut f, "pallas_constants");
    generate_field_constants::<ark_vesta::Fr>(&mut f, "vesta_constants");
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
