## [no_vec] crate

Rust crate for modifying sized arrays. It contains some useful methods:

### `[T; n]::stick(T) -> [T; n+1]`

Adds a new element to an array:

```rust
let arr: [u16; 2] = [123u16].stick(456);
assert_eq!(arr, [123, 456]);
```

### `[T; n+1]::unstick() -> ([T; n], T)`

Removes an element from an array:

```rust
let (arr, item): ([u16; 1], u16) = [123u16, 456].unstick();
assert_eq!(arr, [123]);
assert_eq!(item, 456);
```

### `Vec<T>::concrete() -> [T]`

Converts a vector to a sized array:

```rust
let arr: [u16; 2] = vec![123u16, 456].concrete();
assert_eq!(arr, [123, 456]);
```

### `[T]::melt() -> Vec<T>`

Convers a sized array to a vector:

```rust
let vec: Vec<u16> = [123u16, 456].melt();
assert_eq!(vec, vec![123, 456]);
```

