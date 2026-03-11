use crate::blockchain::imports::*;

sol! { 
    #[derive(Debug)]
    #[sol(rpc)] 
    contract MiniFaucet { 
        function deposit () public payable;
        function  claim(address _to) public payable;
        function faucetBalance() public view returns (uint256);
        function getBalance(address recipient) public view returns(uint256);
        function nextClaimTime(address addy) public view returns (uint256);
    } 
}
