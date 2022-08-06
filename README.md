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

`Makefile` contains command to build the client: `make client`
