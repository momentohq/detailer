# detailer

A convenience tool for workflow logging.

# About
A trim, low-dependency tool for logging things. This project does not
use `unsafe` code. It only depends on std and log.

# Details
Detailer lets you log all your related information about a workflow in
one `log` report. Sometimes you want that. Sometimes you only want that
once in a while.

The overhead of a disabled Detailer log line is similar to the overhead
of a disabled log level, like a `log::debug!()` when the current level
is set to Info. An enabled `detail!()` statement costs a `writeln!()`
into a String. If you want to keep this inexpensive and you have a ton
of these detailers, you should consider using an explicit `flush()` and
reuse them. Remember to `reset()` when your time should restart, if you
are using timings.


```rust
use detailer::{Detailer, detail, new_detailer, scope};

let mut detailer = new_detailer!(); // Info level, WithTimings
detail!(detailer, "some {} message", "log");
```

You might consider taking a Detailer as a parameter for functions that should log detail when
the detailer is enabled. The functions do not need to inspect the Detailer: They only log their
details, similarly to how you use `log`.
```rust
fn expensive_work(detailer: &mut Detailer, name: &str) {
    let _guard = scope!(detailer, "expensive work: {name}"); // indent lines under the expensive_work stack
    detail!(detailer, "some part of the work");
    detail!(detailer, "some other part of the work");
}
```
