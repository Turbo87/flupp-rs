flupp
==============================================================================

[FluPP] flight log file reader for [Rust]

[FluPP]: http://www.flupp-flightlog.org
[Rust]: https://www.rust-lang.org/


Usage
------------------------------------------------------------------------------

```rust
fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("flugbuch.flu")?;
    let decoded_file: flupp::File = content.parse()?;
    // ...
    Ok(())
}
```


License
------------------------------------------------------------------------------

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)

at your option.
