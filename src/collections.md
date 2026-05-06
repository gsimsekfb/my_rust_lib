| Collection | Lookup | Insert | Remove | Prod usage | Use cases |
|---|---|---|---|---|---|
| `Vec<T>` | **O(1)** - by index<br>O(n) - linear search | **O(1)\*** - push_back<br>O(n) - mid/front (shifts elems) | **O(1)** - pop_back<br>O(n) - mid/front (shifts elems) | ~40% | buffers, sequential storage, return values |
| `VecDeque<T>` | **O(1)** - by index | **O(1)\*** - push front/back<br>O(n) - mid | **O(1)\*** - pop front/back<br>O(n) - mid | ~7% | double-ended queue, sliding windows, BFS |
| `HashMap<K,V>` | **O(1)\*** - get<br>O(n) - worst case collision (rare)<br> | **O(1)\*** - insert | **O(1)\*** - remove | ~30% | unordered key-value w/ unique keys <br> **Note: HashMap/HashSet iteration is O(capacity)** - walks empty buckets|
| `HashSet<T>` | **O(1)\*** - contains | **O(1)\*** - insert | **O(1)\*** - remove | ~10% | unordered unique elements |
| `BTreeMap<K,V>` | **O(log n)** - get (binary search)<br>**O(log n + k)** - range query `for (k, v) in map.range(3..=7)` | **O(log n)** - insert (binary search to find place to insert) | **O(log n)** - remove (binary search) | ~5% | sorted key-value, range queries |
| `BTreeSet<T>` | **O(log n)** - contains | **O(log n)** - insert | **O(log n)** - remove | ~3% | sorted unique elements |
| `BinaryHeap<T>` | **O(1)** - peek max (always at root)<br>O(n) - arbitrary search | **O(log n)** - push | **O(log n)** - pop max (sift-down)<br>O(n) - arbitrary remove (rebuild) | ~3% | where only min or max matters - priority queues, task scheduling |
| `LinkedList<T>` | O(n) - walk node by node from head | O(1) - push front/back<br>O(n) - mid (find position first) | O(1) - pop front/back<br>O(n) - mid | ~2% | rarely justified; cursor-based edits |

\* amortized average case

