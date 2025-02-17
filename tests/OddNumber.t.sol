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

        uint256 previous_count = oddNumber.get();

        oddNumber.set(abi.decode(journal, (uint256)), seal);
        
        // check that the counter was incremented
        assert(oddNumber.get() == previous_count + 1);    
    }

    function test_SetOdd_Error() public {
        uint256 number = 1312;
        (bytes memory journal, bytes memory seal) = prove(Elf.IS_ODD_PATH, abi.encode(number));

        uint256 previous_count = oddNumber.get();

        oddNumber.set(abi.decode(journal, (uint256)), seal);
        
        // check that the counter was incremented
        assert(oddNumber.get() == previous_count);    
    }
}
