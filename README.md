# Zencryption
 
Command line tool for file encryption coded in Rust!

> Note: This repo is not intended for use in the real world...this was a pet project and it is vulnerable to timing attacks (among other attack vectors)

## Why Rust
From [The Book](https://doc.rust-lang.org/book/2018-edition/ch12-00-an-io-project.html): "Rust's speed, safety, single binary output and cross-platform support make it an ideal langtuage for creating command line tools".

## File Encryption Algorithm

We are using asymmetric encryption. This requires the following syntax from the command line.

```
cargo run <command> <filename> <secret_key>
```

We utilize the [symmetriccipher.rs](https://docs.rs/crate/rust-crypto/0.2.36/source/examples/symmetriccipher.rs) example from the ```Rust-Crypto``` crate. At a high level, we take the following steps:

**To encrypt:**
1. Generate an ephemeral secret key (a curve25519 point)
2. Derives a public key from the secret key (curve25519 multiplication)
3. Derives a symmetric key by combining the permanent public key with the ephemeral secret key (curve25519 multiplication)
4. Symmetrically encrypts/authenticates the text using chacha20-poly1305
5. Packages the ephemeral public key with the encrypted/integrity-checked text.

**To decrypt:**
1. Derive a symmetric key by combining the ephemeral public key with the permanent secret key (curve25519 multiplication)
2. Decrypts/integrity-checks the text with chacha20-poly1305