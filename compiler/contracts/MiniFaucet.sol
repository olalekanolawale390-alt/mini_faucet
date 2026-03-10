// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

contract MiniFaucet {
    mapping (address claimer => uint256 next_claim_time) public list_of_claimers;

    receive() external payable {}

    function  claim(address _to) public payable {
        uint256 user_next_claim = list_of_claimers[_to];
        if (list_of_claimers[_to] != 0) {
            require(block.timestamp > user_next_claim, "you can only claim once every 24hrs");
        }
        (bool sent,) = _to.call{value: 2e15}("");
        require(sent, "Failed to send Ether");
        list_of_claimers[_to] = block.timestamp + 1 days;
    }

    function deposit () public payable {}

    function faucetBalance() public view returns (uint256) {
        return address(this).balance;
    }

    function getBalance(address recipient) public view returns(uint256) {
        return address(recipient).balance;
    }

    function nextClaimTime(address addy) public view returns (uint256) {
        require(list_of_claimers[addy]!= 0, "try claiming your first token");
        return list_of_claimers[addy];
    }
}
