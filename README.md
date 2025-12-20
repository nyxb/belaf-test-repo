# belaf-test-repo

Rust monorepo for testing the belaf CLI and GitHub App.

## Structure

```
belaf-test-repo/
├── apps/
│   ├── cli/          # Command-line interface
│   ├── api/          # API server
│   └── web/          # Web application
├── packages/
│   ├── utils/        # Shared utility functions
│   └── config/       # Configuration management
└── Cargo.toml        # Workspace configuration
```

## Building

```bash
cargo build
```

## Running

```bash
# Run CLI
cargo run -p cli

# Run API
cargo run -p api

# Run Web
cargo run -p web
```

## Testing

```bash
cargo test
```

## License

MIT
