use regex::Regex;
use thiserror::Error;

pub trait Backend: Sized {
    fn create() -> Result<Self, BackendError>;
    fn active_window_matches(&mut self, regex: &Regex) -> bool;
    fn wait_for_active_window(&mut self);
}

#[derive(Error, Debug)]
pub enum BackendError {
    #[error("backend failed to initialize")]
    Initialize { source: Box<dyn std::error::Error> },
}

pub mod x11;
