use anchor_lang::prelude::*;

declare_id!("3M1C9Z8941rrXfpeAiXyQLYxaJPg5GLiVXUZGPqRx26K");

#[derive(Accounts)]
pub struct CreateFoo<'info> {
    #[account(zero)]
    foo: AccountLoader<'info, Foo>,
    #[account(signer)]
    authority: AccountInfo<'info>,
}

#[account(zero_copy)]
#[derive(Default)]
#[repr(packed)]
pub struct Foo {
    pub authority: Pubkey,
    pub data: u64,
    pub second_data: u64,
    #[accessor(Pubkey)] // The `accessor` api will likely be removed.
    pub second_authority: [u8; 32],
}

#[derive(Accounts)]
pub struct UpdateFoo<'info> {
    #[account(mut, has_one = authority)]
    foo: AccountLoader<'info, Foo>,
    #[account(signer)]
    authority: AccountInfo<'info>,
}

#[program]
pub mod abc {
    use super::*;

    pub fn create_foo(ctx: Context<CreateFoo>) -> ProgramResult {
        let foo = &mut ctx.accounts.foo.load_init()?;
        foo.authority = *ctx.accounts.authority.key;
        foo.set_second_authority(ctx.accounts.authority.key);
        Ok(())
    }

    pub fn update_foo(ctx: Context<UpdateFoo>, data: u64) -> ProgramResult {
        let mut foo = ctx.accounts.foo.load_mut()?;
        foo.data = data;
        Ok(())
    }
}
