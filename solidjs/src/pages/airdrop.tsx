import { my_public, my_tokenkey, save, setTrigger, trans } from "../store";
import { produce } from "solid-js/store";

export default function AirDrop() {
   let amount;

   return (
      <>
         <div>
            <input
               type="number"
               ref={amount}
               class="input validator"
               required
               placeholder="申请数量"
            />
            <button
               class="btn"
               on:click={() => {
                  trans(() => {
                     my_public().set(
                        produce((v) => {
                           v.token_map[my_tokenkey()] += Number(amount.value);
                        })
                     );
                     setTrigger(6);
                     save();
                  });
               }}>
               申请空投
            </button>
         </div>
      </>
   );
}
