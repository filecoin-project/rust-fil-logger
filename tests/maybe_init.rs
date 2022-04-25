#[test]
fn maybe_init_always_possible() {
    // Make sure that the logger is initialized.
    fil_logger::init();

    // Subsequent calls will fail silently.
    fil_logger::maybe_init();
    fil_logger::maybe_init();
}
