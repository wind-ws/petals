import { Keypair } from "@solana/web3.js";
import {createEffect, createSignal, Signal} from "solid-js";
import {init, my_public, setTrigger} from "./store";


let SolanaKeyPair: Signal<Keypair | null> = createSignal(null);

createEffect(()=>{
   if(SolanaKeyPair[0]()!=null){
      init(SolanaKeyPair[0]().publicKey,null)
      setTrigger()
   }
})

export {
   SolanaKeyPair
}