use std::sync::{Arc, RwLock};

use crate::error::event::Event;
use crate::error::severity::Severity;

pub type AsyncLoggerReference = Arc<RwLock<Logger>>;
pub type NoAux = ();
pub const NO_AUX: &NoAux = &();

pub trait Logger {
    fn log(&mut self, _: Severity, _: &str);
}

pub trait Log
{
    fn register_logger(&mut self, logger: AsyncLoggerReference);
    fn retrieve_logger(&self) -> Option<AsyncLoggerReference>;
    fn log(&self, sev: Severity, desc: &str) {
        if let Some(logger) = self.retrieve_logger() {
            let mut alr = logger.write().unwrap();

            alr.log(sev, desc);
        }
    }
}

impl Logger for Vec<Event> {
    fn log(&mut self, sev: Severity, desc: &str) {
        self.push(Event::new(sev, desc));
    }
}