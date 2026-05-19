# RustHelloWorld

Bare-bones Rust hello-world app with end-to-end test coverage and GitHub Actions CI/CD.

## Run locally

```bash
cargo run
```

Expected output:

```text
Hello, world!
```

## Test locally

```bash
cargo test
```

This includes:
- a unit test (`src/lib.rs`) for the greeting logic
- an end-to-end test (`tests/e2e.rs`) that executes the compiled binary and verifies output

## CI/CD and deployment

- **CI** (`.github/workflows/ci.yml`): runs format check, clippy, and tests on pushes and pull requests.
- **Release deployment** (`.github/workflows/release.yml`): on tags like `v1.0.0`, builds a release binary and publishes a tarball to a GitHub Release.
