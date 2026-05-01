Which iterator methods can we early exit ?

We can generalize the **behavior of the closure**, but not the **behavior of the method** calling it.

The generalization is: **A `return` inside any closure only exits that specific execution of the closure.** It never exits the "parent" loop or method (like `retain`, `map`, or `filter`).

However, whether the parent method "stops" depends on its specific logic:

### 1. The "Exhaustive" Methods (Like `retain`)
These methods are designed to process the **entire** collection. They ignore the fact that you might be "done" because they have a mechanical responsibility to maintain the data structure's integrity (e.g., rebalancing the tree or shifting array elements).
*   **Examples:** `retain`, `map`, `for_each`, `filter`, `fold`.
*   **Result:** Even if you `return` early from the closure, the method keeps "calling the closure for every remaining element".

### 2. The "Short-Circuiting" Methods
These methods are designed to stop as soon as a specific condition is met. They "listen" to the `return` value of the closure and decide whether to break the loop internally.
*   **Examples:** `any`, `all`, `find`, `position`.
*   **Result:** A `return true` in `any()` will physically stop the iteration immediately.

### 3. The "Lazy" Iterators
These don't "run" at all until you ask for a result. You can force them to stop early by using adapters.
*   **Example:** `state.bids.iter().filter(|...|).take_while(|...|).collect()`
*   **Result:** `.take_while()` will physically stop the iteration the moment its condition returns `false`.

