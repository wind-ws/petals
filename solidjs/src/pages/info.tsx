import {SolanaKeyPair} from "../state";
import {my_pubkey, my_public, my_tokenkey, save, setTrigger, trans} from "../store";
import {For, Show} from "solid-js";
import {produce} from "solid-js/store";

export default function Info() {
   let name;
   let id;
   let phone;

   return <>
      <div class="overflow-x-auto rounded-box border border-base-content/5 bg-base-100">
         <table class="table">
            <tbody>
            <tr>
               <td class="">公钥</td>
               <td>{SolanaKeyPair[0]().publicKey.toString()}</td>
            </tr>
            <tr>
               <td>私钥</td>
               <td>{SolanaKeyPair[0]().secretKey.toString()}</td>
            </tr>
            <tr>
               <td>Token地址</td>
               <td>{my_tokenkey()}</td>
            </tr>
            <tr>
               <td>Token数量(软妹币)</td>
               <td>{my_public().get.token_map[my_tokenkey()]}</td>
            </tr>
            </tbody>
         </table>
      </div>
      <div>
         <Show when={my_public().get.myself[my_pubkey()].info != null}
               fallback={
                  <button class="btn" on:click={()=>{
                     // @ts-ignore
                     document.getElementById('my_modal_2_1').showModal()
                  }}>注册个人信息</button>
               }>
            <div class="overflow-x-auto rounded-box border border-base-content/5 bg-base-100">
               <table class="table">
                  <tbody>
                  <tr>
                     <td class="">姓名</td>
                     <td>{my_public().get.myself[my_pubkey()].info.name}</td>
                  </tr>
                  <tr>
                     <td>身份证</td>
                     <td>{my_public().get.myself[my_pubkey()].info.id}</td>
                  </tr>
                  <tr>
                     <td>手机号</td>
                     <td>{my_public().get.myself[my_pubkey()].info.phone}</td>
                  </tr>

                  </tbody>
               </table>
            </div>
         </Show>
         <dialog id="my_modal_2_1" class="modal">
            <div class="modal-box">
               <h3 class="text-lg font-bold">注册你的个人信息</h3>
               <input type="text" ref={name} placeholder="姓名" class="input input-sm"/>
               <input type="text" ref={id} placeholder="身份证" class="input input-sm"/>
               <input type="text" ref={phone} placeholder="手机号" class="input input-sm"/>
               <button class="btn" on:click={() => {
                  trans(() => {
                     my_public().set(produce((token) => {
                        token.myself[my_pubkey()].info = {
                           name:name.value,
                           id:id.value,
                           phone:phone.value
                        };
                        
                     }));
                     setTrigger(1);
                     save();
                  });
               }}>注册</button>
            </div>
            <form method="dialog" class="modal-backdrop">
            <button>close</button>
            </form>
         </dialog>
      </div>
      <div>
         其他 用户Token信息
         <div class="overflow-x-auto rounded-box border border-base-content/5 bg-base-100">
            <table class="table">
               <thead>
               <tr>
                  <th>Token地址</th>
                  <th>软妹币数量</th>
               </tr>
               </thead>
               <tbody>
               <For each={Object.entries(my_public().get.token_map)} fallback={<div>None...</div>}>
                  {
                     ([key,item])=>{
                        return <tr>
                           <td class="">{key}</td>
                           <td class="">{item}</td>
                        </tr>
                     }
                  }
               </For>

               </tbody>
            </table>
         </div>
      </div>
   </>;
}
