pragma solidity ^0.8.24;

import {Script} from "forge-std/Script.sol";
import "forge-std/Test.sol";
import {RiscZeroCheats} from "risc0/test/RiscZeroCheats.sol";
import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {RiscZeroGroth16Verifier} from "risc0/groth16/RiscZeroGroth16Verifier.sol";
import {ControlID} from "risc0/groth16/ControlID.sol";

import {OddNumber} from "../contracts/OddNumber.sol";

/// @notice Deployment script for the RISC Zero starter project.
/// @dev Use the following environment variable to control the deployment:
///     * Set one of these two environment variables to control the deployment wallet:
///         * ETH_WALLET_PRIVATE_KEY private key of the wallet account.
///         * ETH_WALLET_ADDRESS address of the wallet account.

contract OddNumberDeploy is Script, RiscZeroCheats {
    // Path to deployment config file, relative to the project root.
    string constant CONFIG_FILE = "script/config.toml";

    IRiscZeroVerifier verifier;

    function run() external {
        // Read and log the chainID
        uint256 chainId = block.chainid;
        console2.log("You are deploying on ChainID %d", chainId);

        // Read the config profile from the environment variable, or use the default for the chainId.
        // Default is the first profile with a matching chainId field.
        string memory config = vm.readFile(string.concat(vm.projectRoot(), "/", CONFIG_FILE));
        string memory configProfile = vm.envOr("CONFIG_PROFILE", string(""));
        if (bytes(configProfile).length == 0) {
            string[] memory profileKeys = vm.parseTomlKeys(config, ".profile");
            for (uint256 i = 0; i < profileKeys.length; i++) {
                if (stdToml.readUint(config, string.concat(".profile.", profileKeys[i], ".chainId")) == chainId) {
                    configProfile = profileKeys[i];
                    break;
                }
            }
        }

        if (bytes(configProfile).length != 0) {
            console2.log("Deploying using config profile:", configProfile);
            string memory configProfileKey = string.concat(".profile.", configProfile);
            address riscZeroVerifierAddress =
                stdToml.readAddress(config, string.concat(configProfileKey, ".riscZeroVerifierAddress"));
            // If set, use the predeployed verifier address found in the config.
            verifier = IRiscZeroVerifier(riscZeroVerifierAddress);
        }

        // Determine the wallet to send transactions from.
        uint256 deployerKey = uint256(vm.envOr("ETH_WALLET_PRIVATE_KEY", bytes32(0)));
        address deployerAddr = address(0);
        if (deployerKey != 0) {
            // Check for conflicts in how the two environment variables are set.
            address envAddr = vm.envOr("ETH_WALLET_ADDRESS", address(0));
            require(
                envAddr == address(0) || envAddr == vm.addr(deployerKey),
                "conflicting settings from ETH_WALLET_PRIVATE_KEY and ETH_WALLET_ADDRESS"
            );

            vm.startBroadcast(deployerKey);
        } else {
            deployerAddr = vm.envAddress("ETH_WALLET_ADDRESS");
            vm.startBroadcast(deployerAddr);
        }

        // Deploy the verifier, if not already deployed.
        if (address(verifier) == address(0)) {
            verifier = deployRiscZeroVerifier();
        } else {
            console2.log("Using IRiscZeroVerifier contract deployed at", address(verifier));
        }

        // Deploy the application contract.
        OddNumber oddNumber = new OddNumber(verifier);
        console2.log("Deployed OddNumber to", address(oddNumber));

        vm.stopBroadcast();
    }
}
