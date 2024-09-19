A non automated market making (escrow) application that settles transfer of spl tokens bettwen 2 parties.

How to run the code ?
- Requirements:
  - Anchor 0.30.1
  - Solana 1.18.17,
  - yarn 1.22.22 , 
Solana Validator
  - Run `solana-test-validator -r`
  - On another terminal, under /anchor-escrow
  - `anchor build`
  - `anchor test --skip-local-validator`
All tests should pass. 
