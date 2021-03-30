To run the examples, you need [`wasm-bindgen-cli`](https://github.com/rustwasm/wasm-bindgen/) and [`trunk`](https://github.com/thedodd/trunk).

```
cargo install trunk wasm-bindgen-cli
```

Then move into a directory and run it:

```
cd appbar
trunk serve --release
```

If the `wasm-bindgen` version mismatch, please ensure that you are using the latest version of `wasm-bindgen`, and then remove the `Cargo.lock` file and retry.