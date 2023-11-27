# Introduction

[![crates.io](https://img.shields.io/crates/v/nanopub.svg)](https://crates.io/crates/nanopub)
[![PyPI](https://img.shields.io/pypi/v/nanopub-sign)](https://pypi.org/project/nanopub-sign/)
[![npm](https://img.shields.io/npm/v/@nanopub/sign)](https://www.npmjs.com/package/@nanopub/sign)

This project aims to provide a comprehensive cross-platform toolkit to sign, publish, and check **[Nanopublications](https://nanopub.net)**.

It enables developers to:

- Sign and publish nanopubs using a RSA private key, with support for configuration through a `profile.yml` file.
- Check the validity of signed or unsigned nanopubs.

It is packaged to be used easily through various popular interfaces:

- ⌨️ Binary with a CLI for use in the terminal
- 🦀 Crate `nanopub` for Rust
- 🐍 Pip package `nanopub-sign` for Python
- 📦️ NPM package `@nanopub/sign` for JavaScript (compiled to WebAssembly) in the browser, or with NodeJS

On all platforms:

- 🐧 Linux
- 🍎 MacOS
- 🪟 Windows
- 🦊 Web browsers

The library automatically handles most RDF serializations supporting quads for the nanopub:

- TriG
- Nquads
- JSON-LD

When signing a nanopub, some metadata are created automatically in the pubinfo graph if they are not already set in the provided RDF:

- Date and time of the nanopub creation is added using `dct:created`
- ORCID of the creator is added using `dct:creator` if an ORCID was provided in the profile used to sign the nanopub (we also check if the ORCID has been set with `prov:wasAttributedTo` or `pav:createdBy`)

Checkout the page most adapted to your use-case to get started.

> 💡 If you are facing any problem, or have ideas to help improve this project, please [create an issue](https://github.com/vemonet/nanopub-rs/issues) on GitHub.
