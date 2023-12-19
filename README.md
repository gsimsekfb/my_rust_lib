# rust-my-templates

This repo is for my personal use with tips, tools, ready to use examples/explanations of features/keywords of Rust and best practices.  

#### Important tools & tips:
> Use latest Rust -> https://github.com/rust-lang/rust/releases  
> Use [clippy - the Rust linter](https://github.com/rust-lang/rust-clippy) (also use it in CI: [clippy.yml](.github/workflows/clippy.yml))  
> Use [rust-analyzer](https://github.com/rust-lang/rust-analyzer)  
> Use [integration_tests](src/unit_integration_tests.rs) (also use it in CI: [integration_tests.yml](.github/workflows/integration_tests.yml))   
> Use [unit_test](src/unit_tested_file.rs) (also use it in CI: [unit_tests.yml](.github/workflows/unit_tests.yml))   
> Use `sccache` for faster builds (see setup and how to use in section "docs" below)  
> Use [.cargo/config.toml](.cargo/config.toml) (see `faster_builds_with_linker_config` in section "docs" below)  
> Use `vscode debugger` (see setup and examples in section "docs" below)  

#### [best-practice-tips/](best-practices)  

#### [docs/](docs)  
> [faster_builds_with_linker_config.md](docs/faster_builds_with_linker_config.md)   
> [faster_builds_with_sccache.md](docs/faster_builds_with_sccache.md)  
> [how_to_debug_rust.md](docs/how_to_debug_rust.md)  

#### [examples/](examples)  
> [hello.rs](examples/hello.rs)  

#### [src/](src)  
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
> [iterators_aka_algorithms.rs](src/iterators_aka_algorithms.rs)  
> [let_else.rs](src/let_else.rs)  
> [match_.rs](src/match_.rs)  
> [never_type_never_returns.rs](src/never_type_never_returns.rs)  
> [newtype_pattern.rs](src/newtype_pattern.rs)
> [re_export.rs](src/re_export.rs)  
> [serde_json.rs](src/serde_json.rs)  
> [smart_ptrs.rs](src/smart_ptrs.rs)  
> [string_concat.rs](src/string_concat.rs)  
> [trait_when_to_use_dyn_dispatch.rs](src/trait_when_to_use_dyn_dispatch.rs)    
> [trait_simple_vs_enum.rs](src/trait_simple_vs_enum.rs)   
> [trait_object_vs_struct_obj.rs](src/trait_object_vs_struct_obj.rs)  
> [traits_associated_type_vs_generics.rs](src/traits_associated_type_vs_generics.rs)  
> [traits_default_type_parameter.rs](src/traits_default_type_parameter.rs)  
> [traits_same_name_fn_call.rs](src/traits_same_name_fn_call.rs)  
> [traits_supertraits.rs](src/traits_supertraits.rs)  
> [type_.rs](src/type_.rs)  
> [unit_integration_tests.rs](src/unit_integration_tests.rs)  
> [unit_tested_file.rs](src/unit_tested_file.rs)  
> [web-server-async-and-parallel](src/web-server-async-and-parallel)  
> [web-server-multi-threaded](src/web-server-multi-threaded)  


#### Sources:  
- [Rust Programming Language Book by Steve Klabnik](https://doc.rust-lang.org/book/)
- [Rust in Action – 2021 by Tim McNamara](https://www.amazon.ca/Rust-in-Action/dp/1617294551)
- [Effective Rust - 35 Specific Ways to Improve Your Rust Code by David Drysdale](https://www.lurklurk.org/effective-rust/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/about.html)
- Rust for Rustaceans Idiomatic Programming for Experienced Developers - 2021 - by Jon Gjengset


#### What's more in this repo?  
- Organization of main.rs, lib.rs and unit tests
- Working with command line arguments
- Working with environment variables
- Simple Cacher struct -> [src/cacher.rs](src/cacher.rs)
- Examples of using closures -> [src/closure.rs](src/closure.rs)
- Debugging, `.vscode/launch.json` template
- `.cargo/config.toml` sample, including fastest linkers, very&more useful for big codebases
- Read from file, find lines that contains a string
- And more... see -> [src/*.rs](src/)
