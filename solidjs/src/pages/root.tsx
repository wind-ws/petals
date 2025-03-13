import { useNavigate } from "@solidjs/router";
import { SolanaKeyPair } from "../state";
import { Show } from "solid-js";
import { Keypair } from "@solana/web3.js";

export default function Root(props) {
   const navigate = useNavigate();

   return (
      <>
         <div class="navbar bg-base-100 shadow-sm">
            <div class="navbar-start">
               <a
                  class="btn btn-ghost text-xl"
                  on:click={() => {
                     navigate("/");
                  }}>
                  Petls
               </a>
            </div>
            <div class="navbar-center hidden lg:flex">
               <ul class="menu menu-horizontal px-1">
                  <li>
                     <a
                        on:click={() => {
                           navigate("/");
                        }}>
                        Home
                     </a>
                  </li>

                  <li>
                     <a
                        on:click={() => {
                           navigate("/info");
                        }}>
                        Info
                     </a>
                  </li>
               </ul>
            </div>
            <div class="navbar-end">
               <Show
                  when={SolanaKeyPair[0]() == null}
                  fallback={<p>{SolanaKeyPair[0]().publicKey.toString()}</p>}>
                  <a
                     class="btn"
                     on:click={() => {
                        SolanaKeyPair[1](Keypair.generate());
                     }}>
                     Connect
                  </a>
               </Show>
            </div>
         </div>
         <div>{props.children}</div>
      </>
   );
}
