TLA CLI
=======

Command-line tool providing linting, formatting, and TLC invocation for TLA+ projects.

How to get the binary
---------------------
- From source (current flow):
  - Build once and run: `cargo build --release` then use `./target/release/tla ...`
  - Or install into your cargo bin dir: `cargo install --path .` then run `tla ...`
- When published to crates.io: `cargo install tla` (not yet published).

Prerequisites (Linux)
---------------------
- Rust (MSRV 1.79+)
- Java runtime (for TLC)
- `tlafmt` formatter: `cargo install tlafmt`
- `tlc` model checker: download `tla2tools.jar` and add a wrapper script on `PATH` (snap `tlaplus` ships only the Toolbox GUI, not `tlc`).

Install tlc (wrapper script, no hardcoded paths)
-----------------------------------------------
```
# inside your project (or any directory)
mkdir -p tools
curl -L -o tools/tla2tools.jar \
  https://github.com/tlaplus/tlaplus/releases/latest/download/tla2tools.jar

# generate a wrapper that uses that jar
cargo run -- doctor \
  --write-tlc-wrapper tools/tlc \
  --jar tools/tla2tools.jar

# put it on PATH (add to your shell rc to persist)
export PATH="$PWD/tools:$PATH"

# verify
cargo run -- doctor

# Reminder: keep the tools directory on PATH in future shells (e.g., add the export line to ~/.zshrc).
```

You can also set `TLA_TOOLS_JAR=/path/to/tla2tools.jar` and run:
```
cargo run -- doctor --write-tlc-wrapper /some/bin/tlc
```

Usage
-----
```
# Lint TLA+ files (default .)
cargo run -- lint [PATH...]

# JSON diagnostics
cargo run -- lint path --json

# Format using tlafmt
cargo run -- fmt [PATH...]

# Model check with TLC
cargo run -- check --spec MySpec [--cfg MySpec.cfg]

# Environment check (tlafmt/tlc presence, optional wrapper creation)
cargo run -- doctor [--write-tlc-wrapper <PATH>] [--jar <tla2tools.jar>]
```

Exit codes: success 0; lint errors or formatter/check failures 1; unexpected internal errors non-zero.

Development & Tests
-------------------
```
cargo fmt
cargo clippy -- -D warnings
cargo test

# Smoke (examples)
cargo run -- lint fixtures/ok.tla
cargo run -- lint fixtures/unused.tla --json
cargo run -- check --spec Minimal   # requires tlc on PATH
```

Notes
-----
- Linting uses Tree-sitter and runs even without tlafmt/tlc installed.
- Formatting and checking require the external tools; doctor helps detect and set them up.
