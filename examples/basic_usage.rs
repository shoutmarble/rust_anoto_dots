//! Basic example demonstrating encoding and decoding with the Anoto codec.

use ndarray::s;
use rust_anoto_dots::defaults;

fn main() {
    println!("=== Anoto Codec Demo ===\n");

    // Use the default embodiment with A4 sequence fixed
    let codec = defaults::anoto_6x6_a4_fixed();
    println!("Created codec with MNS order: {}", codec.mns_order);

    // Generate a bit-matrix for section (10, 2)
    let section = (10, 2);
    println!("\nEncoding bit-matrix for section: {:?}", section);
    let g = codec.encode_bitmatrix((20, 30), section);
    println!("Generated bit-matrix of shape: {:?}", g.dim());

    // Decode a partial matrix at position (7, 3)
    let test_pos = (7, 3);
    println!("\nDecoding from position: {:?}", test_pos);
    
    let sub = g.slice(s![test_pos.1..test_pos.1 + 6, test_pos.0..test_pos.0 + 6, ..]).to_owned();
    
    let pos = codec.decode_position(&sub).unwrap();
    println!("  Decoded position: {:?}", pos);
    assert_eq!(pos, test_pos, "Position mismatch!");
    
    let sec = codec.decode_section(&sub, pos).unwrap();
    println!("  Decoded section: {:?}", sec);
    assert_eq!(sec, section, "Section mismatch!");

    // Decode rotation from a larger matrix
    println!("\nTesting rotation detection...");
    let r = g.slice(s![3..11, 7..15, ..]).to_owned();
    let rot = codec.decode_rotation(&r).unwrap();
    println!("  Rotation: {} (0 = canonical orientation)", rot);
    assert_eq!(rot, 0, "Expected canonical orientation");

    // Test with rotated pattern
    let rotated = rust_anoto_dots::helpers::rot90(&r, 1);
    let rot = codec.decode_rotation(&rotated).unwrap();
    println!("  90° CCW rotation: {}", rot);
    assert_eq!(rot, 1, "Expected 90° rotation");

    println!("\n=== All tests passed! ===");
}
