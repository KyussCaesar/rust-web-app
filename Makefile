.PHONY: client clean

client: openapi.yml
	npx @openapitools/openapi-generator-cli generate -i $< -g rust -o $@

clean:
	rm -rf client
