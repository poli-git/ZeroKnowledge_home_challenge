# Fermah - Challenge: Integration/Orchestration App

An off-chain intgration applications is required to perform two main actions:

* Produce a proof e.g. by sending a proof request to [Bonsai].
* Send a transaction to Ethereum to execute your on-chain logic.

## Publisher

The [`publisher` CLI][publisher], is an application that sends an off-chain proof request to the [Bonsai] proving service, and publishes the received proofs to your deployed app contract.

### Usage

Run the `publisher` with:

```sh
cargo run --bin publisher
```

```text
$ cargo run --bin publisher -- --help

Usage: publisher --chain-id <CHAIN_ID> --eth-wallet-private-key <ETH_WALLET_PRIVATE_KEY> --rpc-url <RPC_URL> --contract <CONTRACT> --input <INPUT>

Options:
      --chain-id <CHAIN_ID>
          Ethereum chain ID
      --eth-wallet-private-key <ETH_WALLET_PRIVATE_KEY>
          Ethereum Node endpoint [env: ETH_WALLET_PRIVATE_KEY=]
      --rpc-url <RPC_URL>
          Ethereum Node endpoint
      --contract <CONTRACT>
          Application's contract address on Ethereum
  -i, --input <INPUT>
          The input to provide to the guest binary
  -h, --help
          Print help
  -V, --version
          Print version
```

[publisher]: ./src/bin/publisher.rs
[Bonsai]: https://dev.bonsai.xyz/
