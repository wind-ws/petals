use solana_program::pubkey::Pubkey;

/// Mint
pub struct MintRmb {}
/// 意义化Pubkey
pub type MintRmbPubkey = Pubkey;
/// 意义化Pubkey
pub type TokenRmbPubkey = Pubkey;
impl MintRmb {
    pub const SEED: &[u8; 8] = b"mint_rmb";

    pub fn pda(program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[MintRmb::SEED], program_id)
    }

    /// get associated token account pda
    ///
    /// require: mint_rmb_key == MintRmbPda::pda(...).0
    ///
    /// todo : remove the program_id
    pub fn token_account(
        _program_id: &Pubkey,
        owner: &Pubkey,
        mint_rmb: &MintRmbPubkey,
    ) -> (Pubkey, u8) {
        // getAssociatedTokenAddressSync
        Pubkey::find_program_address(
            &[
                &owner.to_bytes(),
                &spl_token::ID.to_bytes(),
                &mint_rmb.to_bytes(),
            ],
            &spl_associated_token_account::ID,
        )
    }
}
