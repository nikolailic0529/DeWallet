[![codecov](https://codecov.io/gh/walletd/mnemonic/branch/main/graph/badge.svg?token=BA4YTBMLEP)](https://codecov.io/gh/walletd/mnemonic)
# walletd mnemonic

`mnemonic` is a hierarchical deterministic (HD) key generator.

More information about this crate can be found in the [crate documentation][docs].

## High level features

- BIP32.
- BIP39.
- BIP44.
- BIP49.
- BIP84.
- Monero seeds 14 word (mymonero style) or 25 words.

## Usage example

```rust
// still to come
```

You can find this [example][readme-example] as well as other example projects in
the [example directory][examples].

See the [crate documentation][docs] for way more examples.

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in
100% safe Rust.

## Minimum supported Rust version

mnemonic's MSRV is 1.60.

## Examples

The [examples] folder contains various examples of how to use `mnemonic`. The
[docs] also provide lots of code snippets and examples.

## Getting Help

In the `mnemonic`'s repo we also have a [number of examples][examples] showing how
to put everything together. You're also welcome to open a [discussion] with your question.

## Contributing

:balloon: Thanks for your help improving the project! We are so happy to have
you! We have a [contributing guide][contributing] to help you get involved in the
`mnemonic` project.

## License

Licensed under the [Apache license][license-apache], Version 2.0
or the [MIT license][license-mit], at your option. Files in the project may not be copied, modified, or distributed except according to those terms.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `mnemonic` by you, shall be licensed as MIT, without any
additional terms or conditions.

[readme-example]: https://github.com/walletd/mnemonic/tree/main/examples/readme
[examples]: https://github.com/walletd/mnemonic/tree/main/examples
[docs]: https://docs.rs/walletd_mnemonic
[contributing]: https://github.com/walletd/mnemonic/blob/main/CONTRIBUTING.md
[discussion]: https://github.com/walletd/mnemonic/discussions/new?category=q-a
[ecosystem]: https://github.com/walletd/mnemonic/blob/main/ECOSYSTEM.md
[license-mit]: https://github.com/walletd/mnemonic/blob/main/LICENSE-MIT
[license-apache]: https://github.com/walletd/mnemonic/blob/main/LICENSE-APACHE