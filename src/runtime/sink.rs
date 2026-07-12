use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use wp_model_core::model::DataRecord;

use crate::{SinkDefProvider, SinkResult};

// Reuse workspace error type to avoid duplicating an error abstraction

// ---------- Batch Metadata ----------

/// Batch-level runtime metadata passed from engine to sink.
///
/// `BatchMeta` carries information that belongs to the batch/frame level
/// (not to individual records), such as the OML output model name and output
/// metadata field policy.
/// Sinks decide how to consume it — e.g. `TcpArrowSink` uses `oml_name`
/// as the Arrow frame tag in `arrow_framed` mode.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BatchMeta {
    /// OML output model name — identifies the logical output stream.
    pub oml_name: Option<String>,
    /// Metadata payload fields disabled by the engine/group configuration.
    pub output_disabled: Vec<String>,
}

impl BatchMeta {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_oml_name(name: impl Into<String>) -> Self {
        Self {
            oml_name: Some(name.into()),
            output_disabled: Vec::new(),
        }
    }

    pub fn oml_name(&self) -> Option<&str> {
        self.oml_name.as_deref()
    }

    pub fn with_output_disabled(fields: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            oml_name: None,
            output_disabled: fields.into_iter().map(Into::into).collect(),
        }
    }

    pub fn set_output_disabled(&mut self, fields: impl IntoIterator<Item = impl Into<String>>) {
        self.output_disabled = fields.into_iter().map(Into::into).collect();
    }

    pub fn output_disabled(&self) -> &[String] {
        &self.output_disabled
    }

    pub fn is_output_disabled(&self, field: &str) -> bool {
        self.output_disabled.iter().any(|item| item == field)
    }

    pub fn is_empty(&self) -> bool {
        self.oml_name.as_deref().is_none_or(str::is_empty) && self.output_disabled.is_empty()
    }
}

// ---------- Core Sink Traits ----------

/// Runtime control trait for managing sink lifecycle.
///
/// Implementors must ensure all methods are **idempotent** - calling them
/// multiple times should be safe and produce consistent results.
///
/// # Example
/// ```ignore
/// #[async_trait]
/// impl AsyncCtrl for MySink {
///     async fn stop(&mut self) -> SinkResult<()> {
///         self.connection.close().await?;
///         Ok(())
///     }
///     async fn reconnect(&mut self) -> SinkResult<()> {
///         self.connection = Connection::new(&self.config).await?;
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait AsyncCtrl {
    /// Gracefully stop the sink and release all resources.
    ///
    /// This method must be idempotent - subsequent calls should return `Ok(())`
    /// without side effects. After calling `stop()`, the sink should not accept
    /// any more data.
    async fn stop(&mut self) -> SinkResult<()>;

    /// Reconnect or refresh the sink's underlying connection.
    ///
    /// Use this to recover from transient failures without recreating the sink.
    /// The method should preserve any configuration and state that doesn't
    /// depend on the connection itself.
    async fn reconnect(&mut self) -> SinkResult<()>;
}

/// Trait for sinking structured records.
///
/// Provides methods for writing parsed, typed data records to a destination.
/// Implementations should handle batching efficiently when `sink_records` is called.
#[async_trait]
pub trait AsyncRecordSink {
    /// Write a single data record to the sink.
    ///
    /// # Arguments
    /// * `data` - Reference to the record to write
    async fn sink_record(&mut self, data: &DataRecord) -> SinkResult<()>;

    /// Write multiple data records in batch.
    ///
    /// Implementations should optimize for batch writes when possible.
    /// The order of records in the vector should be preserved.
    ///
    /// # Arguments
    /// * `data` - Vector of records wrapped in Arc for shared ownership
    async fn sink_records(&mut self, data: Vec<Arc<DataRecord>>) -> SinkResult<()>;

    /// Write multiple data records with batch-level metadata.
    ///
    /// Default implementation ignores [`BatchMeta`] and falls back to
    /// [`sink_records`](Self::sink_records), so existing connectors compile
    /// unchanged. Sinks that need the metadata should override this method.
    ///
    /// # Implementation Requirements
    ///
    /// Sink implementations SHOULD consume `meta.oml_name` according to their
    /// output format:
    ///
    /// | Sink 类型 | `oml_name` 用途 |
    /// |-----------|------------------|
    /// | **Arrow** (`arrow_framed`) | 写入 frame header `tag`（`[tag_len][tag][IPC]`） |
    /// | **Arrow** (`arrow_ipc`) | 忽略（裸 IPC Stream 无 tag 位置） |
    /// | **JSON / CSV**（文本格式） | 作为 `wp_oml_name` 字段追加到每条记录中 |
    ///
    /// 当 `meta.oml_name` 为空时，所有 sink 保持原有行为（Arrow 使用 connector
    /// 配置的固定 `tag`，文本格式不追加字段）。
    ///
    /// # Arguments
    /// * `meta` - Batch-level runtime metadata (e.g. OML output name)
    /// * `data` - Vector of records wrapped in Arc for shared ownership
    async fn sink_records_with_meta(
        &mut self,
        meta: BatchMeta,
        data: Vec<Arc<DataRecord>>,
    ) -> SinkResult<()> {
        let _ = meta;
        self.sink_records(data).await
    }
}

