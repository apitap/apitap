use reqwest::Client;
use std::sync::Arc;
use url::Url;

use crate::http::fetcher::FetchStats;
use crate::pipeline::QueryParam;
use crate::utils::template;
use crate::{
    errors::{ApitapError, Result},
    http::fetcher::{DataFusionPageWriter, PaginatedFetcher, Pagination},
    writer::{DataWriter, WriteMode},
};

#[derive(Debug, Clone)]
pub struct FetchOpts {
    pub concurrency: usize,
    pub default_page_size: usize,
    pub fetch_batch_size: usize, // internal http batch size
}

/// Configuration for the HTTP fetch request
#[derive(Debug)]
pub struct FetchRequest {
    pub client: Client,
    pub url: Url,
    pub data_path: Option<String>,
    pub extra_params: Option<Vec<QueryParam>>,
    pub pagination: Option<Pagination>,
    pub retry: crate::pipeline::Retry,
}

/// Configuration for SQL query execution
#[derive(Debug)]
pub struct QueryConfig<'a> {
    pub sql: &'a str,
    pub dest_table: &'a str,
}

/// Configuration for data writing
pub struct WriteConfig {
    pub writer: Arc<dyn DataWriter>,
    pub write_mode: WriteMode,
}

fn clean_param(params: Option<Vec<QueryParam>>) -> Result<Vec<(String, String)>> {
    match params {
        Some(params) => params
            .into_iter()
            .map(|q| {
                let key = q.key;
                let val = template::substitute_templates(&q.value)?;
                Ok((key, val))
            })
            .collect(),
        None => Ok(Vec::new()),
    }
}
pub async fn run_fetch(
    request: FetchRequest,
    query: QueryConfig<'_>,
    write_config: WriteConfig,
    opts: &FetchOpts,
) -> Result<FetchStats> {
    let page_writer = Arc::new(DataFusionPageWriter::new(
        query.dest_table,
        query.sql,
        write_config.writer.clone(),
    ));

    // Convert QueryParam to (String, String) tuples
    let extra_params_vec: Vec<(String, String)> = clean_param(request.extra_params)?;

    match request.pagination {
        Some(Pagination::LimitOffset {
            limit_param,
            offset_param,
        }) => {
            let fetcher = PaginatedFetcher::new(request.client, request.url, opts.concurrency)
                .with_limit_offset(&limit_param, &offset_param)
                .with_batch_size(opts.fetch_batch_size);

            let page_size: u64 = opts.default_page_size.try_into().map_err(|_| {
                ApitapError::ConfigError(format!(
                    "Invalid page size: {} (must fit in u64)",
                    opts.default_page_size
                ))
            })?;

            let stats = fetcher
                .fetch_limit_offset(
                    page_size,
                    request.data_path,
                    Some(&extra_params_vec),
                    None,
                    page_writer,
                    write_config.write_mode,
                    &request.retry,
                )
                .await?;
            Ok(stats)
        }

        Some(Pagination::PageNumber {
            page_param,
            per_page_param,
        }) => {
            let page_writer = Arc::new(DataFusionPageWriter::new(
                query.dest_table,
                query.sql,
                write_config.writer.clone(),
            ));

            let fetcher = PaginatedFetcher::new(request.client, request.url, opts.concurrency)
                .with_batch_size(opts.fetch_batch_size)
                .with_page_number(&page_param, &per_page_param);

            let per_page: u64 = opts.default_page_size.try_into().map_err(|_| {
                ApitapError::ConfigError(format!(
                    "Invalid page size: {} (must fit in u64)",
                    opts.default_page_size
                ))
            })?;

            let stats = fetcher
                .fetch_page_number(
                    per_page,
                    request.data_path.as_deref(),
                    None,
                    page_writer,
                    write_config.write_mode,
                    &request.retry,
                )
                .await?;

            Ok(stats)
        }

        Some(Pagination::PageOnly { page_param: _ }) => {
            let _fetcher = PaginatedFetcher::new(request.client, request.url, opts.concurrency)
                .with_batch_size(opts.fetch_batch_size);
            Ok(FetchStats::new())
        }

        Some(Pagination::Cursor {
            cursor_param: _,
            page_size_param: _,
        }) => {
            let _fetcher = PaginatedFetcher::new(request.client, request.url, opts.concurrency)
                .with_batch_size(opts.fetch_batch_size);
            Ok(FetchStats::new())
        }

        Some(Pagination::Default) | None => Err(ApitapError::PaginationError(
            "no supported pagination configured".into(),
        )),
    }
}
