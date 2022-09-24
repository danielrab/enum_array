A small crate adding a macro that creates a constant array containing all the items of the enum.
usage example:
```rust
enum_array!{
    #[derive(Debug, PartialEq, Eq)]
    pub enum Example {
        A,
        B,
    }
}
assert_eq!(Example::ENTRIES, [Example::A, Example::B])
```