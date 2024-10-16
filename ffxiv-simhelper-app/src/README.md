# environment setting
```bash
npm install -g yarn
yarn install
```

# dev testing
```bash
# for app
$Env:NODE_ENV="application"

# for server
$Env:NODE_ENV="server"

yarn dev
```

# Release Test using Serve:
```bash
yarn build
yarn serve -s dist
```

# electron testing
```bash
yarn electron-pack
cd dist
# execute file
```
