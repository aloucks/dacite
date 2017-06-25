# dacite-winit

This is a small interoperability library for [dacite] and [winit], which allows the creation of
Vulkan surfaces in an easy and platform-independent manner.

[dacite]: https://gitlab.com/dennis-hamester/dacite/tree/master/dacite
[winit]: https://github.com/tomaka/winit

[![build status](https://gitlab.com/dennis-hamester/dacite/badges/master/build.svg)](https://gitlab.com/dennis-hamester/dacite)

## Quick Links

 - Crate on crates.io: <https://crates.io/crates/dacite-winit>
 - Documentation: <https://docs.rs/dacite-winit>

## Usage

Dacite-winit is available on [crates.io]. Add this to your `Cargo.toml`:

```toml
[dependencies]
dacite = "0.4"
dacite-winit = "0.3"
winit = "0.6"
```

Check out the [examples] subdirectory to get started.

[crates.io]: https://crates.io/crates/dacite-winit
[examples]: https://gitlab.com/dennis-hamester/dacite/tree/master/examples

## License

Dacite-winit is licensed under the ISC license:

```
Copyright (c) 2017, Dennis Hamester <dennis.hamester@startmail.com>

Permission to use, copy, modify, and/or distribute this software for any
purpose with or without fee is hereby granted, provided that the above
copyright notice and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
PERFORMANCE OF THIS SOFTWARE.
```
