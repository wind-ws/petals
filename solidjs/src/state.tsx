import { Keypair } from "@solana/web3.js";
import {createEffect, createSignal, Signal} from "solid-js";
import {init, my_public, save, setTrigger} from "./store";


let SolanaKeyPair: Signal<Keypair | null> = createSignal(null);

createEffect(()=>{
   if(SolanaKeyPair[0]()!=null){
      let b = my_public().get.myself[SolanaKeyPair[0]().publicKey.toString()];
      if(b == null){
         init(SolanaKeyPair[0]().publicKey,null)
         setTrigger(2)
      }
   }
})

export {
   SolanaKeyPair
}