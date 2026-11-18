use std::process::Command;

#[test]
fn maybe_init_always_possible() {
    // Set the environment before starting the isolated logger test process.
    if std::env::var_os("FIL_LOGGER_TEST_CHILD").is_none() {
        let status = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "maybe_init_always_possible", "--nocapture"])
            .env("FIL_LOGGER_TEST_CHILD", "1")
            .env("RUST_LOG", "warn")
            .status()
            .unwrap();
        assert!(status.success(), "logger subprocess failed: {}", status);
        return;
    }

    fil_logger::init();

    fil_logger::maybe_init();
    fil_logger::maybe_init();

    assert!(log::log_enabled!(log::Level::Warn));
    assert!(!log::log_enabled!(log::Level::Info));
}
