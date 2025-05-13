import { useNavigate } from "@solidjs/router";
import { SolanaKeyPair } from "../state";
import { createSignal, Show } from "solid-js";
import { Keypair } from "@solana/web3.js";
import bs58 from "bs58";
import { Toaster } from "solid-toast";

export default function Root(props) {
   const navigate = useNavigate();
   const [inputValue, setInputValue] = createSignal(""); // 创建信号
   let inputRef;
   return (
      <>
         <Toaster />
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
                           navigate("/donation");
                        }}>
                        Donation
                     </a>
                  </li>
                  <li>
                     <a
                        on:click={() => {
                           navigate("/publish");
                        }}>
                        Publish
                     </a>
                  </li>
                  <li>
                     <a
                        on:click={() => {
                           navigate("/airdrop");
                        }}>
                        AirDrop
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
                        // @ts-ignore
                        document.getElementById("my_modal_1").showModal();
                     }}>
                     Connect
                  </a>
               </Show>
            </div>
            <dialog
               id="my_modal_1"
               class="modal">
               <div class="modal-box">
                  <h3 class="text-lg font-bold">连接账户</h3>
                  <p class="py-4">
                     注意:确保私钥的安全性,你需要断开网络,关闭其他可能存在危险的程序,不要将私钥保存在本地或剪切板
                  </p>
                  <input
                     type="text"
                     placeholder="输入你的私钥"
                     ref={inputRef}
                     class="input"
                  />
                  <button
                     class="btn"
                     on:click={() => {
                        console.log(inputRef.value);
                        // const privateKeyBytes = bs58.decode("["+inputRef.value+"]");
                        const numbers = inputRef.value
                           .split(",")
                           .map((num) => Number(num));
                        const privateKeyBytes = Uint8Array.from(numbers);
                        console.log(privateKeyBytes);
                        const keypair = Keypair.fromSecretKey(privateKeyBytes);
                        SolanaKeyPair[1](keypair);
                     }}>
                     Connect
                  </button>
                  <button
                     class="btn"
                     on:click={() => {
                        SolanaKeyPair[1](Keypair.generate());
                     }}>
                     生成一个私钥
                  </button>
                  <div class="modal-action">
                     <form method="dialog">
                        <button class="btn">Close</button>
                     </form>
                  </div>
               </div>
            </dialog>
         </div>
         <div>{props.children}</div>
      </>
   );
}
