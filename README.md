## Introduction

The codes show numerous ways to modify a list in Rust, to see what the borrow checker is about.

## Basic

```console
cd test_mini
rustc test_ownership.rs && ./test_ownership
```

1. Mutable Borrow - Safe lock: You open, modify, and close it.
2. Immutable Borrow - Hotel room: You can look but not change anything.
3. Ownership Transfer - Moving van: You give away the whole thing.

## Advanced

```console
cd test_full
cargo run
```

1. Mutable Borrow - Safe lock: You open, modify, and close it.
2. Immutable Borrow - Hotel room: You can look but not change anything.
3. Ownership Transfer - Moving van: You give away the whole thing.
4. Box - Storage unit: You keep ownership but store it elsewhere.
5. Rc - Shared Library: Multiple users can read it but not modify it.
6. Arc - Secure Webpage: Many threads access read-only content.
7. RefCell - Restroom: Multiple access, but only one can modify at a time.
8. Mutex - Safe Lock: One at a time access in multi-threaded scenarios.
9. Async - Server request queue: Processing data asynchronously.
10. Async Lifetime - Managing a connection that outlives a function.
11. Gc - Smart Trash Bin: It will be cleaned when unused.

 
