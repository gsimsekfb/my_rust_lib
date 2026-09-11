# Concurrency
- **Data race**: root cause — unsynchronized concurrent access to shared memory with ≥1 write; result — corrupted/incorrect data (UB); fix — wrap in `Mutex<T>`/atomics.
- **Race condition**: root cause — individual ops synchronized but the overall sequence/logic isn't; result — incorrect state or panic (e.g. double-delete); fix — lock the entire critical section, not just each line.
- **Deadlock**: root cause — mutexes acquired in inconsistent order across threads; result — threads permanently blocked; fix — enforce a global lock ordering.
- **Livelock**: root cause — threads react to each other and keep changing state to avoid conflict; result — no blocking, but also no progress; fix — add randomized backoff or a tie-breaker priority.
