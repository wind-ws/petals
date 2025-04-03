import {For} from "solid-js";
import {my_public, my_tokenkey, setTrigger, trans} from "../store";
import {toast} from "solid-toast";
import {produce} from "solid-js/store";

export default function Donation() {
    let addr ;
    let amount;
    return <div class="flex w-full flex-col">
        <div class="divider">指定地址捐助</div>
        <div class="flex w-full h-[300px] justify-center">
            <input type="text" ref={addr} placeholder="Token地址" class="input m-2"/>
            <input type="number" ref={amount} class="input validator m-2"
                   required placeholder="捐助数量"
            />
            <button class="btn m-2" on:click={() => {
                let reciver = my_public().get.token_map[addr.value];
                if (reciver == null) {
                    toast.error("该地址不存在");
                    return ;
                }
                let my_amount = my_public().get.token_map[my_tokenkey()];
                if (Number(amount.value) > my_amount) {
                    toast.error("当前账户的软妹币数量不够捐赠数量");
                    return ;
                }

                trans(()=>{
                    my_public().set(produce(v=>{
                        v.token_map[addr.value] += Number(amount.value);
                        v.token_map[my_tokenkey()] -=Number(amount.value);
                    }))
                    setTrigger();
                })
            }}>发起捐助
            </button>
        </div>

        <div class="divider">公开募捐项目</div>

        <div class="flex flex-row w-full">
            <For each={Object.entries(my_public().get.raise_fund_list)} fallback={<div>None...</div>}>
                {([key,item],index) =>
                    <div class="flex flex-col rounded-md border border-gray-300 w-[250px] m-4"
                         on:click={()=>{
                             addr.value = key;
                            }}>

                        <div class={`flex  bg-cover bg-no-repeat w-full h-30`}>
                            <img class="w-full" src={(()=>{
                                let i = index()%3;
                                return `/images/${i}.jpg`
                            })()} alt=""></img>
                        </div>
                        <div class="flex w-full">
                            {item.title}
                        </div>
                        <div class="flex w-full">
                            {item.brief}
                        </div>
                        <div class="flex w-full">

                        </div>

                </div>}
            </For>
        </div>

    </div>;
}
