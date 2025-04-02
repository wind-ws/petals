import { createSignal } from "solid-js";
import { readFile } from "node:fs/promises";

export default function Home() {

   return (
      <>
         <div class={"bg-[url(/images/banner.jpg)]  bg-cover bg-no-repeat w-full h-[1100px]  "}>
            <p class={"text-[70px] text-white text-center pt-[270px] pb-[50px]"}>让我们帮助那些更需要帮助的人</p>
            <div class="flex justify-center">

               <button class="btn btn-accent btn-xl " on:click={()=>{

                  // @ts-ignore
                  document.getElementById('my_modal_1').showModal();

               }}>进行捐款</button>

            </div>
         </div>
      </>
   );
}
