# easyduration

Rust library that simplifies construction of a `std::time::Duration` with the
`easyduration::EasyDuration` extension trait.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
easyduration = "1.0.0"
```

```rust
use easyduration::EasyDuration;

fn main() {
    let v = 1.seconds();
    let v = 12.minutes();
    let v = 5.hours();
}
```

## License

Licensed under:

* [MIT](http://opensource.org/licenses/MIT)
  See [LICENSE](./LICENSE)
