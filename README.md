Helper functions for retrieving stdin, stdout, stderr to work with `libc`.

Get started
------------

Add this to your `Cargo.toml`:

```rust
[dependencies]
libc-stdhandle = "0.8.2"
```

And you can:
```rust
extern crate libc_stdhandle;
use libc_stdhandle::*;

unsafe { stdout() }
```
