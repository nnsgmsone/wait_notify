//! rust_waitnotify
use std::sync::{Arc, Condvar, Mutex};

/// A WaitNotify waits for a notification to be sent.
///
/// # Examples
/// ```
/// use wait_notify::WaitNotify;
/// use std::thread;
///
/// let wn = WaitNotify::default();
/// let wn_clone = wn.clone();
/// let handle = thread::spawn(move || {
///    // do some work
///    wn_clone.wait();
///    // do some work
///    });
/// // do some work
/// wn.notify();
/// // do some work
/// handle.join().unwrap();
/// ```
#[derive(Clone)]
pub struct WaitNotify {
    counter_cond: Arc<(Mutex<u64>, Condvar)>,
}

impl WaitNotify {
    pub fn new() -> Self {
        WaitNotify {
            counter_cond: Arc::new((Mutex::new(0), Condvar::new())),
        }
    }
    /// notify all thread waiting on this WaitNotify.
    pub fn notify(&self) {
        let (lock, cvar) = &*self.counter_cond;
        let mut count = lock.lock().unwrap();
        if *count == 0 {
            *count = 1;
            cvar.notify_all();
        }
    }
    /// wait for notification.
    pub fn wait(&self) {
        let (lock, cvar) = &*self.counter_cond;
        let mut count = lock.lock().unwrap();
        while *count == 0 {
            count = cvar.wait(count).unwrap();
        }
        *count = 0;
    }
}

impl Default for WaitNotify {
    fn default() -> Self {
        WaitNotify::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn it_works() {
        let wn = WaitNotify::default();
        let wn_clone = wn.clone();
        let handle = thread::spawn(move || {
            wn_clone.wait();
        });
        wn.notify();
        handle.join().unwrap();
    }
}
