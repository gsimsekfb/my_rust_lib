// interv-1

// Book: Rust_in_Action_2021_McNamara_Tim
// Listing 5.3 Exploring the effect of incrementing an integer past its range

#[should_panic] // comment this line to see the panic
#[allow(arithmetic_overflow)] // compile error without this
#[test] fn ex_1_panics_w_arithmetic_overflow_unsigned() {
    let (a, b) = (200, 200);
    let c: u8 = a + b;
    println!("200 + 200 = {c}");
}
// error: this arithmetic operation will overflow
//   --> src/int_overflow.rs:31:17
//    |
// 31 |     let c: u8 = a + b;
//    |                 ^^^^^ attempt to compute `200_u8 + 200_u8`, 
// which would overflow

#[should_panic] // comment this line to see the panic
#[allow(arithmetic_overflow)] // compile error without this
#[test] fn ex_2_panics_w_arithmetic_overflow_signed() {
    let (a, b) = (100, 100);
    let c: i8 = a + b;
    println!("200 + 200 = {c}");    
}
