# Development

### First make sure you are inside the /frontend/ directory.
Install latest node using [nvm](https://github.com/nvm-sh/nvm):
```bash
nvm install --lts && nvm use --lts
```

Install all dependencies:
```bash
npm install
```

Use [parcel](https://parceljs.org/) to serve files:
```bash
npm start
```

Use tsc to compile typescript:
```bash
tsc --watch
```

Parcel is a little finnicky and sometimes requires deleting `.parcel-cache` to fix errors.

Before opening a PR run `npm run lint && npm run format` or CI will reject it.

# Release
TODO
