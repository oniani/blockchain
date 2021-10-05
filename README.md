# blockchain

A Blockchain implementation in pure Rust.

## API

Below find an example usage of the library:

```rust
use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new(20);
    for data in ["what", "a", "wonderful", "world"] {
        blockchain.add_data(data.as_bytes().to_vec());
    }
    blockchain.mine();
}
```

## References

- [Blockchain](https://en.wikipedia.org/wiki/Blockchain)

## License

[MIT License](LICENSE)
