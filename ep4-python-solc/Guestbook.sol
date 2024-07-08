// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

contract Guestbook {
    event NewMessage(address indexed from, string message);
    event Reset();

    struct Entry {
        address from;
        string message;
    }

    Entry[] public entries;
    address public owner;

    constructor () {
        owner = msg.sender;
    }

    function addMessage(string memory message) public {
        require(bytes(message).length > 0, "Message cannot be empty");
        entries.push(Entry(msg.sender, message));
        emit NewMessage(msg.sender, message);
    }

    function getNumberOfMessages() public view returns (uint256) {
        return entries.length;
    }

    function reset() external {
        require(msg.sender == owner, "Only the owner can reset the guestbook");
        delete entries;
        emit Reset();
    }
}
