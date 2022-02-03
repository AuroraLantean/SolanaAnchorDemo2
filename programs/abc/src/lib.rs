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
    pub uotes: [u128; 3], // unsigned 128 bit integers
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

    pub fn update_foo(ctx: Context<UpdateFoo>, data: u64, uotes_a: u128) -> ProgramResult {
        let mut foo = ctx.accounts.foo.load_mut()?;
        foo.data = data;
        foo.uotes[0] = uotes_a;
        Ok(())
    }

    pub fn create_bar(ctx: Context<CreateBar>) -> ProgramResult {
        let bar = &mut ctx.accounts.bar.load_init()?;
        bar.authority = *ctx.accounts.authority.key;
        Ok(())
    }

    pub fn update_bar(ctx: Context<UpdateBar>, data: u64, uotes_a: u128) -> ProgramResult {
        let bar = &mut ctx.accounts.bar.load_mut()?;
        bar.data = data;
        bar.uotes[0] = uotes_a;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateBar<'info> {
    #[account(
        init,
        seeds = [authority.key().as_ref(), foo.key().as_ref()],
        bump,
        payer = authority, owner = *program_id
    )]
    bar: AccountLoader<'info, Bar>,
    #[account(signer)]
    authority: AccountInfo<'info>,
    foo: AccountLoader<'info, Foo>,
    system_program: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct UpdateBar<'info> {
    #[account(
        mut,
        has_one = authority,
        seeds = [authority.key().as_ref(), foo.key().as_ref()],
        bump,
    )]
    pub bar: AccountLoader<'info, Bar>,
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    pub foo: AccountLoader<'info, Foo>,
}

#[account(zero_copy)]
#[derive(Default)]
pub struct Bar {
    pub authority: Pubkey,
    pub data: u64,
    pub uotes: [u128; 3], // unsigned 128 bit integers
}
