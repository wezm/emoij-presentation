Emoji Presentation
==================

This is a small tool that prints the `Emoji` and `Emoji_Presentation` Unicode
properties for each character on the command line.

Building
--------

`emoji-presentation` is implemented in [Rust]. Once you have [installed Rust]
run:

    cargo build --release

### Installing

    cargo install --path .

Running
-------

    $ emoji-presentation ✅❌
    ✅ Emoji=true, Emoji_Presentation=true
    ❌ Emoji=true, Emoji_Presentation=true

Licence
-------

This project is dual licenced under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/wezm/leaf/blob/master/LICENSE-APACHE))
- MIT license ([LICENSE-MIT](https://github.com/wezm/leaf/blob/master/LICENSE-MIT))

at your option.

[Rust]: https://www.rust-lang.org/
[installed Rust]: https://www.rust-lang.org/tools/install
