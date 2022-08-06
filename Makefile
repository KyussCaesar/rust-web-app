.PHONY: setup client

setup:
	asdf install
	npm install

client: openapi.yml
	npx @openapitools/openapi-generator-cli generate -i $< -g rust -o $@