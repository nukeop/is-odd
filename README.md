# is-odd
Returns true if the given number is odd.

## Install
Specify the dependencty in Cargo.toml:

```yaml
[dependencies]
is-odd = "~1.0.1"
```

Fetch it with cargo:
```bash
$ cargo build
```

## Usage

```rust
extern crate is_odd;
use is_odd::IsOdd;

let _i : i32 = 1;
println!("{}", _i.is_odd()); // prints true
```

## Known tradeoffs
Currently, the library doesn't support floating point numbers.

## About
### License
Copyright © 2018, [nukeop](https://github.com/nukeop).
Released under the [MIT License](LICENSE).
