# MINI SEPOLIA FAUCET

This project is solely built to demonstrate handling of errors and detecting errors instead of using unsafe ways like unwrap and expect...also using rate limit how the basics of alloy-rs works(deploying of contract and interacting with it)

### /api/claim/YOUR-ADDRESS - use this endpoint to request for faucet every 24hrs 
### /api/faucet-address - Use this endpoint to check faucet address if you will like to contribute
### /api/faucet-balance - Use this endpoint to check if there's enough token in the faucet
### /api/my-balance/YOUR-ADDRESS - Use this address to check your own balance
### /api/next-claim-time/YOUR-ADDRESS - Use this endpoint to check your next available claim time(you can only claim once every 24hrs)