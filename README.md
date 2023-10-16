# rust-my-templates

This repo is for my personal education and use with ready to use examples/explanations of features/keywords of Rust and best practices.  

Important tools:   
> clippy the linter: https://github.com/rust-lang/rust-clippy  
> rust-analyzer: https://github.com/rust-lang/rust-analyzer  
> sccache for faster builds (see setup and how to use under section "docs" below)  
> vscode debugger (see setup and examples under section "docs" below)  
  
[docs/](docs)  
> [faster_builds_with_linker_config.md](docs/faster_builds_with_linker_config.md)   
> [faster_builds_with_sccache.md](docs/faster_builds_with_sccache.md)  
> [how_to_debug_rust.md](docs/how_to_debug_rust.md)  

[src/](src)  
> [async_.rs](src/async_.rs)  
> [averaged_collection.rs](src/averaged_collection.rs)  
> [cacher.rs](src/cacher.rs)  
> [clap-cmd-line-parser](src/clap-cmd-line-parser)  
> [compile_vs_runtime_read_env_and_file.rs](src/compile_vs_runtime_read_env_and_file.rs)  
> [concurrency.rs](src/concurrency.rs)  
> [const_fn.rs](src/const_fn.rs)  
> [constructor_chain.rs](src/ctor_chain.rs)  
> [doc_test.rs](src/doc_test.rs)  
> [drop_aka_dtor.rs](src/drop_aka_dtor.rs)  
> [dynamically_sized_types.rs](src/dynamically_sized_types.rs)  
> [enum.rs](src/enum.rs)  
> [error_handling.rs](src/error_handling.rs)  
> [func_ptr.rs](src/func_ptr.rs)  
> [int_overflow.rs](src/int_overflow.rs)  
> [interior_mutability.rs](src/interior_mutability.rs)  
> [iterators.rs](src/iterators.rs)  
> [match_.rs](src/match_.rs)  
> [never_type_never_returns.rs](src/never_type_never_returns.rs)  
> [newtype_pattern.rs](src/newtype_pattern.rs)
> [re_export.rs](src/re_export.rs)  
> [serde_json.rs](src/serde_json.rs)  
> [smart_ptrs.rs](src/smart_ptrs.rs)  
> [string_concat.rs](src/string_concat.rs)  
> [trait_object_vs_struct_obj.rs](src/trait_object_vs_struct_obj.rs)  
> [traits_associated_type_vs_generics.rs](src/traits_associated_type_vs_generics.rs)  
> [traits_default_type_parameter.rs](src/traits_default_type_parameter.rs)  
> [traits_same_name_fn_call.rs](src/traits_same_name_fn_call.rs)  
> [traits_supertraits.rs](src/traits_supertraits.rs)  
> [type_.rs](src/type_.rs)  
> [unit_integration_tests.rs](src/unit_integration_tests.rs)  
> [unit_test.rs](src/unit_test.rs)  
> [web-server-async-and-parallel](src/web-server-async-and-parallel)  
> [web-server-multi-threaded](src/web-server-multi-threaded)  


Sources:  
- Rust Programming Language  by Steve Klabnik and Carol Nichols (Covers Rust 2018) - 2019
- Rust_in_Action_McNamara_Tim - 2021
- Rust for Rustaceans Idiomatic Programming for Experienced Developers - 2021 - by Jon Gjengset


### What's more in this repo?  
- Organization of main.rs, lib.rs and unit tests
- Working with command line arguments
- Working with environment variables
- Simple Cacher struct -> [src/cacher.rs](src/cacher.rs)
- Examples of using closures -> [src/closure.rs](src/closure.rs)
- Debugging, `.vscode/launch.json` template
- `.cargo/config.toml` sample, including fastest linkers, very&more useful for big codebases
- Read from file, find lines that contains a string
- And more... see -> [src/*.rs](src/)
