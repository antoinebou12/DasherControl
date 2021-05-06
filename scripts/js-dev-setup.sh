#!/usr/bin/env bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.38.0/install.sh | bash
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm
source ~/.bashrc
nvm install 14
source ~/.bashrc
cd frontend
npm install
npm i -g rimraf webpack webpack-cli
npm run build
