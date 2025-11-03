#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::four_forward_slashes)]
#![allow(clippy::empty_line_after_doc_comments)]

// mod doc_test; 
  // todo: failed to resolve: use of undeclared crate or module `doc_test`

#[cfg(test)]
mod integration_tests;

mod api;
mod array;
mod array_slice_unpack;
mod async_;
mod async_closure;
mod averaged_collection;
mod break_;
mod cargo_expand;
mod catch_unwind;
mod cfg;
mod closure;
mod copy_vs_move;
mod const_generics_lifetimes;
mod cow;
mod ctor_chain;
mod compile_vs_runtime_read_env_and_file;
mod concurrency;
mod const_fn;
mod conversions;
mod default;
mod drop_aka_dtor;
mod doc_test;
mod r#dyn;
mod r#enum;
mod env_logger;
mod environment_variable;
mod error_handling;
mod func_ptr;
mod generics;
mod hash_map_set;
mod int_overflow;
mod interior_mutability;
mod iterators_aka_algorithms;
mod let_else;
mod lifetimes;
mod loop_multi;
mod match_;
mod memory_leak;
mod mini_grep;
mod multi_threading_par_iter;
mod r#mut;
mod never_type_never_returns;
mod opaque_types;
mod option;
mod question_operator;
mod struct_basics_enum_trait_player;
mod random_file;
mod range;
mod re_export;
mod serde_json;
mod smart_ptrs;
mod sum_of_times_in_log_file;
mod str_dynamically_sized_types_dst;
mod string_concat;
mod test_unit_tested_file;
mod test_integration_tests;
mod temp;
mod tokio_channels;
mod tokio_w_mutex_rwlock;
mod tokio_pin;
mod tokio_select_join_try_join;
mod tokio_spawn_vs_spawn_blocking;
mod trait_simple_vs_enum;
mod trait_vs_enum_polymorphism;
mod traits_and_generics;
mod traits_conditionally_impl_methods;
mod traits_dynamically_sized_types_dst;
mod traits_static_vs_dyn_dispatch;
mod traits_associated_constants;
mod traits_associated_type_vs_generics;
mod traits_default_type_parameter;
mod traits_return_impl;
mod traits_orphan_rule_newtype_pattern;
mod traits_same_name_fn_call;
mod traits_supertraits;
mod type_;
mod unpack;
mod unsafe_;
mod utils;
mod variadic;
#[cfg(not(target_os = "windows"))]
mod zk_facebook_winterfell;

// re-exports
use crate::utils::print_type_of;
