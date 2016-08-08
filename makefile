all:
	cargo build
	bash -c 'SLOG_LEVEL=Trace ./target/debug/hybrida-rs'
clean:
	cargo clean
dev:
	cargo build --features dev
	bash -c 'SLOG_LEVEL=Trace ./target/debug/hybrida-rs'
fmto:
	cargo fmt -- --write-mode overwrite
