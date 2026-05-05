# Workspace Skeleton Template

This template provides a standardized, "standards-grade" Rust workspace structure designed for the MurMur ecosystem. It enforces a strict "Ports and Adapters" (Hexagonal) architecture and comes pre-configured with best-in-class tooling for linting, security, and CI/CD.

## Architecture Overview

The skeleton generates a workspace with three primary crates:

1.  **`{{ project_slug }}-core`**: The "Domain" layer. Contains business logic, entities, and port definitions. Zero dependencies on external frameworks or I/O.
2.  **`{{ project_slug }}-app`**: The "Application" layer. Orchestrates domain logic and implements adapters (e.g., database clients, external API callers).
3.  **`{{ project_slug }}-cli`**: The "Primary Adapter". A CLI interface for interacting with the application.

## Features Included

- **Linting & Quality**: Pre-configured `clippy.toml` with strict pedantic and nursery lints.
- **Security Audit**: `deny.toml` for `cargo-deny` to check for security vulnerabilities, license compliance, and duplicate dependencies.
- **Toolchain Pinning**: `rust-toolchain.toml` to ensure consistent compiler versions (defaulting to `{{ edition }}`).
- **CI/CD**: Comprehensive GitHub Actions workflow (`ci.yml`) including:
    - Multi-stage linting (fmt, clippy, prek).
    - Architecture validation.
    - Security audits.
    - DCO (Developer Certificate of Origin) sign-off checks.
- **Pre-commit Hooks**: Managed via `prek.toml`.

## Usage

This template is intended to be used with a template generator like `cargo-generate`:

```bash
cargo generate --path path/to/mrmr/templates/workspace-skeleton
```

### Prompted Variables

- `project_short_name`: The kebab-case binary name (e.g., `my-service`).
- `project_canonical_name`: The human-readable name (e.g., `My Service`).
- `authors_line`: Used in `Cargo.toml`.
- `include_prek`: Whether to include the `prek.toml` configuration.
- `include_cargo_deny`: Whether to include the `deny.toml` configuration.

## Maintenance

When updating this template:
1. Ensure the `template.toml` variables match the Tera tags used in the files.
2. Keep the GitHub Action workflow in the `copy_without_render` list in `template.toml` to avoid Tera interpolation conflicts with GitHub's `${{ }}` syntax.
