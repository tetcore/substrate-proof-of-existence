# Proof of Existence Example

Here are the steps so you can run and test the Proof of Existence runtime module:

1. Start your dev chain
    ```
    cd proof-of-existence-node
    ./scripts/build.sh
    cargo build --release
    ./target/release/proof-of-existence --dev
    ```

2. Start the `proof-of-existence-ui` (Requires NodeJS version 11)

    ```
    cd proof-of-existence-ui
    yarn install
    yarn run dev
    ```

3. Navigate to http://localhost:8000/ and play!