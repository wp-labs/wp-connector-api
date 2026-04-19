use derive_more::From;
use orion_error::{ErrorCode, StructError, ToStructError, UvsReason};
use serde::Serialize;
use std::error::Error as StdError;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Serialize, From)]
pub enum SourceReason {
    #[error("not data")]
    NotData,
    #[error("eof")]
    EOF,
    #[error("supplier error : {0}")]
    SupplierError(String),
    #[from(skip)]
    #[error("disconnected: {0}")]
    Disconnect(String),
    #[from(skip)]
    #[error("{0}")]
    Other(String),
    #[error("{0}")]
    Uvs(UvsReason),
}

impl ErrorCode for SourceReason {
    fn error_code(&self) -> i32 {
        match self {
            // Informational: normal conditions
            SourceReason::NotData => 100, // Temporary no data available
            SourceReason::EOF => 101,     // End of data stream

            // Retryable errors
            SourceReason::Disconnect(_) => 503, // Connection lost, can retry

            // Internal/supplier errors
            SourceReason::SupplierError(_) => 500, // Upstream supplier error
            SourceReason::Other(_) => 520,         // Unclassified error

            // Delegate to wrapped reason
            SourceReason::Uvs(r) => r.error_code(),
        }
    }
}

pub type SourceError = StructError<SourceReason>;
pub type SourceResult<T> = Result<T, StructError<SourceReason>>;

impl SourceReason {
    pub fn err(self) -> SourceError {
        self.to_err()
    }

    pub fn err_detail<S: Into<String>>(self, detail: S) -> SourceError {
        self.to_err().with_detail(detail.into())
    }

    pub fn err_source<E>(self, source: E) -> SourceError
    where
        E: StdError + Send + Sync + 'static,
    {
        self.to_err().with_source(source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_reason_error_codes() {
        // Informational codes (1xx)
        assert_eq!(SourceReason::NotData.error_code(), 100);
        assert_eq!(SourceReason::EOF.error_code(), 101);

        // Retryable codes (5xx with specific meaning)
        assert_eq!(
            SourceReason::Disconnect("conn lost".into()).error_code(),
            503
        );

        // Internal errors (5xx)
        assert_eq!(
            SourceReason::SupplierError("upstream".into()).error_code(),
            500
        );
        assert_eq!(SourceReason::Other("misc".into()).error_code(), 520);
    }

    #[test]
    fn source_reason_error_codes_are_distinct() {
        let codes = vec![
            SourceReason::NotData.error_code(),
            SourceReason::EOF.error_code(),
            SourceReason::Disconnect("x".into()).error_code(),
            SourceReason::SupplierError("x".into()).error_code(),
            SourceReason::Other("x".into()).error_code(),
        ];
        // Verify all codes are different
        let mut unique = codes.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(codes.len(), unique.len(), "error codes should be distinct");
    }

    #[test]
    fn source_reason_informational_codes_are_below_200() {
        assert!(SourceReason::NotData.error_code() < 200);
        assert!(SourceReason::EOF.error_code() < 200);
    }

    #[test]
    fn source_reason_retryable_codes_are_5xx() {
        let code = SourceReason::Disconnect("x".into()).error_code();
        assert!((500..600).contains(&code));
    }
}
