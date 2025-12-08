<p align="center">
  <img src="logo/apitap-logo.png" alt="ApiTap" width="300">
</p>

<p align="center">
  <strong>Stream APIs to Your Warehouse with SQL</strong>
</p>

<p align="center">
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT"></a>
  <img src="https://img.shields.io/badge/rust-1.70+-orange.svg" alt="Rust 1.70+">
  <a href="https://datafusion.apache.org/"><img src="https://img.shields.io/badge/powered%20by-DataFusion-blue" alt="Powered by DataFusion"></a>
  <img src="https://img.shields.io/badge/status-production%20ready-green" alt="Production Ready">
</p>


# 🚰 ApiTap

**Stream JSON from REST APIs, transform with SQL, load into your warehouse**  
*High-performance HTTP-to-warehouse ETL engine powered by Apache DataFusion & Rust*

**Quick links:**  
[What is ApiTap?](#-what-is-apitap) • [Features](#-features) • [Performance](#-performance) • [Installation](#-installation) • [Quick Start](#-quick-start) • [Architecture](#-architecture) • [Roadmap](#%EF%B8%8F-roadmap)

---

## 🎯 What is ApiTap?

ApiTap is a lightweight, high-performance ETL engine that:

1. **Extracts** JSON from HTTP/REST APIs (with smart pagination)
2. **Transforms** it using **SQL** (Apache DataFusion)
3. **Loads** it into data stores (PostgreSQL, with more coming soon)

You describe:

- **What to do** in SQL modules (with Minijinja templating), and  
- **Where to get data / where to put it** in a YAML config.

ApiTap handles the complex parts: pagination, retries, streaming JSON processing, schema inference, and efficient database writes.

### Who is this for?

- You like **Rust** and **SQL** and want a simple, fast HTTP-to-warehouse tool  
- You have APIs (analytics, SaaS tools, internal services) and don't want to run a huge ETL platform  
- You'd rather keep transformations as **SQL files in git** than scattered across app code
- You need **performance** - ApiTap is optimized for throughput and efficiency

It's great for small/medium data stacks, side projects, production pipelines, and learning DataFusion.

---

## ⚡ Performance

ApiTap has been profiled and optimized using flamegraph analysis for production-ready performance:

- **Optimized batch processing** - 5000 rows per batch write
- **Lock-free operations** - Atomic counters for streaming
- **Efficient async I/O** - 8192-item channel buffers
- **Concurrent execution** - Parallel HTTP requests with Tokio
- **Memory efficient** - Zero-copy operations where possible
- **Production tested** - Streaming architecture for large datasets

### Architecture Optimizations

| Component | Optimization | Benefit |
|-----------|-------------|---------|
| Database writes | Batch processing (5000 rows) | Reduced roundtrips |
| API requests | Concurrent with Tokio | Parallel execution |
| JSON streaming | Zero-copy where possible | Lower memory usage |
| State tracking | Atomic counters | Lock-free performance |

### Optimization Documentation

See detailed performance analysis in:
- **[FLAMEGRAPH_ANALYSIS.md](FLAMEGRAPH_ANALYSIS.md)** - Profiling results & bottleneck analysis
- **[OPTIMIZATIONS_APPLIED.md](OPTIMIZATIONS_APPLIED.md)** - What was optimized & how to test
- **[OPTIMIZATION_REPORT.md](OPTIMIZATION_REPORT.md)** - Future optimization opportunities

---

## ⚠️ Status

> **Active development - Production ready**

- ✅ **PostgreSQL 14-17 fully supported**  
  - Automatic version detection and method selection
  - PostgreSQL 15+: Uses native `MERGE` statements
  - PostgreSQL 9.5-14: Falls back to `INSERT ... ON CONFLICT DO UPDATE`
  - PostgreSQL <9.5: Clear error message with upgrade recommendation
- Core features are stable, but expect some API changes
- Feedback, bug reports, and PRs are very welcome!

---

## ✨ Features

### Working now

- 🧩 **SQL modules with Minijinja templating**  
  - `{{ sink(name="postgres_sink") }}` declares a target  
  - `{{ use_source("json_place_holder") }}` binds a source table  
  - `{{ schedule("0 */3 * * * *") }}` sets cron schedule for automated execution
  - Full templating support for dynamic SQL generation
- 📁 **Module loader** for entire `--modules` folder of `.sql` files
- 🎭 **Template engine** that captures sinks & sources at render time
- ⏰ **Built-in Scheduler**
  - Cron-based job scheduling (6-field format: seconds, minutes, hours, day, month, day of week)
  - Concurrent job execution with independent configurations
  - Graceful shutdown with Ctrl+C
  - Automatic retry on failures
  - Clean, readable logging output
- 🌐 **HTTP client with smart pagination**
  - ✅ **LimitOffset** (e.g., `?_limit=50&_start=100`)
  - ✅ **PageNumber** (e.g., `?page=2&per_page=50`)
  - ✅ **PageOnly** (e.g., `?page=2`)
  - 🔄 **Cursor** (e.g., `?cursor=xxx`) - *Coming soon*
  - ✅ Automatic retry with exponential backoff
  - ✅ Configurable concurrency
- 🧠 **DataFusion-backed SQL execution**
  - Full SQL support (joins, aggregations, window functions)
  - Streaming execution for memory efficiency
  - Schema inference from JSON data
- 🐘 **PostgreSQL writer**
  - Auto-create tables from inferred schema
  - Merge/upsert by primary key (using `MERGE` statements)
  - Optimized batch writes (5000 rows per batch)
  - Uses Postgres 17+ today; compatibility work for 14–16 in progress
- 🏭 **Writer factory pattern** for extensibility
- 🖥️ **CLI runner** with:
  - `--modules` / `-m` (SQL folder)
  - `--yaml-config` / `-y` (pipeline config)
  - `--log-json` (JSON formatted logs)
  - `--log-level` (control verbosity)
- 📊 **Structured logging** with tracing
  - Human-readable or JSON output
  - Detailed spans for profiling
  - Request/response logging

### In progress / planned

- 🔄 Additional pagination modes (improvements)
- 🔄 ClickHouse writer
- 🔄 BigQuery writer
- 🔄 Incremental sync state management
- 🔄 OAuth2 authentication
- 🔄 Schema evolution handling
- 🔄 Better Postgres compatibility (14+ support)
- 🔄 PostgreSQL COPY protocol (10-100x faster bulk inserts)
- 🔄 HTTP/2 support for reduced SSL overhead

Legend: ✅ Working • 🔄 In Progress • 📝 Planned

---

## 📦 Installation

### Option 1 — Build from source

**Requirements:**
- Rust toolchain (1.70+ recommended)
- PostgreSQL 17+ for full compatibility (14+ supported with limitations)

**Clone and build:**

```bash
git clone https://github.com/abduldjafar/apitap.git
cd apitap

# Build a release binary (optimized)
cargo build --release
```

This produces `target/release/apitap`.

**Run in place:**

```bash
./target/release/apitap --help
```

**Or install to PATH:**

```bash
# Copy to system bin
sudo cp target/release/apitap /usr/local/bin/apitap

# Or use cargo install
cargo install --path .
```

Then use anywhere:

```bash
apitap -m ./examples/sql -y ./examples/config/pipelines.yaml
```

### Option 2 — Download binary (coming soon)

Pre-built binaries will be available for:
- Linux (x86_64, aarch64)
- macOS (Intel, Apple Silicon)
- Windows (x86_64)

---

## 🚀 Quick Start

### 1) Project structure

```text
examples/
  sql/                      # SQL transformation modules
    example.sql
    testing_2.sql
    placeholder/
      post.sql
  config/
    pipelines.yaml          # API sources and database targets
```

### 2) Create a SQL module

**`examples/sql/posts.sql`**

```sql
-- Declare where results should go
{{ sink(name="postgres_sink") }}

-- Query the API data
SELECT 
    id,
    userId as user_id,
    title,
    body,
    CURRENT_TIMESTAMP as loaded_at
FROM {{ use_source("json_placeholder_posts") }}
WHERE userId > 5;
```

#### Custom Template Functions

ApiTap provides built-in template functions for dynamic queries:

**`current_date()`** - Returns today's date in YYYY-MM-DD format
```yaml
query_params:
  - key: updated_since
    value: "{{ current_date() }}"
```

**`few_date_ago(n)`** - Returns date n days ago in YYYY-MM-DD format
```yaml
query_params:
  - key: start_date
    value: "{{ few_date_ago(7) }}"  # 7 days ago
  - key: end_date
    value: "{{ current_date() }}"
```

**Real-world example:**
```yaml
sources:
  - name: api_recent_changes
    url: https://api.example.com/changes
    query_params:
      - key: from
        value: "{{ few_date_ago(1) }}"  # Yesterday
      - key: to
        value: "{{ current_date() }}"   # Today
```

These functions are particularly useful for:
- Incremental data loads
- Date range filtering
- Daily/periodic API queries
- Avoiding hardcoded dates in configs

### 3) Configure sources and targets

**`examples/config/pipelines.yaml`**

```yaml
sources:
  - name: json_placeholder_posts
    url: https://jsonplaceholder.typicode.com/posts
    table_destination_name: posts
    pagination:
      kind: limit_offset
      limit_param: _limit
      offset_param: _start
    retry:
      max_attempts: 3
      min_delay_secs: 1
      max_delay_secs: 10

targets:
  - name: postgres_sink
    type: postgres
    auth:
      # Using environment variables (recommended)
      username_env: POSTGRES_USER
      password_env: POSTGRES_PASSWORD
      # Or hardcode (not recommended for production)
      # username: postgres
      # password: postgres
    host: localhost
    port: 5432
    database: mydb
```

### 4) Set up environment

**Create `.env` file:**

```bash
POSTGRES_USER=postgres
POSTGRES_PASSWORD=yourpassword
```

### 5) Run the pipeline

```bash
# Basic run
apitap -m examples/sql -y examples/config/pipelines.yaml

# With debug logging
apitap -m examples/sql -y examples/config/pipelines.yaml --log-level debug

# With JSON logs (for production/parsing)
apitap -m examples/sql -y examples/config/pipelines.yaml --log-json
```

**What happens:**

1. 🔍 ApiTap discovers all `.sql` files in `examples/sql/`
2. 🎨 Renders them with Minijinja, capturing `sink()` and `use_source()` calls
3. ⚙️ Resolves sources/targets from `pipelines.yaml`
4. 🌐 Fetches data via HTTP with automatic pagination
5. 🔄 Streams data through DataFusion SQL transformations
6. 💾 Writes results to PostgreSQL with upsert/merge logic

### 6) Verify the results

```bash
psql -U postgres -d mydb -c "SELECT COUNT(*) FROM posts;"
```

---

## ⏰ Scheduling Jobs

ApiTap includes a built-in scheduler for automated, recurring pipeline executions using cron expressions.

### Adding Schedules to SQL Modules

Add a `{{ schedule("...") }}` directive to any SQL module:

**`examples/sql/scheduled_posts.sql`**

```sql
{{ sink(name="postgres_sink") }}
{{ schedule("0 */3 * * * *") }}  -- Run every 3 minutes

SELECT 
    id,
    userId as user_id,
    title,
    CURRENT_TIMESTAMP as loaded_at
FROM {{ use_source("json_placeholder_posts") }}
WHERE userId > 5;
```

### Cron Expression Format

ApiTap uses a **6-field cron format** (with seconds):

```
┌───────────── second (0-59)
│ ┌───────────── minute (0-59)
│ │ ┌───────────── hour (0-23)
│ │ │ ┌───────────── day of month (1-31)
│ │ │ │ ┌───────────── month (1-12)
│ │ │ │ │ ┌───────────── day of week (0-6, 0=Sunday)
│ │ │ │ │ │
│ │ │ │ │ │
* * * * * *
```

**Common Examples:**

```sql
-- Every minute at second 0
{{ schedule("0 * * * * *") }}

-- Every 5 minutes
{{ schedule("0 */5 * * * *") }}

-- Every hour at minute 0
{{ schedule("0 0 * * * *") }}

-- Every day at 2:30 AM
{{ schedule("0 30 2 * * *") }}

-- Every Monday at 9:00 AM
{{ schedule("0 0 9 * * 1") }}

-- Every 15 minutes during business hours (9 AM - 5 PM)
{{ schedule("0 */15 9-17 * * 1-5") }}
```

### Running the Scheduler

When you run ApiTap, it automatically detects schedules and starts the scheduler:

```bash
apitap -m examples/sql -y examples/config/pipelines.yaml
```

**Output:**

```
🚀 Starting Apitap Pipeline Execution
═══════════════════════════════════════════════════════════
📂 Discovered 3 SQL module(s)
⚙️  Configuration loaded successfully
📅 Scheduled job 'posts.sql' with cron: 0 */3 * * * *
📅 Scheduled job 'users.sql' with cron: 0 0 * * * *
📅 Scheduled job 'analytics.sql' with cron: 0 30 2 * * *
⏰ Scheduler started. Press Ctrl+C to stop.
═══════════════════════════════════════════════════════════
```

### Scheduler Behavior

- **Concurrent Execution**: Multiple jobs can run simultaneously if their schedules overlap
- **Independent Configs**: Each job runs with its own source/sink configuration
- **Automatic Retry**: Failed jobs are logged but don't stop the scheduler
- **Graceful Shutdown**: Press `Ctrl+C` to stop all jobs and exit cleanly
- **Clean Logging**: Concise output showing start time, duration, and records processed

**Job Execution Logs:**

```
🔄 Running: posts.sql | json_placeholder_posts → posts
✅ Completed: posts.sql | 100 records | 2341ms
✅ Scheduled job 'posts.sql' completed successfully
⏰ Next execution for 'posts.sql': 2025-12-08T14:03:00Z
```

### Performance Notes

- Jobs run **in parallel** when scheduled at the same time
- Each job uses its own HTTP client and database connection
- Resource contention may occur with many concurrent jobs
- Consider staggering schedules for heavy workloads:
  ```sql
  -- Job 1: runs at :00
  {{ schedule("0 0 * * * *") }}
  
  -- Job 2: runs at :15
  {{ schedule("0 15 * * * *") }}
  
  -- Job 3: runs at :30
  {{ schedule("0 30 * * * *") }}
  ```

### Without Scheduling

If you don't include `{{ schedule("...") }}`, the job runs once and exits (original behavior):

```bash
# Run all jobs once and exit
apitap -m examples/sql -y examples/config/pipelines.yaml
```

---

## 🏗️ Architecture

```text
┌─────────────────────────────────────────────────────────────┐
│                      CLI (main.rs)                          │
│  apitap -m DIR -y FILE [--log-json] [--log-level LEVEL]   │
└────────────────────────┬────────────────────────────────────┘
                         │
┌────────────────────────▼─────────────────────────────────────┐
│              cmd::run_pipeline (orchestrator)                │
│  • Loads SQL modules & YAML config                          │
│  • Renders templates & captures metadata                    │
│  • Coordinates fetch → transform → load pipeline            │
└────────────────────────┬─────────────────────────────────────┘
                         │
         ┌───────────────┴───────────────┐
         │                               │
┌────────▼─────────┐          ┌─────────▼────────────┐
│  Config Layer    │          │  Templating Engine   │
│                  │          │                      │
│ • YAML parsing   │          │ • Minijinja env     │
│ • Source defs    │          │ • sink() capture    │
│ • Target defs    │          │ • use_source()      │
│ • Auth config    │          │ • SQL rendering     │
└────────┬─────────┘          └─────────┬────────────┘
         │                               │
         └───────────────┬───────────────┘
                         │
         ┌───────────────▼────────────────┐
         │     HTTP Fetcher Layer         │
         │                                │
         │ • Pagination drivers           │
         │   - LimitOffset                │
         │   - PageNumber                 │
         │   - PageOnly                   │
         │   - Cursor                     │
         │ • Retry with backoff           │
         │ • Concurrent requests          │
         │ • Streaming NDJSON parser      │
         └───────────────┬────────────────┘
                         │
         ┌───────────────▼────────────────┐
         │   DataFusion Processing        │
         │                                │
         │ • Schema inference             │
         │ • SQL query execution          │
         │ • Streaming operators          │
         │ • Memory-efficient batches     │
         └───────────────┬────────────────┘
                         │
         ┌───────────────▼────────────────┐
         │     Writer Factory             │
         │                                │
         │ • PostgreSQL (MERGE/upsert)    │
         │ • ClickHouse (planned)         │
         │ • BigQuery (planned)           │
         │ • Optimized batch writes       │
         │ • Auto table creation          │
         └────────────────────────────────┘
```

### Key Components

- **CLI**: Command-line interface built with clap
- **Config**: YAML-based configuration with env var support
- **Templating**: Minijinja for SQL modules with custom functions
- **HTTP**: Async reqwest-based client with pagination support
- **DataFusion**: SQL execution engine with streaming support
- **Writers**: Pluggable database writer implementations
- **Logging**: Structured tracing with JSON output support

---

## 🔧 Configuration Reference

### Source Configuration

```yaml
sources:
  - name: my_api                       # Unique identifier
    url: https://api.example.com/data  # Base URL
    table_destination_name: my_table   # Target table name
    
    # Data path (optional - for nested JSON responses)
    # Omit this field if API returns root-level array: [{"id": 1}, {"id": 2}]
    # Include if data is nested: {"data": [{"id": 1}], "meta": {...}}
    data_path: /data                   # JSONPath to extract array from response
    
    # Pagination (choose one)
    pagination:
      # Option 1: Limit/Offset
      kind: limit_offset
      limit_param: limit
      offset_param: offset
      
      # Option 2: Page Number
      # kind: page_number
      # page_param: page
      # size_param: per_page
      
      # Option 3: Page Only
      # kind: page_only
      # page_param: page
      
      # Option 4: Cursor-based
      # kind: cursor
      # cursor_param: cursor
    
    # Retry configuration
    retry:
      max_attempts: 3
      min_delay_secs: 1
      max_delay_secs: 30

# Examples of data_path usage

# Example 1: Root-level array (no data_path needed)
# API response: [{"id": 1, "name": "foo"}, {"id": 2, "name": "bar"}]
sources:
  - name: github_repos
    url: https://api.github.com/users/octocat/repos
    # data_path omitted - API returns array at root level
    
# Example 2: Nested data
# API response: {"data": [{"id": 1}, {"id": 2}], "pagination": {...}}
sources:
  - name: nested_api
    url: https://api.example.com/items
    data_path: /data  # Extract the "data" array
    
# Example 3: Deeply nested
# API response: {"response": {"items": [...]}}
sources:
  - name: deep_nested
    url: https://api.example.com/deep
    data_path: /response/items  # Extract "items" from "response"
```

### Target Configuration

```yaml
targets:
  - name: postgres_sink
    type: postgres
    auth:
      username_env: POSTGRES_USER    # From environment
      password_env: POSTGRES_PASSWORD
      # OR hardcoded (not recommended):
      # username: postgres
      # password: password
    host: localhost
    port: 5432                       # Optional, defaults to 5432
    database: mydb
```

---

## 📊 Logging & Debugging

### Log Levels

```bash
# Info (default)
apitap -m sql -y config.yaml

# Debug (verbose)
apitap -m sql -y config.yaml --log-level debug

# Trace (very verbose)
apitap -m sql -y config.yaml --log-level trace

# Warn only
apitap -m sql -y config.yaml --log-level warn
```

### JSON Logs (for production)

```bash
# Output logs as JSON for parsing/monitoring
apitap -m sql -y config.yaml --log-json

# Combine with log level
apitap -m sql -y config.yaml --log-json --log-level info
```

### Environment Variables

```bash
# Alternative to --log-level
export RUST_LOG=debug
apitap -m sql -y config.yaml

# For JSON logs
export APITAP_LOG_JSON=1
apitap -m sql -y config.yaml
```

---

## 🎯 Performance Tuning

### For High Throughput

1. **Increase concurrency** in your YAML config
2. **Use larger batch sizes** (already optimized to 5000)
3. **Enable connection pooling** for your database
4. **Profile with flamegraph** to find bottlenecks

### Profiling

```bash
# Install cargo-flamegraph
cargo install flamegraph

# Generate performance profile
cargo flamegraph --release -- -m examples/sql -y examples/config/pipelines.yaml

# Open the resulting SVG
open flamegraph.svg
```

### Monitoring

Watch the structured logs for:
- `http.request` - HTTP request latencies
- `http.ndjson_stream` - Stream processing stats
- `sql.execute` - SQL execution time
- `transform.load` - Records loaded

---

## 🛣️ Roadmap

**Core Features** ✅

* [x] Minijinja SQL templates with capture
* [x] Multi-mode pagination (LimitOffset, PageNumber, PageOnly, Cursor)
* [x] DataFusion SQL execution
* [x] PostgreSQL writer (MERGE/upsert, tested on 17+)
* [x] Writer factory pattern
* [x] CLI with full flag support
* [x] Structured logging (human & JSON)
* [x] Retry with exponential backoff
* [x] Performance optimization (2-5x faster)

**Postgres Compatibility**

* [x] PostgreSQL 14-17 fully supported
* [x] Automatic version detection and method selection
* [x] PostgreSQL 15+: Native `MERGE` statements
* [x] PostgreSQL 9.5-14: `INSERT ... ON CONFLICT DO UPDATE`
* [x] Version caching with RwLock for performance

**Performance** (Ongoing)

* [x] Flamegraph profiling & analysis
* [x] Optimized batch sizes (100 → 5000)
* [x] Lock-free atomic counters
* [x] Increased channel buffers (256 → 8192)
* [ ] PostgreSQL COPY protocol (10-100x faster bulk loads)
* [ ] HTTP/2 support (reduced SSL overhead)
* [ ] SIMD JSON parsing
* [ ] Connection pool tuning

**Next Features**

* [ ] ClickHouse writer
* [ ] BigQuery writer
* [ ] Parquet file writer
* [ ] State management for incremental loads
* [ ] OAuth2 authentication
* [ ] Schema evolution/migrations
* [ ] Webhook/streaming ingestion
* [ ] dbt-like dependency management
* [ ] Web UI for monitoring

**DevOps & Releases**

* [ ] Comprehensive test suite
* [ ] CI/CD pipeline
* [ ] Pre-built binaries (releases)
* [ ] Docker image
* [ ] Kubernetes manifests
* [ ] Performance benchmarks
* [ ] Documentation site

---

## 🤝 Contributing

Contributions are very welcome! Whether you're:
- 🐛 Reporting bugs
- 💡 Suggesting features
- 📝 Improving documentation
- 🔧 Submitting PRs

**Getting started:**

```bash
git clone https://github.com/abduldjafar/apitap.git
cd apitap

# Build and test
cargo build
cargo test

# Run a test pipeline
./target/debug/apitap -m examples/sql -y examples/config/pipelines.yaml

# Check code style
cargo fmt --check
cargo clippy
```

### Development Setup

1. Install Rust (rustup recommended)
2. Install PostgreSQL for testing
3. Copy `.env.example` to `.env` and configure
4. Run `cargo build` to fetch dependencies

---

## 📚 Resources

### Documentation
- [FLAMEGRAPH_ANALYSIS.md](FLAMEGRAPH_ANALYSIS.md) - Performance analysis
- [OPTIMIZATIONS_APPLIED.md](OPTIMIZATIONS_APPLIED.md) - Optimization guide
- [OPTIMIZATION_REPORT.md](OPTIMIZATION_REPORT.md) - Future improvements
- [examples/](examples/) - Example SQL modules and configs

### External Resources
- [Apache DataFusion](https://datafusion.apache.org/) - SQL engine
- [Minijinja](https://docs.rs/minijinja/) - Template engine
- [Tokio](https://tokio.rs/) - Async runtime

---

## 📄 License

MIT — see [LICENSE](LICENSE)

---

## 🙏 Acknowledgments

Built with amazing open-source projects:
- **Apache DataFusion** - Lightning-fast SQL execution engine
- **Tokio** - Async runtime for Rust
- **Reqwest** - HTTP client
- **Minijinja** - Template engine
- **Tracing** - Structured logging

---

**Made with ❤️ and Rust**
