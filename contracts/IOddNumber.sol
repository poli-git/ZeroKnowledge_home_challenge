pragma solidity ^0.8.24;

/// @title A starter application using RISC Zero.
/// @notice This basic application holds a number, guaranteed to be odd.
/// @dev This contract demonstrates one pattern for offloading the computation of an expensive
///      or difficult to implement function to a RISC Zero guest running on the zkVM.
interface IOddNumber {
    /// @notice Set the odd number stored on the contract. Requires a RISC Zero proof that the number is odd.
    function set(uint256 x, bytes calldata seal) external;

    /// @notice Returns the number stored.
    function get() external view returns (uint256);
}
