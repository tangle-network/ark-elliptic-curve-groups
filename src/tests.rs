use crate::{ArkGroupWrapper, ArkScalarWrapper};
use elliptic_curve::Field;

#[test]
fn run_gennaro_dkg() {
    use gennaro_dkg::*;
    use maplit::btreemap;
    use std::num::NonZeroUsize;
    use vsss_rs::{Share, combine_shares, elliptic_curve::Group};

    let parameters = Parameters::new(NonZeroUsize::new(2).unwrap(), NonZeroUsize::new(3).unwrap());

    let mut participant1 = SecretParticipant::<ArkGroupWrapper<ark_bls12_381::G1Projective>>::new(
        NonZeroUsize::new(1).unwrap(),
        parameters,
    )
    .unwrap();
    let mut participant2 = SecretParticipant::<ArkGroupWrapper<ark_bls12_381::G1Projective>>::new(
        NonZeroUsize::new(2).unwrap(),
        parameters,
    )
    .unwrap();
    let mut participant3 = SecretParticipant::<ArkGroupWrapper<ark_bls12_381::G1Projective>>::new(
        NonZeroUsize::new(3).unwrap(),
        parameters,
    )
    .unwrap();

    // Round 1
    let (b1data1, p2p1data) = participant1.round1().unwrap();
    let (b2data1, p2p2data) = participant2.round1().unwrap();
    let (b3data1, p2p3data) = participant3.round1().unwrap();

    // Can't call the same round twice
    assert!(participant1.round1().is_err());
    assert!(participant2.round1().is_err());
    assert!(participant3.round1().is_err());

    // Send b1data1 to secret_participant 2 and 3
    // Send b2data1 to secret_participant 1 and 3
    // Send b3data1 to secret_participant 1 and 2

    // Send p2p1data[&2] to secret_participant 2
    // Send p2p1data[&3] to secret_participant 3

    // Send p2p2data[&1] to secret_participant 1
    // Send p2p2data[&3] to secret_participant 3

    // Send p2p3data[&1] to secret_participant 1
    // Send p2p3data[&2] to secret_participant 2

    let p1bdata1 = btreemap! {
        2 => b2data1.clone(),
        3 => b3data1.clone(),
    };
    let p1pdata = btreemap! {
        2 => p2p2data[&1].clone(),
        3 => p2p3data[&1].clone(),
    };
    let b1data2 = participant1.round2(p1bdata1, p1pdata).unwrap();

    let p2bdata1 = btreemap! {
        1 => b1data1.clone(),
        3 => b3data1.clone(),
    };
    let p2pdata = btreemap! {
        1 => p2p1data[&2].clone(),
        3 => p2p3data[&2].clone(),
    };
    let b2data2 = participant2.round2(p2bdata1, p2pdata).unwrap();

    let p3bdata1 = btreemap! {
        1 => b1data1.clone(),
        2 => b2data1.clone(),
    };
    let p3pdata = btreemap! {
        1 => p2p1data[&3].clone(),
        2 => p2p2data[&3].clone(),
    };
    let b3data2 = participant3.round2(p3bdata1, p3pdata).unwrap();

    // Send b1data2 to participants 2 and 3
    // Send b2data2 to participants 1 and 3
    // Send b3data2 to participants 1 and 2

    // This is an optimization for the example in reality each secret_participant computes this
    let bdata2 = btreemap! {
        1 => b1data2,
        2 => b2data2,
        3 => b3data2,
    };

    let b1data3 = participant1.round3(&bdata2).unwrap();
    let b2data3 = participant2.round3(&bdata2).unwrap();
    let b3data3 = participant3.round3(&bdata2).unwrap();

    // Send b1data3 to participants 2 and 3
    // Send b2data3 to participants 1 and 3
    // Send b3data3 to participants 1 and 2

    // This is an optimization for the example in reality each secret_participant computes this
    let bdata3 = btreemap! {
        1 => b1data3,
        2 => b2data3,
        3 => b3data3,
    };

    let b1data4 = participant1.round4(&bdata3).unwrap();
    let b2data4 = participant2.round4(&bdata3).unwrap();
    let b3data4 = participant3.round4(&bdata3).unwrap();

    // Send b1data4 to participants 2 and 3
    // Send b2data4 to participants 1 and 3
    // Send b3data4 to participants 1 and 2

    // Verify that the same key is computed then done

    // This is an optimization for the example in reality each secret_participant computes this
    let bdata4 = btreemap! {
        1 => b1data4,
        2 => b2data4,
        3 => b3data4,
    };

    assert!(participant1.round5(&bdata4).is_ok());
    assert!(participant2.round5(&bdata4).is_ok());
    assert!(participant3.round5(&bdata4).is_ok());

    // Get the verification key
    let pk1 = participant1.get_public_key().unwrap();
    // Get the secret share
    let share1 = participant1.get_secret_share().unwrap();

    assert_eq!(pk1.is_identity().unwrap_u8(), 0u8);
    assert_eq!(share1.is_zero().unwrap_u8(), 0u8);

    let pk2 = participant2.get_public_key().unwrap();
    let share2 = participant2.get_secret_share().unwrap();

    assert_eq!(pk2.is_identity().unwrap_u8(), 0u8);
    assert_eq!(share2.is_zero().unwrap_u8(), 0u8);

    let pk3 = participant3.get_public_key().unwrap();
    let share3 = participant3.get_secret_share().unwrap();

    assert_eq!(pk3.is_identity().unwrap_u8(), 0u8);
    assert_eq!(share3.is_zero().unwrap_u8(), 0u8);

    // Public keys will be equal
    assert_eq!(pk1, pk2);
    assert_eq!(pk2, pk3);
    // Shares should not be
    assert_ne!(share1, share2);
    assert_ne!(share1, share3);
    assert_ne!(share2, share3);

    // For demonstration purposes, the shares if collected can be combined to recreate
    // the computed secret

    let s1 = <Vec<u8> as Share>::from_field_element(1u8, share1).unwrap();
    let s2 = <Vec<u8> as Share>::from_field_element(2u8, share2).unwrap();
    let s3 = <Vec<u8> as Share>::from_field_element(3u8, share3).unwrap();

    let sk: ArkScalarWrapper<ark_bls12_381::Fr> = combine_shares(&[s1, s2, s3]).unwrap();
    let computed_pk = ArkGroupWrapper::<ark_bls12_381::G1Projective>::generator() * sk;
    assert_eq!(computed_pk, pk1);
}
