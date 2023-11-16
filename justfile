set dotenv-load

alias r := run

bt := '0'

log := "warn"


@_list:
	just --list --unsorted

# Perform all verifications (compile, test, lint, etc.)
@verify: test lint
    echo ------------ verify done! ------------


# Run the cli locally (from sources)
run *args:
	cargo run {{args}}

	

install:
    cargo build --release
    cargo install --path .

# Watch the source files and run `just verify` when source changes
watch:
	cargo watch -- just verify

# Run the tests
test:
	cargo hack test --feature-powerset --locked

# Run the static code analysis
lint:
	cargo fmt --all -- --check
	cargo hack clippy --feature-powerset --all-targets --workspace --locked

# Install cargo dev-tools used by the `verify` recipe (requires rustup to be already installed)
install-dev-tools:
	rustup install stable
	rustup override set stable
	cargo install cargo-hack cargo-watch cargo-deny hurl

clean:
	rm -rf target
	rm -f Cargo.lock
	rm -rf node_modules

fmt:
  cargo fmt
