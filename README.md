# philipshue

[![Build Status](https://travis-ci.org/Orangenosecom/philipshue.svg?branch=master)](https://travis-ci.org/Orangenosecom/philipshue)
[![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/github/Orangenosecom/philipshue?branch=master&svg=true)](https://ci.appveyor.com/project/Orangenosecom/philipshue)
[![Crates.io](https://img.shields.io/crates/v/philipshue.svg?style=flat-square)](https://crates.io/crates/philipshue)
[![Licence](https://img.shields.io/github/license/Orangenosecom/philipshue.svg?style=flat-square)](https://github.com/Orangenosecom/philipshue/blob/master/LICENCE)
[![Docs.rs](https://docs.rs/philipshue/badge.svg)](https://docs.rs/philipshue)

Library for interacting with the Hue API in order to control Hue lights.

The goal of this library is to provide an easy way of interacting with the Hue API using Rust.

## Current features

- Discovering a bridge by querying the Philips Hue website or via UPnP (currently requires nightly)
- Finding, manipulating and deleting lights from the bridge
- Define, get and manipulate groups of lights from the bridge

## SSL problems, when building with UPnP feature

When building, you might encounter problems with OpenSSL.
You may have to manually tell Rust where OpenSSL is located through environment variables.
Have a look at the [README of rust-openssl][rust-openssl] for more help.

If you'd rather like to not use SSL, you can disable it by turning off
default features and use UPnP for discovering instead:

```toml
[dependencies.philipshue]
version = "*"
default-features = false
features = ["unstable"]
```
Although this currently requires using nightly Rust.

[rust-openssl]: https://github.com/sfackler/rust-openssl#building
