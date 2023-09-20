extern crate log;
use std::error::Error;

pub fn init() -> Result<(), Box<dyn Error>> {
    tui_logger::init_logger(log::LevelFilter::Trace).or(Err("Could not initalize logger"))?;
    tui_logger::set_default_level(log::LevelFilter::Trace);

    log::info!("Info Message");
    log::error!("Error Message");
    log::warn!("Warn Message");
    log::trace!("Trace Message");
    Ok(())
}
