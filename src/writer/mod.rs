use async_trait::async_trait;

use crate::{
    errors::Result,
    utils::datafusion_ext::{QueryError, QueryResult, QueryResultStream},
};

pub mod postgres;

/// Defines how data should be written to the destination.
///
/// # Variants
///
/// * `Merge` - Upsert data based on primary key (insert new, update existing)
/// * `Append` - Always insert new rows without checking for duplicates
///
/// # Example
///
/// ```
/// use apitap::writer::WriteMode;
///
/// // For incremental updates with deduplication
/// let mode = WriteMode::Merge;
/// assert_eq!(mode, WriteMode::Merge);
///
/// // For append-only logs or events
/// let mode = WriteMode::Append;
/// assert_eq!(mode, WriteMode::Append);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum WriteMode {
    /// Upsert mode: Insert new records, update existing ones based on primary key
    Merge,
    /// Append mode: Always insert new records without checking for duplicates
    Append,
}

/// Trait defining the interface for writing query results to various destinations.
///
/// Implementations of this trait handle the specifics of writing data to different
/// storage systems (PostgreSQL, ClickHouse, BigQuery, etc.).
///
/// # Key Methods
///
/// * `write()` - Write in-memory query results
/// * `write_stream()` - Write streaming query results (more memory efficient)
/// * `merge()` - Perform upsert operations
/// * `on_error()` - Handle query errors
///
/// # Lifecycle Hooks
///
/// * `begin()` - Called before writing starts
/// * `commit()` - Called after successful write
/// * `rollback()` - Called if write fails
///
/// # Example Implementation
///
/// ```no_run
/// use async_trait::async_trait;
/// use apitap::writer::{DataWriter, WriteMode};
/// use apitap::utils::datafusion_ext::{QueryResult, QueryResultStream, QueryError};
/// use apitap::errors::Result;
///
/// struct MyCustomWriter {
///     connection_string: String,
/// }
///
/// #[async_trait]
/// impl DataWriter for MyCustomWriter {
///     async fn write(&self, result: QueryResult) -> Result<()> {
///         // Write in-memory results to your destination
///         println!("Writing {} rows to {}", result.row_count, result.table_name);
///         Ok(())
///     }
///
///     async fn write_stream(&self, result: QueryResultStream, mode: WriteMode) -> Result<()> {
///         // Stream results to destination
///         match mode {
///             WriteMode::Merge => println!("Merging data..."),
///             WriteMode::Append => println!("Appending data..."),
///         }
///         Ok(())
///     }
///
///     async fn on_error(&self, error: QueryError) -> Result<()> {
///         eprintln!("Error in {}: {}", error.table_name, error.error);
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait DataWriter: Send + Sync {
    /// Writes query results to the destination (in-memory mode).
    ///
    /// Receives the complete query result in memory and writes it to the destination.
    /// Use this for smaller datasets or when you need the full result set.
    ///
    /// # Arguments
    ///
    /// * `result` - Complete query result with table name, data, and row count
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Data written successfully
    /// * `Err(ApitapError)` - Write failed
    async fn write(&self, result: QueryResult) -> Result<()>;

    /// Writes query results to the destination (streaming mode).
    ///
    /// Processes results as a stream, which is more memory-efficient for large datasets.
    /// Default implementation does nothing - override to support streaming.
    ///
    /// # Arguments
    ///
    /// * `result` - Streaming query result
    /// * `write_mode` - How to write (Merge or Append)
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Data written successfully
    /// * `Err(ApitapError)` - Write failed
    async fn write_stream(&self, _result: QueryResultStream, _write_mode: WriteMode) -> Result<()> {
        Ok(())
    }

    /// Performs a merge/upsert operation on streaming results.
    ///
    /// Inserts new records and updates existing ones based on primary key.
    /// Default implementation does nothing - override to support merge operations.
    ///
    /// # Arguments
    ///
    /// * `result` - Streaming query result to merge
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Merge completed successfully
    /// * `Err(ApitapError)` - Merge failed
    async fn merge(&self, _result: QueryResultStream) -> Result<()> {
        Ok(())
    }

    /// Handles query execution errors.
    ///
    /// Called when a query fails during execution. Default implementation logs
    /// the error but doesn't fail the pipeline.
    ///
    /// # Arguments
    ///
    /// * `error` - Error details including table name and error message
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Error handled successfully
    /// * `Err(ApitapError)` - Error handling failed
    async fn on_error(&self, error: QueryError) -> Result<()> {
        tracing::error!("❌ Error in {}: {}", error.table_name, error.error);
        Ok(())
    }

    /// Called before writing begins (transaction start hook).
    ///
    /// Override to implement custom initialization logic, such as starting
    /// a database transaction.
    async fn begin(&self) -> Result<()> {
        Ok(())
    }

    /// Called after successful write (transaction commit hook).
    ///
    /// Override to implement custom finalization logic, such as committing
    /// a database transaction.
    async fn commit(&self) -> Result<()> {
        Ok(())
    }

    /// Called if write fails (transaction rollback hook).
    ///
    /// Override to implement custom cleanup logic, such as rolling back
    /// a database transaction.
    async fn rollback(&self) -> Result<()> {
        Ok(())
    }
}
