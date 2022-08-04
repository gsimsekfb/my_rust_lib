pub mod kinds {
    pub const KK: u32 = 42;
}

pub mod utils {
    pub const UU: u32 = 42;
}

pub use kinds::KK; // KK re-exported
pub use utils::UU; // UU re-exported

#[test]
fn ex1() {
  assert_eq!(kinds::KK, utils::UU); // w/o reexporting
  assert_eq!(KK, UU);               // w/  reexporting
}