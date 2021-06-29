# Yew Tauri Desktop App Template

This is Template App for Desktop App.

## Run Develop
```bash
$ cargo tauri dev
```

## Build App
```bash
$ cargo tauri build
```

## Setup 
### yew setup
[Using trunk](https://yew.rs/ja/next/getting-started/project-setup/using-trunk)

This project is using master branch(not yet released) and not support cargo-web(yew is removing cargo-web because it tries to eliminate the stdweb library).

```bash
$ cargo install trunk
$ cargo install wasm-bindgen-cli
```


### tauri setup

You need to specify the version for installing tauri-cli now (2021/06/30).

```bash
$ cargo install tauri-cli --version 1.0.0-beta.4
```