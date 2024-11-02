clippy:
	cargo +nightly clippy --all-targets -- -W clippy::all  -W clippy::pedantic

.PHONY: clippy
