use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, 
    metadata::{mpl_token_metadata::instructions::{FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts},
     MasterEditionAccount, Metadata, MetadataAccount}, 
     token::{approve, Approve}, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{errors::StakingError, state::{stake_account::StakeAccount, stake_config::StakeConfig, user_account::UserAccount}};

#[derive(Accounts)]
pub struct Stake<'info>{
    //the user who is staking
    #[account(mut)]
    pub user: Signer<'info>,

    //the NFT the user is about to stake
    pub mint: InterfaceAccount<'info, Mint>,

    //mint ata of the user where the NFT is 
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    pub mint_ata_user: InterfaceAccount<'info, TokenAccount>,

    //to verify if mint is in collection and is verified.
    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump
    )]
    //used here to update the amount staked for the user.
    pub user_account: Account<'info, UserAccount>,

    //may be a config to check against and that is why its not mutable and no constraints?? 
    #[account(
        seeds = [b"config".as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info,StakeConfig>,

    //user + stake acct
    #[account(
        init,
        payer = user,
        space = StakeAccount::INIT_SPACE,
        seeds = [b"stake".as_ref(),mint.key().as_ref(),config.key().as_ref()],
        bump,
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            //check if metadata belongs to the NFT being staked
            mint.key().as_ref()
        ],
        //overriding derivation of seeds for this acct.
        //seed is drived from metadata program id not our program id.
        seeds::program = metadata_program.key(),
        bump,
        //one uses key and another one calls key()
        //The difference is the former is a field and the latter is a function call
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        //as per LEO: you can’t set it to true by default 
        //if the authority over the CreateV1 isn’t the collection_authority. 
        //Same for an NFT already created, 
        //VerifyV1 needs to be signed by the collection authority that switches the verified flag from false to true
        //I believe the metadata mgmt is centralized may be by Tensor and other market places
        //who add NFTs to collections based on request. You have to fill a form.
        constraint = metadata.collection.as_ref().unwrap().verified == true
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump
    )]
    //to make sure only one token in circulation
    pub edition: Account<'info, MasterEditionAccount>,

    //metadata program we used to interact with Metaplex programs?
    metadata_program: Program<'info, Metadata>,
    
    //to delegate authority from user to stake acct ?
    pub token_program: Interface<'info, TokenInterface>,

    //to get ata address for mint_ata_user 
    pub associated_token_program: Program<'info,AssociatedToken>,

    //create new accounts for init and init_if_needed
    pub system_program: Program<'info, System>,

}

impl<'info> Stake <'info> {

    pub fn stake(&mut self, bumps: &StakeBumps) -> Result<()>{

        require!(self.user_account.amount_staked < self.config.max_stake, StakingError::MaxStakeLimitReached);

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Approve {
            delegate: self.mint_ata_user.to_account_info(),
            to: self.stake_account.to_account_info(),
            authority: self.user.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        //approving only 1 NFT. Anyways its edition so only 1 exists.
        approve(cpi_ctx, 1)?;

        //to whom we are delegating the NFT staking
        let delegate = &self.stake_account.to_account_info();
        // Token account to freeze
        let token_account = &self.mint_ata_user.to_account_info();
        let edition = &self.edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();
        let token_program = &self.token_program.to_account_info();

        let seeds = &[
            b"stake",
            self.mint.to_account_info().key.as_ref(),
            self.config.to_account_info().key.as_ref(),
            &[self.stake_account.bump]
        ];

        let signer_seeds =&[&seeds[..]];

        //delegate authority of the NFT to the stake_account
        FreezeDelegatedAccountCpi::new(metadata_program, 
            FreezeDelegatedAccountCpiAccounts {
                delegate,
                token_account,
                edition,
                mint,
                token_program
            }
        ).invoke_signed(signer_seeds)?;

        self.stake_account.set_inner(StakeAccount{
            owner: self.user.key(),
            mint: self.mint.key(),
            //can also consider slot instead of tiem stamp
            //when testing keep in mind this is in seconds and JS is in milli seconds.
            staked_at: Clock::get()?.unix_timestamp,
            bump: bumps.stake_account
        });

        //increment total amt staked
        self.user_account.amount_staked += 1;

        Ok(())
    }
}