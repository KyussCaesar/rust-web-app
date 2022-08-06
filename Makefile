.PHONY: all basic-test serve-docs client clean

all:

serve-docs:
	cargo doc
	@echo 'navigate to http://localhost:8000/rust_web_app/index.html'
	@echo 'press CTRL-C when yer done'
	python -m http.server --directory target/doc

client:
	npx @openapitools/openapi-generator-cli generate --generator-key rust

clean:
	rm -rf client target
