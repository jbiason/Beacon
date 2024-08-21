# Beacon

Beacon is a pure "std" semaphore in Rust, which allows requesting multiple
leases at the same time.

(Note that this doesn't use the system semaphore primitive, since it is not
available in std.)
