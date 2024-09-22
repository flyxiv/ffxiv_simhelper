winget install Schniz.fnm

# npm 설치
fnm env --use-on-cd | Out-String | Invoke-Expression
fnm use --install-if-missing 20

# npm으로 yarn 설치
npm install yarn
# yarn으로 project dependency 설치
yarn install