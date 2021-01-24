# nakama-rust-gen

gRPC client code for Nakama Server.

Using [hyperium/tonic](https://github.com/hyperium/tonic/issues?q=import).



# Usage

See example `tests/auth.rs`

```toml
[dependencies]
rust-nakama-gen = "0.1"
tonic = "0.4"
tokio = { version = "1.0.2", features = ["rt-multi-thread", "time", "fs", "macros", "net"] }
```

# Update

Depends on [heroiclabs/nakama](https://github.com/heroiclabs/nakama) as git submoudle.

```sh
git clone https://github.com/harumaxy/nakama-rust-gen.git
git submodule update
cargo build
```

Then, cargo runs build script (`build.rs`) and client code geneated into `src/gen`
