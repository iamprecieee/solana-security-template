# Contributing

Thank you for your interest in improving the Solana Security Template project.

## Quick Start

```bash
git clone https://github.com/iamprecieee/solana-security-template
cd solana-security-template
```

## Development

| Command | Purpose |
|---------|---------|
| `anchor build` | Compile the Solana programs |
| `cargo test --manifest-path tests/Cargo.toml` | Run the exploit tests |
| `cargo clippy` | Check for common linting issues |
| `cargo fmt` | Format the code according to project standards |

## Pull Requests

1. Fork the repository and create a feature branch.
2. Ensure all instructions have both a `vulnerable_*` and `secure_*` version.
3. Add tests in `tests/src/` for any new vulnerability.
4. Update documentation in `docs/` and the `README.md` files.

## Code Style

- Use `cargo fmt` to maintain consistent formatting.
- Handle errors explicitly using the `thiserror` based enum in `error.rs`.

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
