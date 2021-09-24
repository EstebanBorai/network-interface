<div>
  <h1 align="center">network-interface</h1>
  <h4 align="center">
    Retrieve system's Network Interfaces/Adapters on Linux, macOS and Windows
    on a standarized manner
  </h4>
</div>

<div align="center">

  [![Crates.io](https://img.shields.io/crates/v/network-interface.svg)](https://crates.io/crates/network-interface)
  [![Documentation](https://docs.rs/network-interface/badge.svg)](https://docs.rs/network-interface)
  ![Build](https://github.com/EstebanBorai/network-interface/workflows/build/badge.svg)
  ![Clippy](https://github.com/EstebanBorai/network-interface/workflows/clippy/badge.svg)
  ![Formatter](https://github.com/EstebanBorai/network-interface/workflows/fmt/badge.svg)

</div>

> This crate is under development, feel free to contribute on [GitHub](https://github.com/EstebanBorai/network-interface). API and implementation is subject to change.

The main goal of `network-interface` crate is to retrieve system's Network
Interfaces in a standarized manner.

_standarized manner_ means that every supported platform must expose the same
API and no further changes to the implementation are required to support such
platform.

## Release

In order to create a release you must push a Git tag as follows

```sh
git tag -a <version> -m <message>
```

**Example**

```sh
git tag -a v0.1.0 -m "First release"
```

> Tags must follow semver conventions
> Tags must be prefixed with a lowercase `v` letter.

Then push tags as follows:

```sh
git push origin main --follow-tags
```

## Contributing

Every contribution to this project is welcome. Feel free to open a pull request,
an issue or just by starting this project.

## License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0)
