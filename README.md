# rust web app

try building a web app in rust again.

openapi.yml describe the API.

development requires:

- `asdf-vm` version manager
- `rust` installed
- `nodejs` installed (so that we can use openapi-generator-cli)
- `java` installed (the openapi-generator-cli is just a wrapper around a JAR
  which requires java installed to run)
- `docker` run the app in container with database and monitoring

```bash
asdf plugin add rust
asdf plugin add nodejs
asdf plugin add java

asdf install # install from .tool-versions
npm install # install from package.json, for openapi-generator-cli
```

# Development

`make up` does a few things:

- make sure local API client is up to date by re-generating it from openapi.yml (`make client`)
- make sure app binary is up to date and using newest version of local API client by re-building it (`cargo build`)
- make sure API docs are up to date by re-building it (`cargo doc`)
- run `docker-compose up --build` which does some more things:
  - ensure local application image has newest version of app by re-building it
  - ensure local prometheus image has newest version of configuration by re-building it
  - start all the services

some useful things are bound to host ports:

- `http://localhost:8080` the web app (can `curl` to it or run `cargo run --bin test`)
- `http://localhost:7979/rust_web_app/index.html` the docs (open in your browser)
- `http://localhost:8081` adminer (new version of phpMyAdmin, open in your browser)
- `http://localhost:9090` prometheus (open in your browser)

Other goodies added in the compose file:

- `node-exporter` make metrics about the node (host) available in local prometheus (`localhost:9090`)
- `postgres-exporter` make metrics about the local postgres available in local prometheus (`localhost:9090`)

# API Client Generation

`make client` will generate the rust client.

docs for the generator are here:

https://openapi-generator.tech/docs/generators/rust

if you need to customise it later, refer to those docs and put the values into
`openapitools.json`