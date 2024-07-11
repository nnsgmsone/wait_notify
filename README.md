# wait_notify

`wait_notify` is a simple synchronization primitive for Rust that provides basic wait and notify functionality. It is similar to `WaitGroup`, but with only `wait` and `notify` functions.

## Features

- **wait**: Block the current thread until a notification is received.
- **notify**: Wake up all waiting thread.

## Getting Started

```rust
use wait_notify::WaitNotify;
use std::thread;

let wn = WaitNotify::default();
let wn_clone = wn.clone();
let handle = thread::spawn(move || {
    // do some work
    wn_clone.wait();
    // do some work
});
// do some work
wn.notify();
// do some work
handle.join().unwrap();

```

## License

`wait_notify` source code is available under the GPL [License](/LICENSE).
