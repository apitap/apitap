# 👋 Welcome to ApiTap

<p align="center">
  <img src="https://raw.githubusercontent.com/apitap/apitap/main/logo/apitap-logo.png" alt="ApiTap" width="200">
</p>

## 🚰 High-Performance ETL for Modern Data Teams

**ApiTap** is a blazingly fast HTTP-to-warehouse ETL engine built with Rust and Apache DataFusion.

### 🔧 What It Does

Stream JSON from REST APIs, transform it with SQL, and load it into your data warehouse.

```sql
-- Transform data with SQL
{{ sink(name="postgres") }}

SELECT 
    id,
    title,
    created_at
FROM {{ use_source("my_api") }}
WHERE status = 'active'
```

**Key Features:**
- ⚡ **2-5x faster** than traditional ETL tools
- 🦀 **Rust-powered** for maximum performance
- 🧠 **DataFusion** SQL engine
- 🔄 **Smart pagination** (LimitOffset, PageNumber, Cursor)
- 🐘 **PostgreSQL** support (14-17)

---

## 📊 Current Projects

### 🚀 [ApiTap](https://github.com/apitap/apitap)
Production-ready ETL engine for streaming API data to warehouses

**Stack:** Rust • Apache DataFusion • PostgreSQL • Tokio • Minijinja

**Recent Updates:**
- ✅ PostgreSQL 14-17 full support with automatic version detection
- ✅ Performance optimizations (2-5x faster database writes)
- ✅ Comprehensive documentation with 12 passing doctests
- ✅ Custom template functions for date handling

---

## 💻 Tech Stack

**Languages:**
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)
![SQL](https://img.shields.io/badge/SQL-4479A1.svg?style=flat&logo=postgresql&logoColor=white)

**Frameworks & Tools:**
![Apache DataFusion](https://img.shields.io/badge/DataFusion-blue?style=flat)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-316192?style=flat&logo=postgresql&logoColor=white)
![Tokio](https://img.shields.io/badge/Tokio-000000?style=flat)

**DevOps:**
![Git](https://img.shields.io/badge/git-%23F05033.svg?style=flat&logo=git&logoColor=white)
![GitHub](https://img.shields.io/badge/github-%23121011.svg?style=flat&logo=github&logoColor=white)

---

## 📈 GitHub Stats

![GitHub Stats](https://github-readme-stats.vercel.app/api?username=apitap&show_icons=true&theme=tokyonight&hide_border=true&bg_color=0D1117)

![Top Languages](https://github-readme-stats.vercel.app/api/top-langs/?username=apitap&layout=compact&theme=tokyonight&hide_border=true&bg_color=0D1117)

---

## 🎯 Current Focus

- 🔄 Adding ClickHouse and BigQuery writers
- ⚡ PostgreSQL COPY protocol (10-100x faster bulk loads)
- 🔐 OAuth2 authentication support
- 📊 Schema evolution handling
- 🌐 HTTP/2 support

---

## 📫 Get In Touch

- 💼 **Repository:** [github.com/apitap/apitap](https://github.com/apitap/apitap)
- 📝 **Documentation:** Check out the [README](https://github.com/apitap/apitap#readme)
- 🐛 **Issues:** Found a bug? [Report it](https://github.com/apitap/apitap/issues)
- 💡 **Ideas:** Have a feature request? [Discuss it](https://github.com/apitap/apitap/discussions)

---

## 🤝 Contributing

We welcome contributions in:
- 🦀 Rust development (especially data engineering)
- 📊 Apache DataFusion optimizations
- 🔧 ETL/ELT tooling features
- 🚀 Performance improvements
- 📖 Documentation

**Want to contribute?** PRs are welcome! See our [Contributing Guide](https://github.com/apitap/apitap#-contributing).

---

<p align="center">
  <strong>🧙‍♂️ Making data pipelines magical, one query at a time</strong>
</p>

<p align="center">
  <a href="https://github.com/apitap/apitap">
    <img src="https://img.shields.io/github/stars/apitap/apitap?style=social" alt="GitHub Stars">
  </a>
  <a href="https://github.com/apitap/apitap/network/members">
    <img src="https://img.shields.io/github/forks/apitap/apitap?style=social" alt="GitHub Forks">
  </a>
</p>

---

*Built with ❤️ and Rust*
