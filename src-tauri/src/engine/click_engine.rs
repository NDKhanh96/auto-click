use std::sync::atomic::AtomicBool;
use std::time::Duration;

pub struct ClickEngine {
    pub interval: Duration,
    pub running: AtomicBool,
}
