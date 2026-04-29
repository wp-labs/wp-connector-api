use derive_more::From;
use orion_error::conversion::ToStructError;
use orion_error::{OrionError, StructError, UvsReason};
use serde::Serialize;
use std::error::Error as StdError;
use std::sync::mpsc::SendError;

#[derive(Debug, PartialEq, Serialize, From, OrionError)]
pub enum SinkReason {
    #[orion_error(identity = "biz.sink")]
    Sink(String),
    #[orion_error(identity = "biz.sink_mock", message = "set mock error")]
    Mock,
    #[orion_error(identity = "biz.sink_stg_ctrl", message = "stg ctrl error")]
    StgCtrl,
    #[orion_error(transparent)]
    Uvs(UvsReason),
}

pub type SinkError = StructError<SinkReason>;

pub trait ReasonSummary {
    fn summary(&self) -> String;
}

impl<T> From<SendError<T>> for SinkReason
where
    T: ReasonSummary,
{
    fn from(err: SendError<T>) -> Self {
        SinkReason::Sink(format!("send error: {}", err.0.summary()))
    }
}

impl From<anyhow::Error> for SinkReason {
    fn from(e: anyhow::Error) -> Self {
        SinkReason::Sink(format!("{}", e))
    }
}

pub type SinkResult<T> = Result<T, SinkError>;

impl SinkReason {
    pub fn sink<S: Into<String>>(msg: S) -> Self {
        SinkReason::Sink(msg.into())
    }

    pub fn err(self) -> SinkError {
        self.to_err()
    }

    pub fn err_detail<S: Into<String>>(self, detail: S) -> SinkError {
        self.to_err().with_detail(detail.into())
    }

    pub fn err_source<E>(self, source: E) -> SinkError
    where
        E: StdError + Send + Sync + 'static,
    {
        self.to_err().with_source(source)
    }
}

pub trait SinkErrorOwe<T> {
    fn owe_sink<S: Into<String>>(self, msg: S) -> Result<T, StructError<SinkReason>>;
}

impl<T, E> SinkErrorOwe<T> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn owe_sink<S: Into<String>>(self, msg: S) -> Result<T, StructError<SinkReason>> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                Err(StructError::from(SinkReason::Sink(msg.into())).with_detail(e.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    #[derive(Clone)]
    struct Summary(&'static str);

    impl ReasonSummary for Summary {
        fn summary(&self) -> String {
            self.0.into()
        }
    }

    #[test]
    fn sink_reason_from_send_error_uses_inner_summary() {
        let (tx, rx) = mpsc::channel();
        drop(rx);
        let err = tx.send(Summary("queue overflow")).unwrap_err();
        let reason = SinkReason::from(err);
        match reason {
            SinkReason::Sink(msg) => assert!(msg.contains("queue overflow")),
            other => panic!("unexpected reason: {other:?}"),
        }
    }

    #[test]
    fn sink_error_owe_wraps_displayable_error() {
        let failing: Result<(), &str> = Err("io timeout");
        let err = failing.owe_sink("flush failed").unwrap_err();
        match err.reason() {
            SinkReason::Sink(msg) => assert_eq!(msg, "flush failed"),
            other => panic!("unexpected reason: {other:?}"),
        }
        let detail = err.detail();
        assert_eq!(detail.as_ref().map(|s| s.as_str()), Some("io timeout"));
    }

    #[test]
    fn sink_reason_err_detail_sets_detail() {
        let err = SinkReason::sink("flush failed").err_detail("io timeout");
        assert_eq!(err.detail().as_deref(), Some("io timeout"));
    }

    #[test]
    fn sink_reason_err_source_preserves_source_message() {
        let err = SinkReason::sink("udp send failed").err_source(std::io::Error::other("no route"));
        assert!(err.to_string().contains("no route"));
    }
}
