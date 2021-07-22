use anchor_lang::prelude::*;

#[program]
pub mod iot {
    use super::*;

    pub fn create_iot_source(ctx: Context<CreateIotSource>, name: String) -> Result<()> {
        ctx.accounts.iot_source.name = name;
        ctx.accounts.iot_source.authority = *ctx.accounts.authority.key;
        Ok(())
    }
    pub fn create_iot_data_store(ctx: Context<CreateIotDataStore>, name: String) -> Result<()> {
        let given_name = name.as_bytes();
        let mut name = [0u8; 280];
        name[..given_name.len()].copy_from_slice(given_name);
        let mut iot_data = ctx.accounts.iot_data_store.load_init()?;
        iot_data.name = name;
        Ok(())
    }
    pub fn send_iot_data(ctx: Context<SendIotData>, msg: String) -> Result<()> {
        let mut iot_data = ctx.accounts.iot_data_store.load_mut()?;
        iot_data.append({
            let src = msg.as_bytes();
            let mut data = [0u8; 280];
            data[..src.len()].copy_from_slice(src);
            IotData {
                from: *ctx.accounts.iot_source.to_account_info().key,
                data,
            }
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateIotSource<'info> {
    #[account(init, associated = authority, space = 312)]
    iot_source: ProgramAccount<'info, IotSource>,
    #[account(signer)]
    authority: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
    system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CreateIotDataStore<'info> {
    #[account(init)]
    iot_data_store: Loader<'info, IotDataStore>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SendIotData<'info> {
    #[account(associated = authority, has_one = authority)]
    iot_source: ProgramAccount<'info, IotSource>,
    #[account(signer)]
    authority: AccountInfo<'info>,
    #[account(mut)]
    iot_data_store: Loader<'info, IotDataStore>,
}

#[associated]
pub struct IotSource {
    name: String,
    authority: Pubkey,
}

#[account(zero_copy)]
pub struct IotDataStore {
    head: u64,
    tail: u64,
    name: [u8; 280],            // Human readable name (char bytes).
    iot_data_feed: [IotData; 33607], // Leaves the account at 10,485,680 bytes.
}

impl IotDataStore {
    fn append(&mut self, msg: IotData) {
        self.iot_data_feed[IotDataStore::index_of(self.head)] = msg;
        if IotDataStore::index_of(self.head + 1) == IotDataStore::index_of(self.tail) {
            self.tail += 1;
        }
        self.head += 1;
    }
    fn index_of(counter: u64) -> usize {
        std::convert::TryInto::try_into(counter % 33607).unwrap()
    }
}

#[zero_copy]
pub struct IotData {
    pub from: Pubkey,
    pub data: [u8; 280],
}

#[error]
pub enum ErrorCode {
    Unknown,
}

/*
#[program]
pub mod iot {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>, data: u64) -> ProgramResult {
        let iot_account = &mut ctx.accounts.iot_account;
        iot_account.data = data;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: u64) -> ProgramResult {
        let iot_account = &mut ctx.accounts.iot_account;
        iot_account.data = data;
        Ok(())
    }    
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init)]
    pub iot_account: ProgramAccount<'info, IotAccount>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub iot_account: ProgramAccount<'info, IotAccount>,
}

#[account]
pub struct IotAccount {
    pub data: u64,
}
*/
