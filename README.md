# pleme-testing

Testing utilities library for Pleme platform - test fixtures, testcontainers, integration helpers

## Installation

```toml
[dependencies]
pleme-testing = "0.1"
```

## Usage

```rust
use pleme_testing::{fixtures::UserFixture, containers::PostgresContainer};

#[tokio::test]
async fn test_with_db() {
    let pg = PostgresContainer::start().await;
    let user = UserFixture::builder().email("test@example.com").build();
    // ...
}
```

## Feature Flags

| Feature | Description |
|---------|-------------|
| `containers` | Testcontainers (Postgres, Redis) |
| `database` | Database testing helpers |
| `http` | HTTP testing with reqwest |
| `integration` | pleme-database + pleme-error integration |
| `full` | All features enabled |

Enable features in your `Cargo.toml`:

```toml
pleme-testing = { version = "0.1", features = ["full"] }
```

## Development

This project uses [Nix](https://nixos.org/) for reproducible builds:

```bash
nix develop            # Dev shell with Rust toolchain
nix run .#check-all    # cargo fmt + clippy + test
nix run .#publish      # Publish to crates.io (--dry-run supported)
nix run .#regenerate   # Regenerate Cargo.nix
```

## License

MIT - see [LICENSE](LICENSE) for details.
