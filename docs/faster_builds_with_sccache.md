## Faster clean builds with 'sccache' (3min instead of 7min)

see also: faster_builds_with_linker_config.md

Measure your build time first without 'sccache':  

```
// Clean build of `tower` takes around 7min
cargo clean 
cargo b -p tower 
```

Setup  
```
cargo install sccache
echo "export RUSTC_WRAPPER=sccache" >> ~/.bashrc
source ~/.bashrc

// macos 
echo -e "\nexport RUSTC_WRAPPER=sccache--" >> ~/.bash_profile
source ~/.bash_profile  
```

Use `sccache`  
```
// Ignore this build - caching here
cargo clean 
cargo b -p tower 

// Now, this takes `3min` instead of `7min` - Incredible!
cargo clean 
cargo b -p tower
````

Checking cache hits `sccache -s`:

```
Compile requests                    409
Compile requests executed           282
Cache hits                            0
Cache misses                        282
Cache misses (C/C++)                237
Cache misses (Rust)                  45
Cache timeouts                        0
Cache read errors                     0
Forced recaches                       0
Cache write errors                    0
Compilation failures                  0
Cache errors                          0
Non-cacheable compilations            0
Non-cacheable calls                 123
Non-compilation calls                 4
Unsupported compiler calls            0
Average cache write               0.001 s
Average cache read miss           2.905 s
Average cache read hit            0.000 s
Failed distributed compilations       0

Non-cacheable reasons:
incremental                          90
crate-type                           32
-                                     1

Cache location                  Local disk: "/Users/<user>/Library/Caches/Mozilla.sccache"
Cache size                          274 MiB
Max cache size                       10 GiB
```

