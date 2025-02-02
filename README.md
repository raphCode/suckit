![Build and test](https://github.com/Skallwar/suckit/workflows/Build%20and%20test/badge.svg)
[![codecov](https://codecov.io/gh/Skallwar/suckit/branch/master/graph/badge.svg?token=ZLD369AY2G)](https://codecov.io/gh/Skallwar/suckit)
[![Crates.io](https://img.shields.io/crates/v/suckit.svg)](https://crates.io/crates/suckit)
[![Docs](https://docs.rs/suckit/badge.svg)](https://docs.rs/suckit)
[![Deps](https://deps.rs/repo/github/Skallwar/suckit/status.svg)](https://deps.rs/repo/github/Skallwar/suckit)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![MSRV](https://img.shields.io/badge/MSRV-1.46.0-blue)

# SuckIT

`SuckIT` allows you to recursively visit and download a website's content to
your disk.

![SuckIT Logo](media/suckit_logo.png)

# Features

* [x] Vacuums the entirety of a website recursively
* [x] Uses multithreading
* [x] Writes the website's content to your disk
* [x] Enables offline navigation
* [x] Offers random delays to avoid IP banning
* [ ] Saves application state on CTRL-C for later pickup

# Options

|Option|Behavior|
|---|---|
|`-h, --help`|Displays help information|
|`-v, --verbose`|Activate Verbose output|
|`-d, --depth`|Specify the level of depth to go to when visiting the website. Default is -1 (infinity)|
|`--ext-depth`|Specify the level of depth to go to when visiting websites that have a different domain name. Default is 0 (ignore external links), -1 is infinity|
|`-j, --jobs`|Number of threads to use|
|`-o, --output`|Output directory where the downloaded files are written|
|`-t, --tries`|Number of times to retry when the downloading of a page fails|
|`-u, --user-agent`|User agent to be used for sending requests|
|`-i, --include`|Specify a regex to include pages that match this pattern|
|`-e, --exclude`|Specify a regex to exclude pages that match this pattern|
|`-a, --auth`|Provide usernames and passwords for the downloader to use|
|`--dry-run`|Do everything without saving the files to the disk|

# Example

A common use case could be the following:

`suckit http://books.toscrape.com -j 8 -o /path/to/downloaded/pages/`

![asciicast](media/suckit-adjusted-120cols-40rows-100ms.svg)

# Installation

As of right now, `SuckIT` does not work on Windows.

To install it, you need to have Rust installed.

* Check out [this link](https://www.rust-lang.org/learn/get-started) for
instructions on how to install Rust.

* If you just want to install the suckit executable, you can simply run
`cargo install --git https://github.com/skallwar/suckit`

* Now, run it from anywhere with the `suckit` command.

### Arch Linux

`suckit` can be installed from available [AUR packages](https://aur.archlinux.org/packages/?O=0&SeB=b&K=suckit&outdated=&SB=n&SO=a&PP=50&do_Search=Go) using an [AUR helper](https://wiki.archlinux.org/index.php/AUR_helpers). For example,

```
yay -S suckit
```

__Want to contribute ? Feel free to
[open an issue](https://github.com/Skallwar/suckit/issues/new) or
[submit a PR](https://github.com/Skallwar/suckit/compare) !__

# License

SuckIT is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0)

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
