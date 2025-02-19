pragma solidity ^0.8.24;

/// @title A fermah-challenge application using RISC Zero.
/// @notice  Implements a counter that increments every time an odd number is submitted to this contract
interface IOddNumber {
    /// @notice Set the counter stored on the contract. Requires a RISC Zero proof that the number is odd.
    function set(uint256 x, bytes calldata seal) external;

    /// @notice Returns the counter stored.
    function get() external view returns (uint256);
}
