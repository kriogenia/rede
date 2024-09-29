.PHONY: build
build:
	@cargo build --workspace --exclude rede_placeholders
	@cargo build -p rede_placeholders

.PHONY: it_test
it_test:
	@cargo test -p rede -- --include-ignored

.PHONY: serve_doc
serve_doc:
	@cd book && mdbook serve --open

.PHONY: test
test:
	@cargo test --workspace --exclude rede_placeholders --no-default-features
	@cargo test --workspace --exclude rede_placeholders --all-features
	@cargo test -p rede_placeholders