/// Trait for sinking raw data (strings and bytes).
///
/// Provides methods for writing unstructured data to a destination.
/// Useful for pass-through scenarios or when data doesn't need parsing.
#[async_trait]
pub trait AsyncRawDataSink {
    /// Write a single string payload.
    async fn sink_str(&mut self, data: &str) -> SinkResult<()>;

    /// Write a single byte payload.
    async fn sink_bytes(&mut self, data: &[u8]) -> SinkResult<()>;

    /// Write multiple string payloads in batch.
    ///
    /// Order of strings should be preserved in the output.
    async fn sink_str_batch(&mut self, data: Vec<&str>) -> SinkResult<()>;

    /// Write multiple byte payloads in batch.
    ///
    /// Order of byte slices should be preserved in the output.
    async fn sink_bytes_batch(&mut self, data: Vec<&[u8]>) -> SinkResult<()>;
}

/// Combined trait for full-featured async sinks.
///
/// This is a marker trait that combines [`AsyncRecordSink`], [`AsyncRawDataSink`],
/// and [`AsyncCtrl`]. Types implementing all three traits automatically implement
/// `AsyncSink` through the blanket implementation.
///
/// Use this trait when you need a sink that supports all data formats and
/// lifecycle management.
pub trait AsyncSink: AsyncRecordSink + AsyncRawDataSink + AsyncCtrl + Send + Sync {}

impl<T> AsyncSink for T where T: AsyncRecordSink + AsyncRawDataSink + AsyncCtrl + Send + Sync {}

// ---------- Build Ctx ----------

/// Build context passed to sink factories during construction.
///
/// Contains runtime configuration such as work directories, replica info,
/// and rate limiting hints that sinks may use during initialization.
#[derive(Clone, Debug)]
pub struct SinkBuildCtx {
    /// Root directory for sink-specific working files (state, checkpoints, etc.)
    pub work_root: PathBuf,
    /// Replica index for parallel group builds (0-based). Defaults to 0.
    pub replica_idx: usize,
    /// Replica count for the group (>=1). Defaults to 1.
    pub replica_cnt: usize,
    /// Upstream rate limit hint in requests per second. 0 means unlimited.
    pub rate_limit_rps: usize,
}

impl SinkBuildCtx {
    pub fn new(work_root: PathBuf) -> Self {
        Self {
            work_root,
            replica_idx: 0,
            replica_cnt: 1,
            rate_limit_rps: 0,
        }
    }
    pub fn new_with_replica(work_root: PathBuf, replica_idx: usize, replica_cnt: usize) -> Self {
        Self {
            work_root,
            replica_idx,
            replica_cnt: replica_cnt.max(1),
            rate_limit_rps: 0,
        }
    }
    pub fn with_limit(mut self, rate_limit_rps: usize) -> Self {
        self.rate_limit_rps = rate_limit_rps;
        self
    }
}

/// Handle wrapping a boxed async sink instance.
///
/// Returned by [`SinkFactory::build`] and used by the orchestrator to
/// manage sink lifecycle and data flow.
pub struct SinkHandle {
    /// The boxed sink implementing [`AsyncSink`]
    pub sink: Box<dyn AsyncSink + 'static>,
}

impl std::fmt::Debug for SinkHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Name matches the type to avoid confusion in logs/diagnostics
        f.debug_struct("SinkHandle")
            .field("sink", &"Box<dyn AsyncSink>")
            .finish()
    }
}

impl SinkHandle {
    pub fn new(sink: Box<dyn AsyncSink + 'static>) -> Self {
        Self { sink }
    }
}

// ---------- Resolved Route Spec + Factory (for runtime decoupling) ----------

/// Resolved sink specification with all parameters flattened.
///
/// Contains the fully resolved configuration for a sink instance,
/// with all inheritance and defaults already applied.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ResolvedSinkSpec {
    /// Sink group name for routing and management
    pub group: String,
    /// Unique sink instance name within the group
    pub name: String,
    /// Sink type identifier (e.g., "kafka", "elasticsearch")
    pub kind: String,
    /// Reference to the connector definition
    pub connector_id: String,
    /// Flattened runtime parameters
    pub params: crate::types::ParamMap,
    /// Optional filter expression for selective routing
    pub filter: Option<String>,
}

