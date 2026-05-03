use orion_error::conversion::ToStructError;
use orion_error::{OrionError, StructError, UnifiedReason};
use serde::Serialize;
use std::error::Error as StdError;

#[derive(Debug, Clone, PartialEq, Serialize, OrionError)]
pub enum SourceReason {
    #[orion_error(identity = "biz.not_data", message = "not data")]
    NotData,
    #[orion_error(identity = "biz.eof", message = "eof")]
    EOF,
    #[orion_error(identity = "biz.supplier_error", message = "supplier error")]
    SupplierError,
    #[orion_error(identity = "biz.disconnect", message = "disconnect")]
    Disconnect,
    #[orion_error(identity = "biz.other", message = "other source error")]
    Other,
    #[orion_error(transparent)]
    Uvs(UnifiedReason),
}

pub type SourceError = StructError<SourceReason>;
pub type SourceResult<T> = Result<T, StructError<SourceReason>>;

impl SourceReason {
    pub fn supplier_error<S: Into<String>>(detail: S) -> SourceError {
        SourceReason::SupplierError.err_detail(detail)
    }

    pub fn disconnect<S: Into<String>>(detail: S) -> SourceError {
        SourceReason::Disconnect.err_detail(detail)
    }

    pub fn other<S: Into<String>>(detail: S) -> SourceError {
        SourceReason::Other.err_detail(detail)
    }

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
    fn source_reason_err_detail_sets_detail() {
        let err = SourceReason::other("boom");
        assert_eq!(err.detail().as_deref(), Some("boom"));

        let err = SourceReason::Other.err_detail("ctx");
        assert_eq!(err.detail().as_deref(), Some("ctx"));
    }

    #[test]
    fn source_reason_err_source_preserves_source_message() {
        let err = SourceReason::Disconnect.err_source(std::io::Error::other("disk gone"));
        // StructError no longer impls Error in 0.8; inspect source via as_std()
        let as_std = err.as_std();
        let src = as_std.source().expect("source should be present");
        assert!(src.to_string().contains("disk gone"));
    }
}
