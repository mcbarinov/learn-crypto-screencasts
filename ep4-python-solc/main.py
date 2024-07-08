import solcx

CONTRACT_SOURCE = """
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

"""

# Install a specific version of the Solidity compiler
solcx.install_solc('0.8.26')

# Compile from a source string
res = solcx.compile_source(CONTRACT_SOURCE)
res = res["<stdin>:Guestbook"]
print(res["abi"])
# [{'inputs': [], 'stateMutability': 'nonpayable', 'type'...
print(res["bin"])
# 6080604052348015600e575f80fd5b503360015f6101000a815481...


# Compile from a file
res = solcx.compile_files(["Guestbook.sol"], solc_version="0.8.26", output_values=["abi", "bin"])
res = res["Guestbook.sol:Guestbook"]
print("\n\n")
print(res)