/// Factory trait for creating sink instances.
///
/// Implementors must also implement [`SinkDefProvider`] to provide
/// connector metadata. The orchestrator uses this trait to construct
/// sinks at runtime based on resolved specifications.
///
/// # Example
/// ```ignore
/// #[async_trait]
/// impl SinkFactory for KafkaSinkFactory {
///     fn kind(&self) -> &'static str { "kafka" }
///
///     async fn build(&self, spec: &ResolvedSinkSpec, ctx: &SinkBuildCtx) -> SinkResult<SinkHandle> {
///         let sink = KafkaSink::new(&spec.params).await?;
///         Ok(SinkHandle::new(Box::new(sink)))
///     }
/// }
/// ```
#[async_trait]
pub trait SinkFactory: SinkDefProvider + Send + Sync + 'static {
    /// Returns the unique type identifier for this sink factory.
    fn kind(&self) -> &'static str;

    /// Optional lightweight validation of the sink specification.
    ///
    /// Called before `build()` to catch configuration errors early.
    /// Default implementation accepts all specifications.
    fn validate_spec(&self, _spec: &ResolvedSinkSpec) -> SinkResult<()> {
        Ok(())
    }

    /// Construct a new sink instance from the given specification.
    ///
    /// # Arguments
    /// * `spec` - Resolved sink specification with parameters
    /// * `ctx` - Build context with runtime configuration
    ///
    /// # Returns
    /// A [`SinkHandle`] wrapping the constructed sink, or an error.
    async fn build(&self, spec: &ResolvedSinkSpec, ctx: &SinkBuildCtx) -> SinkResult<SinkHandle>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::{path::PathBuf, sync::Arc};
    use wp_model_core::model::DataRecord;

    #[derive(Default)]
    struct NoopSink;

    #[async_trait]
    impl AsyncCtrl for NoopSink {
        async fn stop(&mut self) -> SinkResult<()> {
            Ok(())
        }

        async fn reconnect(&mut self) -> SinkResult<()> {
            Ok(())
        }
    }

    #[async_trait]
    impl AsyncRecordSink for NoopSink {
        async fn sink_record(&mut self, _data: &DataRecord) -> SinkResult<()> {
            Ok(())
        }

        async fn sink_records(&mut self, _data: Vec<Arc<DataRecord>>) -> SinkResult<()> {
            Ok(())
        }
    }

    #[async_trait]
    impl AsyncRawDataSink for NoopSink {
        async fn sink_str(&mut self, _data: &str) -> SinkResult<()> {
            Ok(())
        }

        async fn sink_bytes(&mut self, _data: &[u8]) -> SinkResult<()> {
            Ok(())
        }

        async fn sink_str_batch(&mut self, _data: Vec<&str>) -> SinkResult<()> {
            Ok(())
        }

        async fn sink_bytes_batch(&mut self, _data: Vec<&[u8]>) -> SinkResult<()> {
            Ok(())
        }
    }

    #[test]
    fn sink_build_ctx_defaults_and_limits() {
        let ctx = SinkBuildCtx::new(PathBuf::from("/tmp/work"));
        assert_eq!(ctx.work_root, PathBuf::from("/tmp/work"));
        assert_eq!(ctx.replica_idx, 0);
        assert_eq!(ctx.replica_cnt, 1);
        assert_eq!(ctx.rate_limit_rps, 0);

        let replica_ctx = SinkBuildCtx::new_with_replica(PathBuf::from("/tmp/work"), 2, 0);
        assert_eq!(replica_ctx.replica_idx, 2);
        assert_eq!(
            replica_ctx.replica_cnt, 1,
            "replica count should clamp to >=1"
        );

        let limited = SinkBuildCtx::new(PathBuf::from("/tmp/work")).with_limit(250);
        assert_eq!(limited.rate_limit_rps, 250);
        assert_eq!(limited.replica_cnt, 1);
    }

    #[test]
    fn batch_meta_tracks_oml_name_and_output_disabled_fields() {
        let mut meta = BatchMeta::with_oml_name("nginx_access");
        assert_eq!(meta.oml_name(), Some("nginx_access"));
        assert!(meta.output_disabled().is_empty());
        assert!(!meta.is_empty());

        meta.set_output_disabled(["wp_oml_name", "wp_event_id"]);
        assert_eq!(meta.output_disabled(), &["wp_oml_name", "wp_event_id"]);
        assert!(meta.is_output_disabled("wp_oml_name"));
        assert!(!meta.is_output_disabled("wp_stream_tag"));

        let disabled_only = BatchMeta::with_output_disabled(["wp_oml_name"]);
        assert_eq!(disabled_only.oml_name(), None);
        assert!(!disabled_only.is_empty());
    }

    #[test]
    fn sink_handle_wraps_async_sink() {
        let handle = SinkHandle::new(Box::new(NoopSink));
        assert!(format!("{handle:?}").contains("SinkHandle"));
    }
}
