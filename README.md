# is-odd
Returns true if the given number is odd.

## Install
Specify the dependencty in Cargo.toml:

```toml
[dependencies]
is-odd = "1.1.1"
```

Or:

```bash
$ cargo add is_odd
```

Fetch it with cargo:
```bash
$ cargo build
```

## Usage

```rust
use is_odd::IsOdd;

let _i : i32 = 1;
println!("{}", _i.is_odd()); // prints true
```

## About
Currently, the library support both integer and floating point numbers.

### License
Copyright © 2018, [nukeop](https://github.com/nukeop).
Released under the [MIT License](LICENSE).
