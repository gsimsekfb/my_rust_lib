#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::four_forward_slashes)]

// mod doc_test; 
  // todo: failed to resolve: use of undeclared crate or module `doc_test`

#[cfg(test)]
mod integration_tests;

pub mod maze_1d; // pub because we this from examples
mod api;
mod array_slice_unpack;
mod async_;
mod averaged_collection;
mod break_;
mod cfg;
mod copy_vs_move;
mod ctor_chain;
mod compile_vs_runtime_read_env_and_file;
mod concurrency;
mod const_fn;
mod conversions;
mod default;
mod drop_aka_dtor;
mod r#enum;
mod env_logger;
mod env_vars;
mod error_handling;
mod file_sum_time_logs;
mod func_ptr;
mod int_overflow;
mod interior_mutability;
mod iterators_aka_algorithms;
mod let_else;
mod loop_multi;
mod match_;
mod mini_grep;
mod multi_threading_par_iter;
mod r#mut;
mod never_type_never_returns;
mod newtype_pattern;
mod option;
mod range;
mod re_export;
mod serde_json;
mod smart_ptrs;
mod string_concat;
mod unit_tested_file;
mod unit_integration_tests;
mod temp;
mod type_;
mod traits_and_generics;
mod trait_simple_vs_enum;
mod trait_vs_enum_in_vec;
mod traits_when_to_use_dyn_dispatch;
mod traits_object_vs_struct_obj;
mod traits;
mod traits_associated_constants;
mod traits_associated_type_vs_generics;
mod traits_default_type_parameter;
mod traits_return_impl; // todo: incomplete
mod traits_same_name_fn_call;
mod traits_supertraits;
mod utils;
mod variadic;
#[cfg(not(target_os = "windows"))]
mod zk_facebook_winterfell;

// re-exports
use crate::utils::print_type_of;
