# 🐶 Hana

Hana is a way to share files across computers connected to the same network using mDNS as its foundation.

The reason behind Hana is to never go out into the web when is not needed. It's fairly common to share files across devices that are not connected to the same network using overkill and untrustworthy platforms such as Google Drive, Dropbox, WhatsApp, Messanger and many others. This project aims to provide a way to share files locally in a safe and reliable way, so you don't need to worry about your privacy, performance and security.

The way Hana works is through a combination of mDNS to discover peers on the local network and a TCP server to exchange files back and forth. The underlying building block behind [Hana](https://github.com/iankressin/hana) and [Hana CLI](https://github.com/iankressin/hana-cli) are [Hana Client](https://github.com/iankressin/hana-client) and [Hana Server](https://github.com/iankressin/hana-server). 

(Documentation on Hana Client and Hana Server soon)

## ⚠️ Alpha Warning

This project is in alpha phase so it's in constant change and some features may not work as expected. 

We appreciate if you take your time to report bugs or open a feature request!

## 📥 Installation

```jsx
curl -L https://raw.githubusercontent.com/iankressin/hana/main/install.sh | bash
```

## 🔧 Development

Setup your environment

- `mkdir ~/.hana && cd ~/.hana`
- `mkdir records && echo "{}" >> records/folders.json`

Setup Tauri and its dependencies:

- [Tauri installation page](https://tauri.studio/en/docs/getting-started/setup-linux)

Running the project

- `git clone https://github.com/iankressin/hana`
- `cd hana && npm install`
- Open 2 shell windows:
    - In the first `npm start`
    - In the second: `npm run tauri dev`

## ↔️ Interoperability

**Hana** and **Hana CLI** were built to work together, so it's safe to share files using the CLI and the GUI versions of Hana.

## 🤕 Known Issues

Currently Hana do not support:
- Retry send files if no server was found on the first try
- Windows OS (Windows doesn't support mDNS out of the box as [RFC 6762](https://tools.ietf.org/html/rfc6762) describes. We're still trying to figure out how to make Hana compatible across OSs. If you ever worked with Microsoft's LLMNR before, your help will be much appreciated 🙃)
