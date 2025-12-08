# APITap Documentation

This directory contains comprehensive documentation for APITap performance optimizations, architecture, and production deployment.

## 📚 Documentation Index

### Getting Started

- **[RUST_LEARNING_GUIDE.md](RUST_LEARNING_GUIDE.md)** - **Learning Rust? START HERE!** 🦀
  - Complete Rust tutorial using APITap as example
  - Ownership, borrowing, async/await explained
  - Traits, error handling, pattern matching
  - Reading real-world Rust code
  - 3 practice exercises included

- **[STREAMING_ARCHITECTURE.md](STREAMING_ARCHITECTURE.md)** - **Understanding the architecture**
  - Explains the factory pattern and data streaming
  - Clarifies that HTTP requests happen only once per pipeline
  - Memory buffering strategy and DataFusion integration

### Core Module Guides

- **[FETCHER_GUIDE.md](FETCHER_GUIDE.md)** - HTTP data fetching module (`src/http/fetcher.rs`)
  - Pagination strategies (limit/offset, page number, ~~cursor~~)
  - ⚠️ **Note:** Cursor-based pagination is not yet implemented
  - Memory management and buffering
  - Concurrency control
  - API reference and usage examples

- **[ENV_VARS_GUIDE.md](ENV_VARS_GUIDE.md)** - Environment variables substitution
  - Using `${ENV_VAR}` in configuration
  - Security best practices
  - Integration with headers, URLs, and query parameters

- **[TABLE_PROVIDER_GUIDE.md](TABLE_PROVIDER_GUIDE.md)** - DataFusion table provider (`src/utils/table_provider.rs`)
  - How SQL queries work over streaming data
  - Factory pattern in detail
  - Integration with DataFusion
  - Best practices

- **[EXECUTION_GUIDE.md](EXECUTION_GUIDE.md)** - Query execution plan (`src/utils/execution.rs`)
  - JSON to Arrow RecordBatch conversion
  - Execution lifecycle
  - Performance characteristics
  - Debugging and monitoring

### Performance Optimization

- **[OPTIMIZATIONS_APPLIED.md](OPTIMIZATIONS_APPLIED.md)** - Summary of all implemented optimizations
  - HTTP connection pooling (6-10% improvement)
  - Database batching (2-5x faster)
  - Atomic counters and channel buffers
  
- **[FLAMEGRAPH_OPTIMIZATIONS.md](FLAMEGRAPH_OPTIMIZATIONS.md)** - Detailed HTTP/TLS optimization guide
  - Based on actual flamegraph analysis
  - Connection pooling and Keep-Alive configuration
  - Expected performance improvements

- **[FLAMEGRAPH_REVIEW.md](FLAMEGRAPH_REVIEW.md)** - Visual flamegraph validation
  - Confirms optimization targets are correct
  - Before/after expectations
  - Verification checklist

- **[FLAMEGRAPH_ANALYSIS.md](FLAMEGRAPH_ANALYSIS.md)** - Original performance analysis report
  - Detailed bottleneck identification
  - CPU time breakdown
  - Optimization recommendations

- **[OPTIMIZATION_REPORT.md](OPTIMIZATION_REPORT.md)** - Early optimization analysis
  - Phase 1 and Phase 2 recommendations
  - Implementation priorities

### Memory Optimization

- **[MEMORY_OPTIMIZATION_GUIDE.md](MEMORY_OPTIMIZATION_GUIDE.md)** (if exists) - Complete guide for high concurrency
  - Strategies to run 1000+ concurrent pipelines
  - Memory footprint reduction (92%)
  - Adaptive buffer sizing

- **[QUICK_START_MEMORY_OPTIMIZATION.md](QUICK_START_MEMORY_OPTIMIZATION.md)** (if exists) - Quick reference
  - TL;DR commands for running 1000 pipelines with <2GB RAM
  - Configuration examples
  - Troubleshooting guide

### Production Deployment

- **[PRODUCTION_READINESS_REPORT.md](PRODUCTION_READINESS_REPORT.md)** - Production deployment checklist
  - Security considerations
  - Monitoring and observability
  - Deployment strategies
  - Operational best practices

## 🎯 Quick Navigation

### For Developers

1. Start with [STREAMING_ARCHITECTURE.md](STREAMING_ARCHITECTURE.md) to understand the codebase
2. Review [OPTIMIZATIONS_APPLIED.md](OPTIMIZATIONS_APPLIED.md) for performance context
3. Check [FLAMEGRAPH_OPTIMIZATIONS.md](FLAMEGRAPH_OPTIMIZATIONS.md) for HTTP optimization details

### For DevOps/SRE

1. Read [PRODUCTION_READINESS_REPORT.md](PRODUCTION_READINESS_REPORT.md) first
2. Review memory optimization guides for scaling
3. Check flamegraph analysis for performance tuning

### For Performance Tuning

1. [FLAMEGRAPH_ANALYSIS.md](FLAMEGRAPH_ANALYSIS.md) - Identify bottlenecks
2. [FLAMEGRAPH_OPTIMIZATIONS.md](FLAMEGRAPH_OPTIMIZATIONS.md) - Apply optimizations
3. [FLAMEGRAPH_REVIEW.md](FLAMEGRAPH_REVIEW.md) - Validate improvements

## 📊 Performance Summary

| Optimization | Impact | Documentation |
|--------------|--------|---------------|
| HTTP Connection Pooling | 6-10% faster | [FLAMEGRAPH_OPTIMIZATIONS.md](FLAMEGRAPH_OPTIMIZATIONS.md) |
| Database Batching | 2-5x faster | [OPTIMIZATIONS_APPLIED.md](OPTIMIZATIONS_APPLIED.md) |
| Memory for 1000 Pipelines | 92% reduction | [MEMORY_OPTIMIZATION_GUIDE.md](MEMORY_OPTIMIZATION_GUIDE.md) |
| Lock-free Counters | 2-5% faster | [OPTIMIZATIONS_APPLIED.md](OPTIMIZATIONS_APPLIED.md) |

## 🚀 Quick Commands

### Run with Optimizations

```bash
# Build optimized binary
cargo build --release

# Run with default config
./target/release/apitap -m examples/sql -y examples/config/pipelines.yaml
```

### High Concurrency Mode (1000 Pipelines)

```bash
# Set memory mode
export APITAP_MEMORY_MODE=minimal
export TOKIO_WORKER_THREADS=8

# Run with high-concurrency config
./target/release/apitap \
  -m examples/sql \
  -y examples/config/high-concurrency.yaml
```

### Profile Performance

```bash
# Generate flamegraph (macOS)
cargo flamegraph --release -- -m examples/sql -y examples/config/pipelines.yaml

# View flamegraph
open flamegraph.svg
```

## 🔗 External Resources

- [Main README](../README.md) - Project overview and setup
- [CHANGELOG](../CHANGELOG.md) - Version history
- [CONTRIBUTING](../CONTRIBUTING.md) - Contribution guidelines
- [SECURITY](../SECURITY.md) - Security policy

## 📝 Documentation Maintenance

This documentation is kept up to date with the codebase. If you find any discrepancies:

1. Check the git commit history for the relevant file
2. Review the actual code implementation
3. Submit an issue or PR with corrections

**Last Updated:** November 15, 2025

---

**Note:** All documentation assumes you're working with the latest version of APITap. For specific version documentation, check the git tags.
