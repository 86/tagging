# tagging

![build](https://github.com/86/tagging/workflows/build/badge.svg)

## Overview

**tagging** is a cli tool to support git tagging easily in Rust.
What **tagging** can do is just adding incremented tags based on [semantic versioning](https://semver.org/).

## Installation

### Cargo

Install rustup (rust and cargo will be installed) if you not have cargo yet.
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install tagging
```
cargo install --git https://github.com/86/tagging
```
or
```
git clone git@github.com:86/tagging.git
cargo install --path tagging
```

## Usage

Simply run `tagging` 🏃
```
🔖 Latest tags:
v2.3.1  <-- 🎯 Target
v2.3.0
v2.2.0
:

🤖 Which position do you want to increment?
major(M) / minor(m) / patch(p):
```

Type a incrementing position ☝️
```
✅ The new tag will be: v2.4.0.
commit 4acba8f33f3edd3c4b035e3c1b998e734e12507f
Merge: e0fb394 1716a31
Author: 86 <triaedz@gmail.com>
Date:   Thu Sep 17 15:35:50 2020 +0900

    Merge pull request #6 from 86/log
    
    feat: show commit log before adding tag


🤖Are you sure you want to add the new tag?: (y/n)
```

Confirm and type `y` 🚀
```
✨ Created the new tag: v2.4.0 ✨

✅ Done.
```

### Notes

- The manual input prompt will be shown if tags based on semantic versioning does not exist yet.

## Flags and Options
```
FLAGS:
    -d, --debug       Activate debug mode
    -h, --help        Prints help information
        --push-tag    Push tag to remote after tagging
    -q, --quiet       Disable prompt
    -V, --version     Prints version information

OPTIONS:
    -i, --inc-position <inc-position>    Specify increment position
    -p, --prefix <prefix>                Specify tag prefix [default: ]
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
