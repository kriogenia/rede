it_test:
	cargo test -p rede -- --include-ignored

serve_doc:
	cd book && mdbook serve --open

test:
	cargo test
	cargo test -p rede_parser --all-features

.PHONY: it_test, serve_doc, test