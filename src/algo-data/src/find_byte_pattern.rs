// last: 4/26


// interv-1, interv-2
// - Find the position of matched bytes in a large data memory - code
// - find using idiomatic rust (optional)
// e.g. find 3,4,5 in 1,2,3,4,5 
// - "large data memory" part is interv-2, mention techniques, hint: at the end
// - ubuntu interv question





// -------------------------------------------------------------------







// 4. Large file optimizations: (read this last)
// e.g. For a 10 GB file
// 1. memmap2  →  load large file efficiently  
//    - MBs RAM usage instead of GB. Don't load entire file into RAM.
//      OS pages in only the parts being accessed. 
//      You only use a few kB of RAM at any given moment for the pages 
//      you are currently inspecting.
//      Avoids unnecessary data copying between the OS kernel and your program.
// + 2. rayon  →  parallel chunks
//     - parallel search across CPU cores
// + 3. memchr →  SIMD accelerated search per chunk  
//     - Process 16/32 bytes at once using CPU vector instructions

// 
// To search a 10GB file, 
// 1. use memmap2 to map the file into memory without exhausting RAM. 
// 2. then use Rayon to divide the map into parallel chunks, 
// 3. and within each chunk, use memchr to leverage SIMD instructions for 
// the actual byte-matching. To handle patterns that span across chunk 
// boundaries, I would overlap the chunks by pattern.len()-1 bytes.




// 1. Naive example

// e.g.
//      &[0,1,2,0,1,2,3u8], &[1,2,3u8];
// pos:     1     4
//
fn find_byte_pattern(bytes: &[u8], pattern: &[u8]) -> Option<usize> {
    println!("\n\n-- find_byte_pattern( b: {bytes:?} , p: {pattern:?} ) ...");
    if bytes.is_empty() || pattern.is_empty() { return None; }
    if pattern.len() > bytes.len() { return None; }
    for (i, b) in bytes.iter().enumerate() {
        for (k, pb) in pattern.iter().enumerate() {
            if &bytes[i + k] != pb { break };
            if k == pattern.len()-1 { // last byte, which also matched
                println!("-- found: index {i}");
                return Some(i) 
            }
        }
        println!("-- bytes[i] processed: {i} of {}", bytes.len());
    }
    None
}

// 2. 
fn idiomatic_rust(bytes: &[u8], pattern: &[u8]) -> Option<usize> {
    if pattern.is_empty() { return None; }
    bytes.windows(pattern.len()).position(|window| window == pattern)
        // windows(): "iter" for pattern length byte windows 
}

// 3. Boyer-More (bad character rule)
//    needed? too geeky? didn't like because need to memorize the rule 

// 4. See above

#[test]
fn t1() {
    assert_eq!( find_byte_pattern(&[0,1,2,0,1,2,3u8], &[1,2,3u8]), Some(4) );
    assert_eq!( find_byte_pattern(&[1,2,3,1,2u8], &[1,2,3u8]), Some(0) );

    assert_eq!( find_byte_pattern(&[1,2,3,1,2u8], &[1u8]), Some(0) );
    assert_eq!( find_byte_pattern(&[2,3,1,2u8], &[1u8]), Some(2) );
    assert_eq!( find_byte_pattern(&[2,3,1u8], &[1u8]), Some(2) );

    assert_eq!( find_byte_pattern(&[0,1u8], &[1,2,3,4u8]), None );
    assert_eq!( find_byte_pattern(&[1,2,3u8], &[1,2,3u8]), Some(0) );

    assert_eq!( find_byte_pattern(&[], &[1,2,3u8]), None );
    assert_eq!( find_byte_pattern(&[1], &[]), None );

    assert_eq!( find_byte_pattern(&[1,2,3,4u8], &[1,2,3u8]), Some(0) );

    //// ai tests
    
    // Refined Test Suite Additions
    assert_eq!(find_byte_pattern(&[1,1,1,2u8], &[1,1,2u8]), Some(1));    // Overlap
    assert_eq!(find_byte_pattern(&[1,2,1,2,3u8], &[1,2,3u8]), Some(2)); // False start
    assert_eq!(find_byte_pattern(&[0,0,1,2,3u8], &[1,2,3u8]), Some(2)); // End boundary
    assert_eq!(find_byte_pattern(&[1,1,1,1u8], &[1,1u8]), Some(0));     // Repeating

    // pattern longer than bytes
    assert_eq!( find_byte_pattern(&[1,2u8], &[1,2,3u8]), None );

    // pattern at exact end
    assert_eq!( find_byte_pattern(&[1,2,3u8], &[2,3u8]), Some(1) );

    // repeated overlapping pattern
    assert_eq!( find_byte_pattern(&[1,1,1u8], &[1,1u8]), Some(0) );

    // all same bytes, pattern not found
    assert_eq!( find_byte_pattern(&[1,1,1u8], &[2u8]), None );

    // single byte, not found
    assert_eq!( find_byte_pattern(&[1,2,3u8], &[4u8]), None );

    // both single byte, match
    assert_eq!( find_byte_pattern(&[1u8], &[1u8]), Some(0) );

    // both single byte, no match
    assert_eq!( find_byte_pattern(&[1u8], &[2u8]), None );
}



// Hint: 
// - Image searching bytes in a 10GB file
