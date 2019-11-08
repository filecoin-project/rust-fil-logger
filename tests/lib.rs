use fil_logger;

#[test]
#[should_panic(expected = "Initializing logger failed. Was another logger already initialized?")]
fn double_init_failure() {
    fil_logger::init();
    fil_logger::init();
}
