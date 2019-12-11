# comlog

Serial port logger, which runs under `no_std` environment.

```rust
comlog::init_with_filter(LevelFilter::Info);
info!("Starting");
```

`comlog` relies on `alloc`; that is, an allocator needs to be implemented.
