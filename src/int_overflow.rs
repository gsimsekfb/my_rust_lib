// interv

// Book: Rust_in_Action_2021_McNamara_Tim
// Listing 5.3 Exploring the effect of incrementing an integer past its range

#[should_panic] // comment this line to see the panic
#[test] fn ex_1_panics_w_int_overflow() {
    let mut i: u16 = 0;
    print!("{}-", i);
    loop {
        i += 1000;
        print!("{}-", i);
        if i % 10000 == 0 {
            println!{}
        }
    }
    // info: let sixty_five_thousand_535: u16 = 0b1111_1111_1111_1111;
}
// cout:
// 0-1000-2000-3000-4000-5000-6000-7000-8000-9000-10000-
// 11000-12000-13000-14000-15000-16000-17000-18000-19000-20000-
// 21000-22000-23000-24000-25000-26000-27000-28000-29000-30000-
// 31000-32000-33000-34000-35000-36000-37000-38000-39000-40000-
// 41000-42000-43000-44000-45000-46000-47000-48000-49000-50000-
// 51000-52000-53000-54000-55000-56000-57000-58000-59000-60000-
// 61000-62000-63000-64000-65000-
// thread 'int_overflow::ex_1_panics_w_overflow' panicked at 'attempt to 
// add with overflow', src/int_overflow.rs:6:9

#[should_panic] // comment this line to see the panic
#[allow(arithmetic_overflow)] // compile error without this
#[test] fn ex_2_panics_w_arithmetic_overflow() {
    let (a, b) = (200, 200);
    let c: u8 = a + b;
    println!("200 + 200 = {}", c);
}
// error: this arithmetic operation will overflow
//   --> src/int_overflow.rs:31:17
//    |
// 31 |     let c: u8 = a + b;
//    |                 ^^^^^ attempt to compute `200_u8 + 200_u8`, 
// which would overflow