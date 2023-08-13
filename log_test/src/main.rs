fn main() {
    env_logger::init();
    log::info!("1");
    log::trace!("2");
    log::debug!("3");
    log::error!("4");
    log::warn!("5");
}
