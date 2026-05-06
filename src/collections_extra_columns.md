| Collection | Memory overhead | Data locality | Prod usage |
|---|---|---|---|
| `Vec<T>` | lowest; ptr+len+cap only, no per-elem overhead | best; fully contiguous, prefetcher loves it | ~40% |
| `VecDeque<T>` | same as Vec + head/tail indices | nearly as good; contiguous ring buffer, minor wrap-around penalty | ~7% |
| `HashMap<K,V>` | control byte per slot + empty slots at ~87% load factor | moderate; control bytes contiguous (SIMD probing), key+value slots scattered | ~30% |
| `HashSet<T>` | same as HashMap; value is `()` | same as HashMap | ~10% |
| `BTreeMap<K,V>` | per node: keys+values+child pointers+metadata; nodes ~512 bytes each | moderate; good within a node, pointer-chase between nodes | ~5% |
| `BTreeSet<T>` | same as BTreeMap; value is `()` | same as BTreeMap | ~3% |
| `BinaryHeap<T>` | same as Vec internally | good; contiguous like Vec but sift-down jumps parent/child indices | ~3% |
| `LinkedList<T>` | worst; 2 pointers (prev+next) per elem = 16 extra bytes on 64-bit + per-node heap alloc | worst; every next/prev is a pointer chase to random heap addr | ~2% |