# Home Assignment


## Overview

The assignment received listed the need to create system consisting of an on-chain smart contract and an off-chain component that demonstrates zero-knowledge proof generation and  verification using RISC Zero. The system should showcase the ability to work with both [Rust][install-rust] and [Solidity][solidity-org] while implementing modern blockchain development practices.


### Technical Assessment

After reviewing and validating proposed documentation, the approach followed to complete the assignment was to generate the solution by levering the [RISC Zero Foundry Template][risc0-foundry-tpl].

This template provides a starting point for building powerful new applications on Ethereum that offload work that is computationally intensive (i.e. gas expensive), or difficult to implement in Solidity (e.g. ed25519 signature verification, or HTML parsing).

In order to build the solution using the RISC Zero Foundry Template, changes were made in three main areas:

- ***Guest Code***: The piece of code developed to validate if a given number is **odd** and generate a proof of this computation was added under the [methods/guest](./methods/guest/) folder. This code runs off-chain within the RISC Zero zkVM and performs the actual computations.
- ***Smart Contracts***: The on-chain part of the solution is located in the [contracts](./contracts/) folder. The smart contract verifies zkVM proofs and updates the blockchain state based on the results of off-chain computations. For instance, in the [OddNumber](./contracts/OddNumber.sol) scenario, the smart contract verifies a proof that a number is odd and implements a counter that increments every time an odd number is submitted.
- ***Publisher Application***: This is the main orchestration app and is located in the [apps](./apps) folder. The **publisher** application bridges off-chain computation with on-chain verification by submitting proof requests, receiving proofs, and publishing them to the smart contract on Ethereum. All on-chain interactions are excuted by using [Alloy][alloy-crate] crate.


## Project Dependencies

First, [install Rust][install-rust] and [Foundry][install-foundry], and then restart your terminal.

```sh
# Install Rust
curl https://sh.rustup.rs -sSf | sh
# Install Foundry
curl -L https://foundry.paradigm.xyz | bash
```

Next, you will use `rzup` to install `cargo-risczero`.

To install `rzup`, run the following command and follow the instructions:

```sh
curl -L https://risczero.com/install | bash
```

This installation will work for x86-64 Linux and arm64 macOS. In case you have a different target, you will need to progress with the manual installation.
Please read [RISC Zero Installation steps][risc0-install]

Next we can install the RISC Zero toolchain by running `rzup`:

```sh
rzup install
```

You can verify the installation was successful by running:

```sh
cargo risczero --version
```

### Build the Code

- Update git submodules.

  ```sh
  git submodule update --init
  ```

- Builds for zkVM program, the publisher app, and any other Rust code.

  ```sh
  cargo build
  ```

  > NOTE: In case you may find building issues with the `stable` version for Rust and need to re-build std library, you can update the [rust-toolchain.toml][rust-toolchain] and add `nightly` like this:

  ```sh
  [toolchain]
  channel = "nightly"
  ```

    > and use the -Z option - It is also recommended to target the specific version of the system - e.g.:

    ```sh 
      cargo build -Zbuild-std --target x86_64-unknown-linux-gnu
    ```

- Build Solidity smart contracts.

  > NOTE: `cargo build` needs to run first to generate the `ImageID.sol` contract.

  ```sh
  forge build
  ```

### Run the Tests

- Tests zkVM program.

  ```sh
  cargo test
  ```

- Test Solidity contracts, integrated with zkVM program.

  ```sh
  RISC0_DEV_MODE=true forge test -vvv 
  ```

- Run the same tests, with the full zkVM prover rather than dev-mode, by setting `RISC0_DEV_MODE=false`.

  ```sh
  RISC0_DEV_MODE=false forge test -vvv
  ```

  Producing the [Groth16 SNARK proofs][groth16] for this test requires running on an x86 machine with [Docker][install-docker] installed, or using [Bonsai](#configuring-bonsai).
  Apple silicon is currently unsupported for local proving, you can find out more info in the relevant issues [here](https://github.com/risc0/risc0/issues/1520) and [here](https://github.com/risc0/risc0/issues/1749).

## Application deployment

Please find here [deployment guide](./deployment-guide.md) to get the application running Locally (w/ Anvil), [Sepolia][sepolia] or Ethereum Mainnet.

## Project Structure

Below are the primary files in the project directory

```text
.
├──.github/workflows                // Proposed github.workflow for automated build and test
├── Cargo.toml                      // Configuration for Cargo and Rust
├── foundry.toml                    // Configuration for Foundry
├── apps
│   ├── Cargo.toml
│   └── src
│       └── bin                     
│           └── publisher.rs        // App to publish the received proofs directly to the deployed app contract
│           └── trace_logs
│               └── mod.rs          // Tracing logs functionality
├── contracts
│   ├── OddNumber.sol               // Contract that implements a counter that increments every time an odd number is submitted
│   └── ImageID.sol                 // Generated contract with the image ID for your zkVM program
├── methods
│   ├── Cargo.toml
│   ├── guest
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── bin                 
│   │           └── is_odd.rs       // Guest program for checking if a number is odd
│   └── src
│       └── lib.rs                  // Compiled image IDs and tests of guest program (is_odd)
└── tests
    ├── OddNumber.t.sol             // Tests for the OddNumber contract
    └── Elf.sol                     // Generated contract with paths the guest program ELF files.
```

[docs-bonsai]: https://dev.risczero.com/api/generating-proofs/remote-proving
[install-foundry]: https://getfoundry.sh/
[install-docker]: https://docs.docker.com/get-docker/
[groth16]: https://www.risczero.com/news/on-chain-verification
[docs-verifier]: https://dev.risczero.com/api/blockchain-integration/contracts/verifier
[docs-zkvm]: https://dev.risczero.com/zkvm
[homepage-risczero]: https://www.risczero.com/
[Sepolia]: https://www.alchemy.com/overviews/sepolia-testnet
[blog-coprocessor]: https://www.risczero.com/news/a-guide-to-zk-coprocessors-for-scalability
[faq]: https://dev.risczero.com/faq#zkvm-application-design
[image-id]: https://dev.risczero.com/terminology#image-id
[install-rust]: https://doc.rust-lang.org/cargo/getting-started/installation.html
[term-journal]: https://dev.risczero.com/terminology#journal
[steel-repo]: https://github.com/risc0/risc0-ethereum/tree/main/crates/steel
[erc20-counter]: https://github.com/risc0/risc0-ethereum/tree/main/examples/erc20-counter
[solidity-org]: https://soliditylang.org/
[risc0-foundry-tpl]: https://github.com/risc0/risc0-foundry-template
[risc0-install]:https://dev.risczero.com/api/zkvm/install
[rust-toolchain]: https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file
[alloy-crate]: https://crates.io/crates/alloy
