use thiserror::Error;

#[derive(Debug, Error)]
pub enum FundingError {
    #[error("oracle error: {0}")]
    Oracle(String),
}

