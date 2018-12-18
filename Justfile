all: check doc build-debug
clean:
	cargo clean
watch TARGET="all":
	watchexec -re rs,toml "just {{TARGET}}"

bench:
	cargo +nightly bench --all
build: build-debug build-release
build-debug:
	cargo build --all
build-release:
	cargo build --all --release
check:
	cargo check --all
clippy:
	cargo +nightly clippy --all
doc:
	cargo doc --all
run +ARGS="":
	cargo run --bin game -- {{ARGS}}
run-release +ARGS="":
	cargo run --bin game --release -- {{ARGS}}
test:
	cargo test --all

# Inkwell is a bit weird with how it does versioning, which results in breakage often...
fix-inkwell:
	cargo update -p inkwell

open-docs:
	cargo doc --open -p engine
outdated-deps:
	cargo outdated -R
