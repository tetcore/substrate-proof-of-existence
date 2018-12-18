# substrate-proof-of-existence

A simple [Proof of Existence](https://en.wikipedia.org/wiki/Proof_of_Existence) chain built with [Parity's Substrate](https://github.com/paritytech/substrate).

![Image of UI](./proof-of-existence-ui/poe-ui.png)

1. Start your dev chain
    ```
    cd proof-of-existence
    ./target/release/proof-of-existence --dev
    ```

2. Upgrade your runtime

    ```
    ./build.sh
    ```

    Use the [polkadot-ui](https://polkadot.js.org/apps/next) to perform the upgrade:
    ```
    ./runtime/wasm/target/wasm32-unknown-unknown/release/node_runtime.compact.wasm
    ```

3. Start the `proof-of-existence-ui`

    ```
    cd proof-of-existence-ui
    yarn install
    yarn run dev
    ```