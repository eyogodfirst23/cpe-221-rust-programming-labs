# Lab 6 — Concurrency & File I/O

- `exerciseA_spawn.rs` — Spawning and joining OS threads
- `exerciseB_shared_state.rs` — Shared state with Arc<Mutex<T>>, naive vs refactored locking with timing
- `exerciseC_channels.rs` — mpsc channels, worker threads, error handling for out-of-range sums
- `exerciseD_file_io.rs` — File I/O with BufReader/BufWriter, recursive .rs file listing
- `exerciseE_async_main.rs` — Async/await with Tokio, sequential vs concurrent vs `tokio::join!`
