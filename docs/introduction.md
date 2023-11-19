# Introduction

This project aims to provide a comprehensive cross-platform toolkit to sign, publish, and check **[Nanopublications](https://nanopub.net)**.

It is packaged to be used easily through various popular interfaces:

- ⌨️ Binary with a CLI for use in the terminal
- 🦀 Crate `nanopub` for Rust
- 🐍 Pip package `nanopub_sign` for Python
- 📦️ NPM package `@nanopub/sign` for JavaScript (compiled to WebAssembly) in the browser, or with NodeJS

On all platforms:

- 🐧 Linux
- 🍎 MacOS
- 🪟 Windows
- 🦊 Web browsers

The library automatically handles most RDF serializations supporting graphs for the nanopub:

- TriG
- Nquads
- JSON-LD
