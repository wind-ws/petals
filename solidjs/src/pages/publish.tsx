import { my_pubkey, publish, save, setTrigger, trans } from "../store";
import { SolanaKeyPair } from "../state";

export default function Publish() {
   let title;
   let brief;
   return (
      <>
         <div class="flex w-full h-[300px] justify-center">
            <div class="flex ">
               <input
                  type="text"
                  ref={title}
                  placeholder="title"
                  class="input m-2"
               />
            </div>
            <div class="flex ">
               <input
                  type="text"
                  ref={brief}
                  placeholder="brief"
                  class="textarea m-2"
               />
            </div>

            <button
               class="btn"
               on:click={() => {
                  trans(() => {
                     publish(
                        SolanaKeyPair[0]().publicKey,
                        title.value,
                        brief.value
                     );
                     setTrigger(4);
                     save();
                  });
               }}>
               发布募捐
            </button>
         </div>
      </>
   );
}
