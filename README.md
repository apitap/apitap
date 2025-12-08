<p align="center">
  <img src="logo/apitap-logo.png" alt="ApiTap" width="200">
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

**High-performance HTTP-to-warehouse ETL engine built with Rust and Apache DataFusion**

ApiTap lets you extract JSON from REST APIs, transform it with SQL, and load it into PostgreSQL—all using simple SQL templates with built-in scheduling.

## ✨ Key Features

- 🦀 **Rust-powered** - 2-5x faster than traditional ETL tools
- 🧠 **SQL transformations** - Apache DataFusion for powerful data processing
- ⏰ **Built-in scheduler** - Cron-based automation with concurrent execution
- 🔄 **Smart pagination** - LimitOffset, PageNumber, PageOnly modes
- 🐘 **PostgreSQL 14-17** - Full support with optimized MERGE operations
- 🎨 **SQL templating** - Minijinja templates with custom functions

## 🚀 Quick Start

```bash
# Clone and build
git clone https://github.com/abduldjafar/apitap.git
cd apitap
cargo build --release

# Run a pipeline
./target/release/apitap -m examples/sql -y examples/config/pipelines.yaml
```

### Example SQL Module

```sql
{{ sink(name="postgres_sink") }}
{{ schedule("0 */3 * * * *") }}  -- Run every 3 minutes

SELECT 
    id,
    userId as user_id,
    title,
    CURRENT_TIMESTAMP as loaded_at
FROM {{ use_source("api_posts") }}
WHERE userId > 5;
```

## 📚 Documentation

- 📖 **[Full Documentation](index.html)** - Complete guide with examples
- 🎯 **[Configuration Guide](examples/config/pipelines.yaml)** - YAML config reference
- 💡 **[SQL Examples](examples/sql/)** - Sample SQL transformation modules

## 🛠️ Configuration

**`pipelines.yaml`** - Define your sources and targets:

```yaml
sources:
  - name: api_posts
    url: https://api.example.com/posts
    table_destination_name: posts
    pagination:
      kind: limit_offset
      limit_param: limit
      offset_param: offset

targets:
  - name: postgres_sink
    type: postgres
    auth:
      username_env: POSTGRES_USER
      password_env: POSTGRES_PASSWORD
    host: localhost
    port: 5432
    database: mydb
```

## 🎯 Use Cases

- **SaaS Data Integration** - Pull data from APIs into your warehouse
- **Analytics Pipelines** - Transform API data with SQL for analysis
- **Data Syncing** - Keep databases updated with scheduled jobs
- **ETL Automation** - Replace complex ETL scripts with SQL templates

## ⚡ Performance

Built with Rust and Apache DataFusion for high performance:

- **Optimized batch processing** - 5000 rows per batch write
- **Concurrent execution** - Efficient async I/O with Tokio
- **Lock-free operations** - Atomic counters for streaming
- **Memory efficient** - Zero-copy operations where possible
- **Profiled & optimized** - Flamegraph analysis applied

## 🤝 Contributing

Contributions welcome! Please check:
- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
- [GitHub Issues](https://github.com/abduldjafar/apitap/issues) - Report bugs or request features

## 📄 License

MIT - see [LICENSE](LICENSE)

## 🙏 Built With

- [Apache DataFusion](https://datafusion.apache.org/) - SQL execution engine
- [Tokio](https://tokio.rs/) - Async runtime
- [Minijinja](https://docs.rs/minijinja/) - Template engine
- [Tokio-cron-scheduler](https://docs.rs/tokio-cron-scheduler/) - Job scheduling

---

**Made with ❤️ and Rust** | [Website](index.html) | [GitHub](https://github.com/abduldjafar/apitap)
