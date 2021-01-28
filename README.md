Emoji Presentation
==================

This is a small tool that prints the `Emoji` and `Emoji_Presentation` Unicode
properties for each character on the command line.

![Unicode 13.0](https://img.shields.io/badge/Unicode-13.0-informational?logo=Unicode&logoColor=whitesmoke)

Building
--------

`emoji-presentation` is implemented in [Rust]. Once you have [installed Rust]
run:

    cargo build --release

### Installing

    cargo install --path .

Running
-------

    $ emoji-presentation ✅❌☂@
    ✅ Emoji=true, Emoji_Presentation=true
    ❌ Emoji=true, Emoji_Presentation=true
    ☂ Emoji=true, Emoji_Presentation=false
    @ Emoji=false, Emoji_Presentation=false

Licence
-------

This project is dual licenced under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/wezm/leaf/blob/master/LICENSE-APACHE))
- MIT license ([LICENSE-MIT](https://github.com/wezm/leaf/blob/master/LICENSE-MIT))

at your option.

[Rust]: https://www.rust-lang.org/
[installed Rust]: https://www.rust-lang.org/tools/install
