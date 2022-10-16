# Yew Tauri Desktop App Template

This is Template App for Desktop App.

## Run Develop

```bash
  cargo tauri dev
```

or

```bash
  cargo make gui
```

When you want to check only web view, you can run next commands.

```bash
trunk serve
```

or

```bash
  cargo make dev
```

## Build App

```bash
  cargo tauri build
```

or

```bash
  cargo make gui-build
```

## Setup

This template uses cargo-make.
Setup requires two commands when Rust is already installed.

```bash
  cargo install cargo-make
  cargo make install
```

## Doc

### yew

You can access your site with [http://localhost:3000](http://localhost:3000).
src-tauri is ignored in trunk.toml because trunk builds programs infinitely.

### tauri

configure is src-tauri
