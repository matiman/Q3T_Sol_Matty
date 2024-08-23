use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, metadata::{mpl_token_metadata::instructions::{FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts}, MasterEditionAccount, Metadata, MetadataAccount}, token::{approve, Approve}, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::state::{stake_account::StakeAccount, stake_config::StakeConfig, user_account::UserAccount};

#[derive(Accounts)]
pub struct Stake<'info>{
    //the user who is staking
    #[account(mut)]
    pub user: Signer<'info>,

    //the NFT to stake
    pub mint: InterfaceAccount<'info, Mint>,

    //to verify if mint is in collection and is verified.
    pub collection: InterfaceAccount<'info, Mint>,

    //mint ata of the user where the NFT is 
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    pub mint_ata_user: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    //TODO config to check against and that is why its not mutable and no constraints?? 
    pub config: Account<'info,StakeConfig>,

    //user + stake acct
    #[account(
        init,
        payer = user,
        space = StakeAccount::INIT_SPACE,
        //using user_aacount in seeds instead of config. Andre did config instead.
        seeds = [b"stake",mint.key().as_ref(),user_account.key().as_ref()],
        bump,
    )]
    pub stake_acct: Account<'info, StakeAccount>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref()
        ],
        //overriding derivation of seeds for this acct.
        //seed is drived from metadata program id not our program id.
        seeds::program = metadata_program.key(),
        bump,
        //one uses key and another one calls key()
        //The difference is the former is a field and the latter is a function call 
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection.key().as_ref(),
        //as per LEO: you can’t set it to true by default if the authority over the CreateV1 isn’t the collection_authority. 
        //Same for an NFT already created, 
        //VerifyV1 needs to be signed by the collection authority that switches the verified flag from false to true
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

    //to create mint ata
    pub associated_token_program: Program<'info,AssociatedToken>,

    //create new accounts for init and init_if_needed
    pub system_program: Program<'info, System>,

}

impl<'info> Stake <'info> {
    pub fn stake(&mut self, bumps: &StakeBumps) -> Result<()>{

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Approve {
            delegate: self.mint_ata_user.to_account_info(),
            to: self.stake_acct.to_account_info(),
            authority: self.user.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        //approving only 1 NFT. Anyways its edition so only exists.
        approve(cpi_ctx, 1)?;

        let delegate = &self.stake_acct.to_account_info();
        let token_account = &self.mint_ata_user.to_account_info();
        let edition = &self.edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();
        let token_program = &self.token_program.to_account_info();

        FreezeDelegatedAccountCpi::new(metadata_program, 
            FreezeDelegatedAccountCpiAccounts {
                delegate,
                token_account,
                edition,
                mint,
                token_program
            }
        ).invoke()?;

        self.stake_acct.set_inner(StakeAccount{
            owner: self.user.key(),
            mint: self.mint.key(),
            //can also consider slot instead of tiem stamp
            //when testing keep in mind this is in seconds and JS is in milli seconds.
            last_update: Clock::get()?.unix_timestamp,
            bump: bumps.stake_acct
        });

        self.user_account.amount_staked += 1;

        Ok(())
    }
}