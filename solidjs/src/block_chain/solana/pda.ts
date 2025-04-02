import {solana_native} from "../adapter";
import {PublicKey} from "@solana/web3.js";
import {Buffer} from "buffer";

export const SPL_TOKEN = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
export const SPL_ASSOCIATED_TOKEN_ACCOUNT = new PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

// export function pda_rmb_mint():PublicKey{
//     return PublicKey.findProgramAddressSync([Buffer.from("mint_rmb")],solana_native.program_id())[0]
// }
//
// export function pda_token(owner:PublicKey):PublicKey {
//     return PublicKey.findProgramAddressSync([owner.toBytes(),SPL_TOKEN.toBytes(),pda_rmb_mint().toBytes()],SPL_ASSOCIATED_TOKEN_ACCOUNT)[0]
// }
