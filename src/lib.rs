use thiserror::Error;

pub mod scan;

#[derive(Debug, Error)]
pub enum ScanErrors {
    #[error("io error")]
    IOError(#[from] std::io::Error),
}

pub type ScanResult<T> = Result<T, ScanErrors>;
