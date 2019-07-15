## tag-guard
[![Build Status](https://travis-ci.org/Nu-SCPTheme/tag-guard.svg?branch=master)](https://travis-ci.org/Nu-SCPTheme/tag-guard)

A Rust library to enforce configured relationships between tags. Ensures consistency between all tagged objects with respect to user-specified rules.

Available under the terms of the MIT License. See [LICENSE.md](LICENSE).

### Compilation
This library targets the latest stable Rust. At time of writing, that is 1.35.0

```sh
$ cargo build --release
```

### Testing
```sh
$ cargo test
```

Add `-- --nocapture` to the end if you want to see test output.
