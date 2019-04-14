# Checks, builds, documents, and tests everything.
all: check clippy build-debug doc build-release test

# Removes compilation artifacts.
clean:
	cargo clean

# Watches the compilation of a target.
watch TARGET="all":
	watchexec -cre frag,lalrpop,rs,toml,vert "just {{TARGET}}"

# Builds in both debug and release configurations.
build: build-debug build-release
# Builds the project in the debug configuration.
build-debug:
	cargo build
# Builds the project in the release configuration.
build-release:
	cargo build --release
# Checks that the project can compile.
check:
	cargo check --all
# Checks for additional lints.
clippy:
	cargo clippy --all
# Creates documentation.
doc:
	cargo doc --all
# Tests in both debug and release configurations.
test:
	cargo test --all
	cargo test --all --release

# Checks for outdated dependencies.
outdated-deps:
	cargo outdated -R

# Runs the debug tool.
debug-tool +ARGS="":
	cargo run --bin ia-internal-debug-tool -- {{ARGS}}

# Fuzzes the IQM parser.
fuzz-iqm:
	cd components/iqm; cargo +nightly fuzz run fuzz_target_1 fuzz/corpus/fuzz_target_1 \
		$(find ../.. -type f -name '*.iqm' | sed -r 's#/[^/]+$##' | sort | uniq)
