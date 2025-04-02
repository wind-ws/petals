import {my_pubkey, publish, setTrigger, trans} from "../store";
import {SolanaKeyPair} from "../state";

export default function Publish() {
    let title;
    let brief;
    return <>
        <div class="flex w-full h-[300px] justify-center">
            <input type="text" ref={title} placeholder="title" class="input m-2"/>
            <input type="text" ref={brief} placeholder="brief" class="textarea m-2"/>
            <button class="btn" on:click={()=>{
                trans(()=>{
                    publish(SolanaKeyPair[0]().publicKey,title.value,brief.value);
                    setTrigger();
                });
            }}>发布募捐</button>
        </div>
    </>;
}
