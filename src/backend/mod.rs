use thiserror::Error;

pub trait Backend: Sized {
    fn create() -> Result<Self, BackendError>;
    fn active_window_matches<F>(&mut self, attribute: WindowAttribute, predicate: F) -> bool
    where
        F: FnOnce(&str) -> bool;
    fn wait_for_active_window(&mut self);
}

#[derive(Error, Debug)]
pub enum BackendError {
    #[error("backend failed to initialize")]
    Initialize { source: Box<dyn std::error::Error> },
}

pub enum WindowAttribute {
    Name,
    Class,
}

pub mod x11;
