# rust-nakama-gen

Generate gRPC client code for Nakama Server.

Using [hyperium/tonic] (https://github.com/hyperium/tonic/issues?q=import).


# Usage

Depend on [heroiclabs/nakama](https://github.com/heroiclabs/nakama) as git submoudle.

```sh
git clone https://github.com/harumaxy/nakama-rust-gen.git
git submodule update
cargo build
```

Then, cargo runs build script (`build.rs`) and client code in geneated `src/gen`