# rust_modules

Small reusable Rust modules for local embedded-style experiments.

## Modules

- `src/ring_buffer.rs`: fixed-capacity, no-heap ring buffer using `[Option<T>; N]` storage.

## RingBuffer

```rust
use rust_modules::RingBuffer;

let mut queue: RingBuffer<u8, 4> = RingBuffer::new();

let _ = queue.push(1);
let item = queue.pop();
```

## License

WTFPL v2. See `LICENSE`.
