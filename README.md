# Hana

## Installation
curl -L https://raw.githubusercontent.com/iankressin/hana/main/install.sh | bash


## Development

Setup your environment
- `mkdir ~/.hana && cd ~/.hana`
- `mkdir records && echo "{}" >> records/folders.json`

Setup Tauri and its dependencies:
- [Tauri installation page](https://tauri.studio/en/docs/getting-started/setup-linux)

Running the project
- `git clone https://github.com/iankressin/hana`
- `cd hana && npm install`
- Open 2 shell windows:
  - In the first `npm start`
  - In the second: `npm run tauri dev`
