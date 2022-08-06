# rust web app

try building a web app in rust again.

openapi.yml describe the API.

development requires:

- `asdf-vm` version manager
- `rust` installed
- `nodejs` installed (so that we can use openapi-generator-cli)
- `java` installed (the openapi-generator-cli is just a wrapper around a JAR
  which requires java installed to run)

```bash
asdf plugin add rust
asdf plugin add nodejs
asdf plugin add java

asdf install # install from .tool-versions
npm install # install from package.json, for openapi-generator-cli
```

# API Client Generation

For testing the application it is good to use client generated from the API spec
because that is what other people will use.

`make client` will generate the rust client.

docs for the generator are here:

https://openapi-generator.tech/docs/generators/rust

if you need to customise it later, refer to those docs and put the values into
openapitools.json

you can view docs for the crate and all dependencies with `make serve-docs`.
