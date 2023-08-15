use log;

pub fn intialize_logger() {
    tui_logger::init_logger(log::LevelFilter::Trace).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Trace);

    log::info!("Hello from the logger");
}
