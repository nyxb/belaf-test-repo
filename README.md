# belaf-test-repo

Rust monorepo with multiple independent workspaces for testing the belaf CLI and GitHub App.

## Structure

```
belaf-test-repo/
├── apps/
│   ├── cli/                    # CLI application (v0.1.0)
│   │   ├── Cargo.toml          # Workspace config
│   │   └── crates/
│   │       ├── core/           # Core library
│   │       └── bin/            # Binary
│   ├── api/                    # API server (v0.2.0)
│   │   ├── Cargo.toml          # Workspace config
│   │   └── crates/
│   │       ├── core/           # Core library
│   │       └── bin/            # Binary
│   └── web/                    # Web application (v0.3.0)
│       ├── Cargo.toml          # Workspace config
│       └── crates/
│           ├── core/           # Core library
│           └── bin/            # Binary
└── rust-toolchain.toml
```

Each app is an independent Cargo workspace with its own version.

## Building

```bash
# Build CLI
cd apps/cli && cargo build

# Build API
cd apps/api && cargo build

# Build Web
cd apps/web && cargo build
```

## Running

```bash
# Run CLI
cd apps/cli && cargo run

# Run API
cd apps/api && cargo run

# Run Web
cd apps/web && cargo run
```

## Testing

```bash
cd apps/cli && cargo test
cd apps/api && cargo test
cd apps/web && cargo test
```

## License

MIT
