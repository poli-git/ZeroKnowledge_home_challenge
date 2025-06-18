# Challenge: Solidity Contracts

This directory contains the Solidity contracts for an application with [RISC Zero] on Ethereum.

The contract included for this challenge is [`OddNumber.sol`](./OddNumber.sol). 
Implements a counter that increments every time an odd number is submitted to this contract.


Contracts are built and tested with [forge], which is part of the [Foundry] toolkit.
Tests are defined in the `tests` directory.

 
## Generated Contracts

As part of the build process, the following contracts are generated: `ImageID.sol` and `Elf.sol`.
Running `cargo build` will generate these contracts with up to date references to the guest code.


- `ImageID.sol`: contains the [Image IDs][image-id] for the guests implemented in the [methods] directory.
- `Elf.sol`: contains the path of the guest binaries implemented in the [methods] directory. This contract is saved in the `tests` directory.

[Foundry]: https://getfoundry.sh/
[RISC Zero]: https://risczero.com
[forge]: https://github.com/foundry-rs/foundry#forge
[github.com/risc0/risc0-ethereum]: https://github.com/risc0/risc0-ethereum/tree/main/contracts
[image-id]: https://dev.risczero.com/terminology#image-id
[methods]: ../methods/README.md
