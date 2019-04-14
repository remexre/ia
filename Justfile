all: check build-debug doc test clippy
clean:
	cargo clean
watch TARGET="all":
	watchexec -cre frag,lalrpop,rs,toml,vert "just {{TARGET}}"

build: build-debug build-release
build-debug:
	cargo build
build-release:
	cargo build --release
check:
	cargo check --all
clippy:
	cargo clippy --all
doc:
	cargo doc --all
test:
	cargo test --all
	cargo test --all --release

outdated-deps:
	cargo outdated -R

debug-tool +ARGS="":
	cargo run --bin ia-internal-debug-tool -- {{ARGS}}

fuzz-iqm:
	cd components/iqm; cargo +nightly fuzz run fuzz_target_1 fuzz/corpus/fuzz_target_1 \
		$(find ../.. -type f -name '*.iqm' | sed -r 's#/[^/]+$##' | sort | uniq)
