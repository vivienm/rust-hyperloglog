# hyperloglog

A [hyperloglog](https://en.wikipedia.org/wiki/HyperLogLog) implementation in Rust.

```rust
use hyperloglog::HyperLogLog;

let mut hll = HyperLogLog::<i32>::new();
for i in 0..100_000 {
     hll.insert(&i);
}
assert!((50_000..150_000).contains(&hll.len()));
```

[API documentation](https://vivienm.github.io/rust-hyperloglog/hyperloglog/index.html)
