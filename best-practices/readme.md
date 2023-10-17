- Use `usize` for return type of size of collections fn.
  > standard collections return their size as a usize (from .len()), so collection indexing means that usize values are quite common – which is obviously fine from a capacity perspective, as there can't be more items in an in-memory collection than there are memory addresses on the syste  
  > Src: https://www.lurklurk.org/effective-rust/use-types.html
