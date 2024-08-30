use anchor_lang::prelude::*;
use mpl_core::{
    ID as MPL_CORE_ID,
    fetch_plugin,
    accounts::{BaseAssetV1,BaseCollectionV1},
    types::{Attributes,Attribute,PluginType, Plugin,FreezeDelegate,PluginAuthority},
    instructions::{UpdatePluginV1CpiBuilder,AddPluginV1CpiBuilder,RemovePluginV1CpiBuilder}
};
pub mod errors;
pub use errors::*;
declare_id!("Ho5bb7wSgECskLxURA2FQNjQ7huUHma1CMQb4Hqx1s11");

#[program]
pub mod nft_staking_leo {

    use super::*;

    ///TODO: abstract out
    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        //check if asset has attribute plugin created, if not add it instead of error.
        //it may not be available at init ? 
        match fetch_plugin::<BaseAssetV1,Attributes>(&ctx.accounts.asset.to_account_info(), PluginType::Attributes) {
            Ok((_,fetched_attribute_list, _)) => {
                //check if staked key, if not add it.
                let mut attribute_list:Vec<Attribute> = vec![];
                //if staking attr is initialzed or not
                let mut is_initalized:bool = false;

                for attribute in fetched_attribute_list.attribute_list{
                    if attribute.key=="staked" {
                        require!(attribute.value=="0",StakingError::AlreadyStaked);
                        attribute_list.push(Attribute{
                            key: "staked".to_string(),
                            value: Clock::get()?.unix_timestamp.to_string(),
                        });
                        is_initalized = true;
                    }
                    else {
                        attribute_list.push(attribute);
                    }
                }
                if !is_initalized {
                    attribute_list.push(Attribute{
                        key: "staked".to_string(),
                        value: Clock::get()?.unix_timestamp.to_string(),
                    });
                    attribute_list.push(Attribute{
                        key: "staked_time".to_string(),
                        value: "0".to_string(),
                    });
                }

                UpdatePluginV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
                    .asset(&ctx.accounts.asset.to_account_info())
                    .collection(Some(&ctx.accounts.collection.to_account_info()))
                    .payer(&ctx.accounts.payer.to_account_info())
                    .authority(Some(&ctx.accounts.update_authority.to_account_info()))
                    .system_program(&ctx.accounts.system_program.to_account_info())
                    .plugin(Plugin::Attributes(Attributes{attribute_list}))
                    .invoke()?;
            },
            Err(_)=> {
                let mut attribute_list:Vec<Attribute> = vec![];

                attribute_list.push(Attribute{
                    key: "staked".to_string(),
                    value: Clock::get()?.unix_timestamp.to_string(),
                });
                attribute_list.push(Attribute{
                    key: "staked_time".to_string(),
                    value: "0".to_string(),
                });

                //init_authority could be added ? 
                AddPluginV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
                .asset(&ctx.accounts.asset.to_account_info())
                .collection(Some(&ctx.accounts.collection.to_account_info()))
                .payer(&ctx.accounts.payer.to_account_info())
                .authority(Some(&ctx.accounts.update_authority.to_account_info()))
                .system_program(&ctx.accounts.system_program.to_account_info())
                .plugin(Plugin::Attributes(Attributes{attribute_list}))
                .invoke()?;
            }
        }
        //Adding freeze delegate wether we have plugins or not but after match
        AddPluginV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
        .asset(&ctx.accounts.asset.to_account_info())
        .collection(Some(&ctx.accounts.collection.to_account_info()))
        .payer(&ctx.accounts.payer.to_account_info())
        .authority(Some(&&ctx.accounts.owner.to_account_info()))
        .system_program(&ctx.accounts.system_program.to_account_info())
        .plugin(Plugin::FreezeDelegate(FreezeDelegate{frozen: true}))
        //optional if we dont set it in it will be update auth.
        .init_authority(PluginAuthority::UpdateAuthority)
        .invoke()?;
            Ok(())
    }

    pub fn unstake(ctx: Context<Stake>) -> Result<()> {

        match fetch_plugin::<BaseAssetV1,Attributes>(&ctx.accounts.asset.to_account_info(), PluginType::Attributes) {
            Ok((_, fetched_attribute_list, _)) => {

                let mut attribute_list: Vec<Attribute> = vec![];
                let is_initialized = false;
                let mut staked_time: i64 = 0;

                for attribute in fetched_attribute_list.attribute_list {
                    if attribute.key == "staked"{
                        require!(attribute.value!="0", StakingError::NotStaked);
                        attribute_list.push(Attribute {
                            key: "staked".to_string(),
                            value: "0".to_string(),
                        });
                        staked_time = staked_time
                            .checked_add(Clock::get()?.unix_timestamp)
                            .ok_or(StakingError::Overflow)?
                            .checked_sub(
                                attribute.value.parse::<i64>().map_err(|_|StakingError::InvalidTimeStamp)?
                            )
                            .ok_or(StakingError::Underflow)?;
                    }
                    else if attribute.key == "staked_time" {
                        staked_time = staked_time 
                            .checked_add(attribute.value.parse::<i64>()
                                .map_err(|_| StakingError::InvalidTimeStamp)?
                            )
                            .ok_or(StakingError::Overflow)?;
                        attribute_list.push(Attribute {
                            key: "staked_time".to_string(),
                            value: staked_time.to_string() 
                        });  
                    }
                    else {
                        attribute_list.push(attribute);
                    }

                }
                require!(is_initialized, StakingError::NotStaked);

                UpdatePluginV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
                .asset(&ctx.accounts.asset.to_account_info())
                .collection(Some(&ctx.accounts.collection.to_account_info()))
                .payer(&ctx.accounts.payer.to_account_info())
                .authority(Some(&ctx.accounts.update_authority.to_account_info()))
                .system_program(&ctx.accounts.system_program.to_account_info())
                .plugin(Plugin::Attributes(Attributes{attribute_list}))
                .invoke()?;

            }
            Err(_) => {
                return Err(StakingError::AttributesNotInitialized.into());
            }
        }

        UpdatePluginV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
        .asset(&ctx.accounts.asset.to_account_info())
        .collection(Some(&ctx.accounts.collection.to_account_info()))
        .payer(&ctx.accounts.payer.to_account_info())
        .authority(Some(&ctx.accounts.update_authority.to_account_info()))
        .system_program(&ctx.accounts.system_program.to_account_info())
        .plugin(Plugin::FreezeDelegate(FreezeDelegate{frozen: false}))
        .invoke()?;

        RemovePluginV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
        .asset(&ctx.accounts.asset.to_account_info())
        .collection(Some(&ctx.accounts.collection.to_account_info()))
        .payer(&ctx.accounts.payer.to_account_info())
        .authority(Some(&ctx.accounts.owner.to_account_info()))
        .system_program(&ctx.accounts.system_program.to_account_info())
        .plugin_type(PluginType::FreezeDelegate)
        .invoke()?;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Stake<'info> {
    pub owner: Signer<'info>,
    pub update_authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        has_one = owner
    )]
    pub asset: Account<'info,BaseAssetV1>,
    #[account(
        mut,
        has_one = update_authority
    )]
    //collection has update authority ?
    pub collection: Account<'info, BaseCollectionV1>,
    #[account( address = MPL_CORE_ID)]
    ///CHECK: This is safe because of address constraint above
    pub mpl_core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
