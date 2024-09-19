Anchor program that lets a user deposit SOL into a vault, the ability to withdraw that SOL from a vault, and close the vault and state.
How to run the code ?
- Requirements:
  - Anchor 0.30.1
  - Solana 1.18.17,
  - yarn 1.22.22 , 
- Solana Local Validator
  - Run `solana-test-validator -r`
- On another terminal, under /anchor-escrow
  - `anchor build`
  - `anchor test --skip-local-validator`
All tests should pass. 

