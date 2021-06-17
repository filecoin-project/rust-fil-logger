/*!
Using `fil_logger`.

By default the `fil_logger` doesn't log anything. You can change this by setting the `RUST_LOG`
environment variable to another level. This will show log output on stderr. Example:

```console
$ RUST_LOG=info cargo run --example simple
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/examples/simple`
2019-11-11T20:26:09.448 INFO simple > logging on into level
2019-11-11T20:26:09.448 WARN simple > logging on warn level
2019-11-11T20:26:09.448 ERROR simple > logging on error level
```

It is also possible to ouput the log as JSON. Simply set the `GOLOG_LOG_FMT` environment variable
to `json`. It is a bit more verbose and also contains the line file and line number of the log
call:

```console
$ GOLOG_LOG_FMT=json RUST_LOG=info cargo run --example simple
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/examples/simple`
{"level":"info","ts":"2021-06-17T10:17:57.605+0200","logger":"simple","caller":"examples/simple.rs:38","msg":"logging on into level"}
{"level":"warn","ts":"2021-06-17T10:17:57.605+0200","logger":"simple","caller":"examples/simple.rs:39","msg":"logging on warn level"}
{"level":"error","ts":"2021-06-17T10:17:57.605+0200","logger":"simple","caller":"examples/simple.rs:40","msg":"logging on error level"}
{"level":"info","ts":"2021-06-17T10:17:57.605+0200","logger":"simple","caller":"examples/simple.rs:42","msg":"logging string with json: {\"hello\": true}"}
```
*/

use fil_logger;
use log::{debug, error, info, trace, warn};

fn main() {
    fil_logger::init();

    trace!("logging on trace level");
    debug!("logging on debug level");
    info!("logging on into level");
    warn!("logging on warn level");
    error!("logging on error level");

    info!(r#"logging string with json: {{"hello": true}}"#);
}
