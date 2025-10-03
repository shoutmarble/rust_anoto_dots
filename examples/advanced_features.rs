//! Advanced example showing various codec features and error handling.

use ndarray::s;
use rust_anoto_dots::defaults;

fn main() {
    println!("=== Advanced Anoto Codec Features ===\n");

    // Create codecs
    let codec_fixed = defaults::anoto_6x6_a4_fixed();
    println!("Created codec with fixed A4 sequence");

    // Example 1: Encode and decode multiple sections
    println!("\n1. Testing multiple sections:");
    let sections = vec![(0, 0), (5, 10), (20, 30), (50, 50)];
    
    for section in &sections {
        let m = codec_fixed.encode_bitmatrix((40, 40), *section);
        let sub = m.slice(s![5..11, 5..11, ..]).to_owned();
        let pos = codec_fixed.decode_position(&sub).unwrap();
        let decoded_sec = codec_fixed.decode_section(&sub, pos).unwrap();
        println!("  Section {:?}: Encoded and decoded successfully", section);
        assert_eq!(decoded_sec, *section);
    }

    // Example 2: Test different positions within the same section
    println!("\n2. Testing different positions in section (10, 5):");
    let m = codec_fixed.encode_bitmatrix((50, 50), (10, 5));
    
    let test_positions = vec![(0, 0), (10, 10), (20, 20), (30, 30)];
    for pos in &test_positions {
        if pos.0 + 6 <= 50 && pos.1 + 6 <= 50 {
            let sub = m.slice(s![pos.1..pos.1 + 6, pos.0..pos.0 + 6, ..]).to_owned();
            let decoded = codec_fixed.decode_position(&sub).unwrap();
            println!("  Expected: {:?}, Decoded: {:?} ✓", pos, decoded);
            assert_eq!(decoded, *pos);
        }
    }

    // Example 3: Rotation detection
    println!("\n3. Testing all rotation angles:");
    let m = codec_fixed.encode_bitmatrix((30, 30), (7, 3));
    let sample = m.slice(s![5..13, 5..13, ..]).to_owned();
    
    for k in 0..4 {
        let rotated = rust_anoto_dots::helpers::rot90(&sample, k);
        let detected_rot = codec_fixed.decode_rotation(&rotated).unwrap();
        println!("  Rotated by {}×90°: detected rotation = {}", k, detected_rot);
        assert_eq!(detected_rot, k as usize);
    }

    // Example 4: Error handling
    println!("\n4. Testing error handling:");
    
    // Test with matrix that's too small
    let small = ndarray::Array3::<i8>::zeros((4, 4, 2));
    match codec_fixed.decode_position(&small) {
        Ok(_) => println!("  ERROR: Should have failed with small matrix"),
        Err(e) => println!("  Small matrix correctly rejected: {}", e),
    }
    
    // Test with wrong number of channels
    let wrong_channels = ndarray::Array3::<i8>::zeros((6, 6, 3));
    match codec_fixed.decode_position(&wrong_channels) {
        Ok(_) => println!("  ERROR: Should have failed with wrong channels"),
        Err(e) => println!("  Wrong channels correctly rejected: {}", e),
    }

    // Example 5: Large pattern generation
    println!("\n5. Generating large pattern:");
    let large = codec_fixed.encode_bitmatrix((200, 200), (15, 25));
    println!("  Generated {}×{} pattern successfully", large.dim().0, large.dim().1);
    
    // Sample a few random positions
    let sample_positions = vec![(50, 50), (100, 100), (150, 150)];
    for pos in &sample_positions {
        let sub = large.slice(s![pos.1..pos.1 + 6, pos.0..pos.0 + 6, ..]).to_owned();
        let decoded = codec_fixed.decode_position(&sub).unwrap();
        let decoded_sec = codec_fixed.decode_section(&sub, decoded).unwrap();
        println!("  Position {:?}: decoded correctly with section {:?}", pos, decoded_sec);
        assert_eq!(decoded, *pos);
        assert_eq!(decoded_sec, (15, 25));
    }

    println!("\n=== All advanced tests passed! ===");
}
