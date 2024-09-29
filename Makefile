.PHONY: build
build:
	@cargo build -p rede
	@cargo build -p rede_schema
	@cargo build -p rede_parser
	@cargo build -p rede_placeholders

.PHONY: it_test
it_test:
	@cargo test -p rede -- --include-ignored

.PHONY: serve_doc
serve_doc:
	@cd book && mdbook serve --open

.PHONY: test
test:
	@cargo test -p rede
	@cargo test -p rede_schema
	@cargo test -p rede_parser --all-features
	@cargo test -p rede_placeholders
