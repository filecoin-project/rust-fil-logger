#[test]
fn maybe_init_always_possible() {
    std::env::set_var("RUST_LOG", "warn");
    fil_logger::init();

    fil_logger::maybe_init();
    fil_logger::maybe_init();

    assert!(log::log_enabled!(log::Level::Warn));
    assert!(!log::log_enabled!(log::Level::Info));
}
