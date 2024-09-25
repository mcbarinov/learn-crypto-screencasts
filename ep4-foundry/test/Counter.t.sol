// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {Ownable} from "@openzeppelin/access/Ownable.sol";
import {Counter} from "../src/Counter.sol";

contract CounterTest is Test {
    Counter public counter;

    event Inc(address indexed who);
    event OwnerInc(address indexed who);

    uint256 public INIT_VALUE = 10;

    function setUp() public {
        counter = new Counter(INIT_VALUE);
    }

    function test_inc() public {
        assertEq(counter.value(), INIT_VALUE);

        vm.expectEmit(true, true, true, true);
        emit Inc(address(this));

        counter.inc();
        assertEq(counter.value(), INIT_VALUE + 1);
    }

    function test_ownerInc_ok() public {
        assertEq(counter.value(), INIT_VALUE);

        vm.expectEmit(true, true, true, true);
        emit OwnerInc(address(this));

        counter.ownerInc();
        assertEq(counter.value(), INIT_VALUE + 100);
    }

    function test_ownerInc_onlyOwner() public {
        address alien = address(123);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, alien));
        vm.prank(alien);
        counter.ownerInc();
    }
}
