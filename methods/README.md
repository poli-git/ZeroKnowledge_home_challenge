# Challenge:  zkVM Methods

 
This directory contains the [zkVM] portion of the [RISC Zero] application.

The `is_odd` [guest program] has been defined to act as a coprocessor to the [on-chain logic].

 

## From Guest Code to Binary File

Code in the `methods/guest` directory will be compiled into one or more binaries.

Build configuration for the methods is included in `methods/build.rs`. Each will have a corresponding image ID, which is a hash identifying the program.

[zkVM]: https://dev.risczero.com/zkvm
[RISC Zero]: https://www.risczero.com/
[guest programs]: https://dev.risczero.com/terminology#guest-program
[on-chain logic]: ../contracts/
[guest/src/bin]: ./guest/src/bin/
[zkvm-hello-world]: https://dev.risczero.com/api/zkvm/tutorials/hello-world
[RISC Zero examples]: https://github.com/risc0/risc0/tree/release-1.1/examples
