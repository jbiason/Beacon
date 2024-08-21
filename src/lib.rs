//! A Semaphore-like structure.

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Condvar;
use std::sync::Mutex;

pub struct Beacon {
    lock: Mutex<AtomicUsize>,
    guard: Condvar,
}

impl Beacon {
    pub fn new(leases: usize) -> Self {
        Self {
            lock: Mutex::new(AtomicUsize::new(leases)),
            guard: Condvar::new(),
        }
    }

    pub fn acquire(&self, leases: usize) {
        let mut control = self.lock.lock().unwrap();
        let mut current_leases = control.load(Ordering::Relaxed);
        while current_leases < leases {
            control = self.guard.wait(control).unwrap();
            current_leases = control.load(Ordering::Relaxed);
        }

        control.fetch_sub(leases, Ordering::SeqCst);
    }

    pub fn release(&self, leases: usize) {
        let control = self.lock.lock().unwrap();
        control.fetch_add(leases, Ordering::SeqCst);
        self.guard.notify_all();
    }
}
