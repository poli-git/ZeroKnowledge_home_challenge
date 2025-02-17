pragma solidity ^0.8.24;

import {RiscZeroCheats} from "risc0/test/RiscZeroCheats.sol";
import {console2} from "forge-std/console2.sol";
import {Test} from "forge-std/Test.sol";
import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {OddNumber} from "../contracts/OddNumber.sol";
import {Elf} from "./Elf.sol"; // auto-generated contract after running `cargo build`.

contract OddNumberTest is RiscZeroCheats, Test {
    OddNumber public oddNumber;

    function setUp() public {
        IRiscZeroVerifier verifier = deployRiscZeroVerifier();
        oddNumber = new OddNumber(verifier);
        assertEq(oddNumber.get(), 0);
    }

    function test_SetOdd() public {
        uint256 number = 1311;
        (bytes memory journal, bytes memory seal) = prove(Elf.IS_ODD_PATH, abi.encode(number));

        oddNumber.set(abi.decode(journal, (uint256)), seal);
        assertEq(oddNumber.get(), number);
    }

    function test_SetEven() public {
        uint256 number = 12;
        (bytes memory journal, bytes memory seal) = prove(Elf.IS_ODD_PATH, abi.encode(number));

        oddNumber.set(abi.decode(journal, (uint256)), seal);
        !assertEq(oddNumber.get(), number);
    }
}
