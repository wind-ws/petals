import {toast} from "solid-toast";
import {Keypair, PublicKey} from "@solana/web3.js";
import {solana_native} from "./block_chain/adapter";
import {SolanaKeyPair} from "./state";
import {createEffect, createSignal, Signal} from "solid-js";
import {createStore, produce} from "solid-js/store";
import {SetStoreFunction, Store} from "solid-js/store/types/store";

export const [trigger, setTrigger] = createSignal(0);
let my_public_signal:[get: Store<Public>, set: SetStoreFunction<Public>] = null;

const INIT = (()=>{
    let a = localStorage.getItem("init");
    if(a == null || a.length == 0) {
        localStorage.setItem("init","123");
        [{
            title:"癌症",
            brief:"恶性肿瘤腹痛、黄疸、体重减轻或食欲下降"}, {
            title:"中风",
            brief:"单侧肢体无力或麻木、言语困难、视力模糊、行走不稳或剧烈头痛"}
        ].forEach((n)=>{
            let pubkey = Keypair.generate().publicKey;
            init(pubkey,{
                name:"刘超",
                id:"0123456789012345679",
                phone:"1553215789"
            });
            publish(pubkey,n.title,n.brief);
        })
        return false;
    } else {
        return true;
    }
})();



export function my_pubkey(){
    return SolanaKeyPair[0]().publicKey.toString()
}
export function my_tokenkey(){
    return my_public().get.myself[my_pubkey()].token_pubkey.toString()
}

export function init(pubkey:PublicKey,info:{
    phone:string,
    id:string,
    name:string,
}|null){
    my_public().set(produce(v=>{
        let token_key = Keypair.generate().publicKey;
        v.myself[pubkey.toString()] = {
            token_pubkey: token_key,
            info,
        };
        v.token_map[token_key.toString()] = 999;
    }))
}

export function publish(pubkey:PublicKey,title:string,brief:string,){
    my_public().set(produce(v=>{
        if(v.myself[pubkey.toString()] == null){
            toast("没有此公钥")
            return;
        }
        if(v.myself[pubkey.toString()].info == null){
            toast("未注册个人信息,不可发布募捐");
            return;
        }
        let tokenkey = v.myself[pubkey.toString()].token_pubkey;
        v.raise_fund_list[tokenkey.toString()] = {
            title,
            brief,

        }
    }))
}
export type Public = {
    raise_fund_list:{
        [token_pubkey:string]: {
            title: string,
            brief: string,
        }
    },
    token_map:{
        // token地址: 数量
        [token:string]:number
    },
    myself:{
        // 公钥
        [pubkey:string]:{
            // token地址
            token_pubkey:PublicKey,
            info:{
                phone:string,
                id:string,
                name:string,
            }|null,
        }
    }
};
/// 公共数据
export function my_public(){
    if (my_public_signal==null){
        let a = localStorage.getItem("my_public");
        let my_public:Public;
        if (a==null){
            let _my_public = <Public>{
                raise_fund_list:{},
                token_map:{},
                myself:{}
            };
            localStorage.setItem("my_public",JSON.stringify(_my_public));
            my_public = _my_public;
        } else {
            my_public = JSON.parse(a);
        }
        my_public_signal= createStore(my_public);
        createEffect(() => {
            my_public_signal[0].myself;
            my_public_signal[0].token_map;
            my_public_signal[0].raise_fund_list;
            trigger();
            localStorage.setItem("my_public", JSON.stringify(my_public_signal[0]));
        });
    }
    return {
        get:my_public_signal[0],
        set:my_public_signal[1],
    };
}



export function trans(fn: (() => void)){
    toast.promise(new Promise((resolve,reject)=>{
        const randomDelay = Math.floor(Math.random() * 2000) + 1000; // 1000ms - 3000ms
        setTimeout(()=>{
            let b = Math.random() < 0.050 ? false : true;
            if(b){
                fn();
                resolve(null);
            }else{
                reject();
            }
        },randomDelay);

    }), {
        loading: "交易执行中...",
        success: "交易成功",
        error: "交易失败!!!"
    })
}