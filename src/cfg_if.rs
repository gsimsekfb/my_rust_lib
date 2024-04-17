// cfg blocks
// https://crates.io/crates/cfg-if

// lib.rs
cfg_if::cfg_if! {
    if #[cfg(test)] {
        pub mod integration_tests;
        pub const TEST_PATH: &'static str = "src/integration_tests/";
    } else {
    }
}

// calc_engine.rs
cfg_if::cfg_if! {
    if #[cfg(test)] {
        use crate::utils::assert_nodes_rust_vs_ts_test;
        use crate::TEST_PATH;
    } else { }
}