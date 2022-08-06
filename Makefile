.PHONY: client clean

client:
	npx @openapitools/openapi-generator-cli generate --generator-key rust

clean:
	rm -rf client
