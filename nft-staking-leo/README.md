[NFT Staking with Metaplex Core](https://developers.metaplex.com/core/plugins) An application to help users stake their NFT and earn rewards using the new Metaplex Core Plugin library.

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