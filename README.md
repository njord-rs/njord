<img align="right" width="128" height="128" alt="njord" src="https://github.com/njord-rs/njord/blob/78d13f9143f5b99dedab7f2561b9fb0e8ca2b902/resources/logo.png">

# Njord <!-- omit in toc -->

![build](https://img.shields.io/github/actions/workflow/status/njord-rs/njord/core.yml?branch=master)
![crates.io](https://img.shields.io/crates/v/njord.svg)
[![documentation](https://img.shields.io/badge/docs-njord-blue?logo=rust)](https://docs.rs/njord/latest/)

A highly versatile and feature-rich ORM library for Rust, designed to simplify database interactions across a wide range of systems with robust performance and flexibility.

## Table of Contents <!-- omit in toc -->

- [Feature Support by Database](#feature-support-by-databases)
- [Migrations CLI](#migrations-cli)
- [Machine Learning Tools](#machine-learning-tools)
  - [Feature Roadmap](#feature-roadmap)
- [Getting Help](#getting-help)
- [Reporting Issues](#reporting-issues)
- [Contributing](#contributing)
- [Code of Conduct](#code-of-conduct)
- [Contributors](#contributors)
- [License](#license)

## Feature Support by Database

| Database         | JOIN | SELECT | INSERT | UPDATE | DELETE | Raw SQL | Transactions |
|------------------|-----|--------|-------|--------|-------|--------|----------|
| SQLite           | ✅   | ✅      | ✅     | ✅      | ✅     | 🏗️    | 🏗         |
| PostgreSQL       | 🏗️ | 🏗️    | 🏗️   | 🏗️    | 🏗️   | 🏗️    | 🏗️      |
| MySQL            | ✅   | ✅      | ✅     | ✅      | ✅     | 🏗️    | 🏗         |
| MariaDB          | ✅  | ✅️    | ✅    | ✅️    | ✅    | ✅     | 🏗️      |
| Oracle           | ✅   | ✅      | ✅     | ✅      | ✅     | 🏗️    | 🏗         |
| MSSQL            | ✅   | ✅      | ✅     | ✅      | ✅     | 🏗️    | 🏗         |

## Migrations CLI

_Work on CLI migrations is currently in paused under the `njord_cli` directory. No database is supported yet. Below is the planned feature tracking table once we start again._

| Database         | Create Migration | Apply Migration | Rollback Migration | Migration History | Seed Data | Schema Diffing |
|------------------|------------------|-----------------|--------------------|-------------------|-----------|----------------|
| SQLite           | ❌                | ❌               | ❌                  | ❌                 | ❌         | ❌              |
| PostgreSQL       | ❌                | ❌               | ❌                  | ❌                 | ❌         | ❌              |
| MySQL            | ❌                | ❌               | ❌                  | ❌                 | ❌         | ❌              |
| MariaDB          | ❌                | ❌               | ❌                  | ❌                 | ❌         | ❌              |
| Oracle           | ❌                | ❌               | ❌                  | ❌                 | ❌         | ❌              |
| MSSQL            | ❌                | ❌               | ❌                  | ❌                 | ❌         | ❌              |

## Machine Learning Tools

The `njord_ml` is a specialized library for machine learning workflows.
It provides features to facilitate efficient data handling for ML pipelines, including data extraction, transformation, and injection.
This toolset will be designed to integrate seamlessly with popular ML frameworks while maintaining robust database support.

### Feature Roadmap

- [ ] Data Preparation
    - [ ] In-database preprocessing
    - [ ] Query optimization for ML models
    - [ ] Data source loading and merging
    - [ ] Data extraction and transformation
- [ ] Handling Large Datasets
  - [ ] Streaming large datasets
  - [ ] Batch processing for large datasets
- [ ] Data Management
  - [ ] Data injection for ML models
  - [ ] Data validation and verification
  - [ ] Data versioning and tracking
  - [ ] Data lineage and provenance
- [ ] Data Quality Assurance
  - [ ] Data quality and consistency checks

## Getting Help

Are you having trouble with Njord? We want to help!

- Read through the documentation on our [docs](https://docs.rs/njord/latest/njord/).

- If you are upgrading, read the release notes for upgrade instructions and "new and noteworthy" features.

- Ask a question we monitor stackoverflow.com for questions tagged with Njord.

- Report bugs with Njord at https://github.com/njord-rs/njord/issues.

## Reporting Issues

Njord uses GitHub’s integrated issue tracking system to record bugs and feature requests. If you want to raise an issue, please follow the recommendations below:

- Before you log a bug, please search the issue tracker to see if someone has already reported the problem.

- If the issue doesn’t already exist, create a new issue.

- Please provide as much information as possible with the issue report. We like to know the Njord version, operating system, and Rust version version you’re using.

- If you need to paste code or include a stack trace, use Markdown. ``` escapes before and after your text.

- If possible, try to create a test case or project that replicates the problem and attach it to the issue.

## Contributing

Before contributing, please read the [contribution](https://github.com/njord-rs/njord/blob/master/CONTRIBUTING.md) guide for useful information how to get started with Njord as well as what should be included when submitting a contribution to the project.

## Code of Conduct

Anyone who interacts with Njord in any space, including but not limited to this GitHub repository, must follow our code of conduct.

## Contributors

The following contributors have either helped to start this project, have contributed
code, are actively maintaining it (including documentation), or in other ways
being awesome contributors to this project. **We'd like to take a moment to recognize them.**

[<img src="https://github.com/mjovanc.png?size=72" alt="mjovanc" width="72">](https://github.com/mjovanc)
[<img src="https://github.com/appelskrutt34.png?size=72" alt="appelskrutt34" width="72">](https://github.com/appelskrutt34)
[<img src="https://avatars.githubusercontent.com/u/23294573?v=4&size=72">](https://github.com/ahsentekd)
[<img src="https://avatars.githubusercontent.com/u/167654108?v=4&size=72">](https://github.com/chinmer)
[<img src="https://github.com/SvMak.png?size=72" alt="SvMak" width="72">](https://github.com/SvMak)
[<img src="https://github.com/TomasWild.png?size=72" alt="TomasWild" width="72">](https://github.com/TomasWild)
[<img src="https://github.com/chaseWillden.png?size=72" alt="chaseWillden" width="72">](https://github.com/chaseWillden)
[<img src="https://github.com/Hiccup-za.png?size=72" alt="Hiccup-za" width="72">](https://github.com/Hiccup-za)

## License

The BSD 3-Clause License.
