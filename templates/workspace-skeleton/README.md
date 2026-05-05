# {{ project_canonical_name }}

{{ description }}

## Overview

This project was generated using the MurMur Workspace Skeleton. It follows a **Ports and Adapters** (Hexagonal) architecture to ensure modularity, testability, and long-term maintainability.

### Workspace Structure

- **`crates/{{ project_slug }}-core`**: The heart of the application. Contains domain logic, pure business rules, and Port definitions.
- **`crates/{{ project_slug }}-app`**: The glue layer. Implements Adapters and orchestrates workflows using the core domain.
- **`crates/{{ project_slug }}-cli`**: The command-line entry point.

## Getting Started

### Prerequisites

- Rust {{ rust_version }} or later.
- [prek](https://github.com/...) for pre-commit hooks (optional).

### Basic Commands

- **Build**: `cargo build`
- **Test**: `cargo test`
- **Lint**: `cargo clippy`
- **Format**: `cargo fmt`

## Quality & Compliance

This workspace is configured with:
- **Strict Linting**: See `clippy.toml`.
- **Dependency Auditing**: Managed via `cargo-deny` (`deny.toml`).
- **CI/CD**: GitHub Actions are located in `.github/workflows/ci.yml`.

## License

Licensed under:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms
or conditions.
