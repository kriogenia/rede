# Installation

As this still an early stage of the project the package repository where it will
be provided is cargo, so you would need to have Rust installed and run:

```shell
cargo install -p rede
```

Of course, if you have Rust installed you can also build it and/or install it yourself.

Another alternative is to download the latest binary from the
[releases page](https://github.com/kriogenia/rede/releases) but only Linux binary
will be available for the moment.

## Minimal rede

If you won't use the `input_params` feature of rede, you can also install or build
the tool without that feature.

```sh
cargo install -p rede --no-default-features
```

