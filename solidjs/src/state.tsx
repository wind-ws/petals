import { Keypair } from "@solana/web3.js";
import { createSignal, Signal } from "solid-js";


let SolanaKeyPair: Signal<Keypair | null> = createSignal(null);

export {
   SolanaKeyPair
}