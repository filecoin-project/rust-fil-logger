use std::fs::{self, File};

#[test]
fn file_logging_preserves_json_filtering_and_trace() {
    let path = std::env::temp_dir().join(format!("fil-logger-{}.log", std::process::id()));
    let file = File::create_new(&path).unwrap();
    std::env::set_var("RUST_LOG", "trace/keep");
    std::env::set_var("GOLOG_LOG_FMT", "json");
    fil_logger::init_with_file(file);

    log::info!("keep \"quoted\"\nline");
    log::warn!("drop this message");
    log::debug!("keep debug");
    fil_logger::maybe_init();
    log::trace!("keep trace");
    log::logger().flush();

    let output = fs::read_to_string(&path).unwrap();
    fs::remove_file(path).unwrap();
    let lines: Vec<_> = output.lines().collect();
    assert_eq!(lines.len(), 3);
    assert!(lines[0].starts_with("{\"level\":\"info\","));
    assert!(lines[0].ends_with(r#""msg":"keep \"quoted\"\nline"}"#));
    assert!(lines[1].starts_with("{\"level\":\"debug\","));
    assert!(lines[1].ends_with(r#""msg":"keep debug"}"#));
    assert!(lines[2].starts_with("{\"level\":\"trace\","));
    assert!(lines[2].ends_with(r#""msg":"keep trace"}"#));
}
