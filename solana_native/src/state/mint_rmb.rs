use solana_program::pubkey::Pubkey;

pub struct MintRmbPda {}

impl MintRmbPda {
    pub const SEED: &[u8; 8] = b"mint_rmb";

    pub fn pda(program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[MintRmbPda::SEED], program_id)
    }
}
