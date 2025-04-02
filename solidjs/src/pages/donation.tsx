import {For} from "solid-js";
import {my_public} from "../store";

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

            }}>发起捐助
            </button>
        </div>

        <div class="divider">公开募捐项目</div>

        <div class="flex flex-row w-full">
            <For each={Object.entries(my_public().get.raise_fund_list)} fallback={<div>Loading...</div>}>
                {([key,item],index) =>
                    <div class="flex flex-col rounded-md border border-gray-300 w-[250px] m-4">

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
