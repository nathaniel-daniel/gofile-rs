# gofile-rs

A Rust library to interact with [gofile.io](https://gofile.io).

## CLI

This repository contains a small CLI to upload and download files.

### Installing

#### Downloads

You can a build for Windows [here](https://github.com/nathaniel-daniel/gofile-rs/releases/download/nightly/gofile.exe).

#### Build

Alternatively, you can build this yourself.
Clone this repo, install Rust, open a terminal in that folder, then run `cargo build -p gofile-cli --release`.
You should find your program in the `target` folder.

### Download a file

```bash
gofile-cli get <url>
```

### Upload a file (needs user token in config)

```bash
gofile-cli upload <file-path>
```

### Upload a file without logging in

```bash
gofile-cli upload <file-path> --use-guest
```

## Library Documentation

https://nathaniel-daniel.github.io/gofile-rs/gofile/

## License

Licensed under either of

- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
  at your option.

## Contributing

Unless you explicitly state otherwise,
any contribution intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license,
shall be dual licensed as above,
without any additional terms or conditions.
