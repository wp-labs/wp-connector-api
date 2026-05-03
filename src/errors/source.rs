use derive_more::From;
use orion_error::conversion::ToStructError;
use orion_error::{OrionError, StructError, UnifiedReason};
use serde::Serialize;
use std::error::Error as StdError;

#[derive(Debug, Clone, PartialEq, Serialize, From, OrionError)]
pub enum SourceReason {
    #[orion_error(identity = "biz.not_data", message = "not data")]
    NotData,
    #[orion_error(identity = "biz.eof", message = "eof")]
    EOF,
    #[orion_error(identity = "biz.supplier_error")]
    SupplierError(String),
    #[orion_error(identity = "biz.disconnect")]
    #[from(skip)]
    Disconnect(String),
    #[orion_error(identity = "biz.other")]
    #[from(skip)]
    Other(String),
    #[orion_error(transparent)]
    Uvs(UnifiedReason),
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
    fn source_reason_err_detail_sets_detail() {
        let err = SourceReason::Other("boom".into()).err_detail("ctx");
        assert_eq!(err.detail().as_deref(), Some("ctx"));
    }

    #[test]
    fn source_reason_err_source_preserves_source_message() {
        let err = SourceReason::Disconnect("read failed".into())
            .err_source(std::io::Error::other("disk gone"));
        // StructError no longer impls Error in 0.8; inspect source via as_std()
        let as_std = err.as_std();
        let src = as_std.source().expect("source should be present");
        assert!(src.to_string().contains("disk gone"));
    }
}
