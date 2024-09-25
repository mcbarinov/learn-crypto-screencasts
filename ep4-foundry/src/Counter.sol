// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Ownable} from "@openzeppelin/access/Ownable.sol";

contract Counter is Ownable(msg.sender) {
    uint256 public value;

    event Inc(address indexed who);
    event OwnerInc(address indexed who);

    constructor(uint256 initValue) {
        value = initValue;
    }

    function inc() external {
        value++;
        emit Inc(msg.sender);
    }

    function ownerInc() external onlyOwner {
        value += 100;
        emit OwnerInc(msg.sender);
    }
}
