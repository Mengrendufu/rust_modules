# rust_modules

Small reusable Rust modules for local embedded-style experiments.

## Modules

- `ring_buffer.rs`: fixed-capacity, no-heap ring buffer using `[Option<T>; N]` storage.

## RingBuffer

```rust
#[path = "../../rust_modules/ring_buffer.rs"]
mod ring_buffer;

use ring_buffer::RingBuffer;

let mut queue: RingBuffer<u8, 4> = RingBuffer::new();

let _ = queue.push(1);
let item = queue.pop();
```

## License

WTFPL v2. See `LICENSE`.
