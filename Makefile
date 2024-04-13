it_test:
	cargo test -p rede -- --include-ignored

serve_doc:
	cd book && mdbook serve --open

.PHONY: it_test, serve_doc