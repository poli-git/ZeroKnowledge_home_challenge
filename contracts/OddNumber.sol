pragma solidity ^0.8.24;

import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {ImageID} from "./ImageID.sol"; // auto-generated contract after running `cargo build`.

/// @title A starter application using RISC Zero.
/// @notice This basic application holds a number, guaranteed to be odd.
/// @dev This contract demonstrates one pattern for offloading the computation of an expensive
///      or difficult to implement function to a RISC Zero guest running on the zkVM.
contract OddNumber {
    /// @notice RISC Zero verifier contract address.
    IRiscZeroVerifier public immutable verifier;
    /// @notice Image ID of the only zkVM binary to accept verification from.
    ///         The image ID is similar to the address of a smart contract.
    ///         It uniquely represents the logic of that guest program,
    ///         ensuring that only proofs generated from a pre-defined guest program
    ///         (in this case, checking if a number is odd) are considered valid.
    bytes32 public constant imageId = ImageID.IS_ODD_ID;

    /// @notice A number that is guaranteed, by the RISC Zero zkVM, to be odd.
    ///         It can be set by calling the `set` function.
    uint256 public counter;

    /// @notice Initialize the contract, binding it to a specified RISC Zero verifier.
    constructor(IRiscZeroVerifier _verifier) {
        verifier = _verifier;
        counter = 0;
    }

    /// @notice Set the odd number stored on the contract. Requires a RISC Zero proof that the number is odd.
    function set(uint256 x, bytes calldata seal) public {
        // Construct the expected journal data.
        // Verify will fail if journal does not match.
        bytes memory journal = abi.encode(x);
        verifier.verify(seal, imageId, sha256(journal));

        counter += 1;
    }

    /// @notice Returns the number stored.
    function get() public view returns (uint256) {
        return counter;
    }
}
