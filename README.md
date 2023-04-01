# Linkstamp static link page generator

Currently supports bookmark data in either
[IDM](https://github.com/rsaarelm/idm) or
[TOML](https://toml.io/) format.

See `example-links` files for how the data looks like.

Test use:

    cargo run < example-links.idm > /tmp/links.html

TOML version:

    cargo run -- --toml < example-links.toml > /tmp/links.html
