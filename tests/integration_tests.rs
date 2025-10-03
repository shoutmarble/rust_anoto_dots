//! Comprehensive integration tests for the Anoto codec.

use ndarray::s;
use rust_anoto_dots::defaults;

#[test]
fn test_bitmatrix_encode_different_sections() {
    let anoto = defaults::anoto_6x6_a4_fixed();

    let m = anoto.encode_bitmatrix((60, 60), (0, 0));
    assert_eq!(m.dim(), (60, 60, 2));
    assert_eq!(
        m.slice(s![..8, 0, 0]).to_vec(),
        vec![0, 0, 0, 0, 0, 0, 1, 0]
    );
    assert_eq!(
        m.slice(s![0, ..8, 1]).to_vec(),
        vec![0, 0, 0, 0, 0, 0, 1, 0]
    );

    let m = anoto.encode_bitmatrix((60, 60), (1, 1));
    assert_eq!(m.dim(), (60, 60, 2));
    assert_eq!(
        m.slice(s![..8, 0, 0]).to_vec(),
        vec![0, 0, 0, 0, 0, 1, 0, 0]
    );
    assert_eq!(
        m.slice(s![0, ..8, 1]).to_vec(),
        vec![0, 0, 0, 0, 0, 1, 0, 0]
    );
}

#[test]
fn test_bitmatrix_decode_position() {
    let anoto = defaults::anoto_6x6_a4_fixed();

    let test_cases = vec![(256, (0, 0)), (256, (10, 5)), (256, (5, 10))];

    for (size, section) in test_cases {
        let m = anoto.encode_bitmatrix((size, size), section);
        assert_eq!(m.dim(), (size, size, 2));

        // Test every 10th position to balance thoroughness with performance
        for y in (0..size - 6).step_by(10) {
            for x in (0..size - 6).step_by(10) {
                let sub = m.slice(s![y..y + 6, x..x + 6, ..]).to_owned();
                let xy = anoto.decode_position(&sub).unwrap();
                assert_eq!(xy, (x, y), "Failed at position ({}, {})", x, y);

                let sec = anoto.decode_section(&sub, xy).unwrap();
                assert_eq!(sec, section, "Failed section at position ({}, {})", x, y);
            }
        }
    }
}

#[test]
fn test_bitmatrix_decode_rotation() {
    let anoto = defaults::anoto_6x6_a4_fixed();
    let m = anoto.encode_bitmatrix((256, 256), (5, 10));

    // Test rotations at various positions
    for i in (0..128 - 8).step_by(20) {
        for j in (0..128 - 8).step_by(20) {
            let s = m.slice(s![i..i + 8, j..j + 8, ..]).to_owned();

            assert_eq!(
                anoto.decode_rotation(&s).unwrap(),
                0,
                "Failed at position ({}, {})",
                i,
                j
            );

            let r1 = rust_anoto_dots::helpers::rot90(&s, 1);
            assert_eq!(
                anoto.decode_rotation(&r1).unwrap(),
                1,
                "Failed rotation 1 at position ({}, {})",
                i,
                j
            );

            let r2 = rust_anoto_dots::helpers::rot90(&s, 2);
            assert_eq!(
                anoto.decode_rotation(&r2).unwrap(),
                2,
                "Failed rotation 2 at position ({}, {})",
                i,
                j
            );

            let r3 = rust_anoto_dots::helpers::rot90(&s, 3);
            assert_eq!(
                anoto.decode_rotation(&r3).unwrap(),
                3,
                "Failed rotation 3 at position ({}, {})",
                i,
                j
            );
        }
    }
}

#[test]
fn test_decode_errors() {
    let anoto = defaults::anoto_6x6_a4_fixed();

    // Test with wrong sized matrix
    let small_matrix = ndarray::Array3::<i8>::zeros((3, 3, 2));
    assert!(anoto.decode_position(&small_matrix).is_err());

    // Test with wrong number of channels
    let wrong_channels = ndarray::Array3::<i8>::zeros((6, 6, 3));
    assert!(anoto.decode_position(&wrong_channels).is_err());
}

#[test]
fn test_encode_decode_round_trip() {
    let anoto = defaults::anoto_6x6_a4_fixed();

    // Test multiple section coordinates
    let sections = vec![(0, 0), (1, 1), (10, 5), (20, 30)];

    for section in sections {
        let m = anoto.encode_bitmatrix((50, 50), section);

        // Pick a few positions to test
        for y in [5, 15, 25].iter() {
            for x in [5, 15, 25].iter() {
                let sub = m.slice(s![*y..*y + 6, *x..*x + 6, ..]).to_owned();
                let decoded_pos = anoto.decode_position(&sub).unwrap();
                let decoded_sec = anoto.decode_section(&sub, decoded_pos).unwrap();

                assert_eq!(decoded_pos, (*x, *y));
                assert_eq!(decoded_sec, section);
            }
        }
    }
}
