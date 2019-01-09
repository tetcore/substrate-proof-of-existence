# substrate-proof-of-existence

A simple [Proof of Existence](https://en.wikipedia.org/wiki/Proof_of_Existence) chain built with [Parity's Substrate](https://github.com/paritytech/substrate).

![Image of UI](./proof-of-existence-ui/poe-ui.png)

1. Start your dev chain
    ```
    cd proof-of-existence
    ./build.sh
    cargo build --release
    ./target/release/proof-of-existence --dev
    ```

2. Start the `proof-of-existence-ui`

    ```
    cd proof-of-existence-ui
    yarn install
    yarn run dev
    ```
