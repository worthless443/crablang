# crablangc_tools_util

A small tool to help you generate version information
for packages installed from a git repo

## Usage

Add a `build.rs` file to your repo and list it in `Cargo.toml`
````toml
build = "build.rs"
````

List crablangc_tools_util as regular AND build dependency.
````toml
[dependencies]
crablangc_tools_util = "0.3.0"

[build-dependencies]
crablangc_tools_util = "0.3.0"
````

In `build.rs`, generate the data in your `main()`

```crablang
fn main() {
    crablangc_tools_util::setup_version_info!();
}
```

Use the version information in your main.rs

```crablang
fn show_version() {
    let version_info = crablangc_tools_util::get_version_info!();
    println!("{}", version_info);
}
```

This gives the following output in clippy:
`clippy 0.1.66 (a28f3c8 2022-11-20)`

## Repository

This project is part of the crablang/crablang-clippy repository. The source code
can be found under `./crablangc_tools_util/`.

The changelog for `crablangc_tools_util` is available under:
[`crablangc_tools_util/CHANGELOG.md`](https://github.com/crablang/crablang-clippy/blob/master/crablangc_tools_util/CHANGELOG.md)

## License

<!-- REUSE-IgnoreStart -->

Copyright 2014-2022 The CrabLang Project Developers

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
option. All files in the project carrying such notice may not be
copied, modified, or distributed except according to those terms.

<!-- REUSE-IgnoreEnd -->
