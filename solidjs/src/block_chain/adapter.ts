import { Keypair, PublicKey } from "@solana/web3.js";


// import * as _solana_web3 from "@solana/web3.js";
// export const solana_web3 = _solana_web3;

/// 统一操作
export interface _a{

}

/// 区块链网络(虽然kas不是区块[链]技术)
enum BlockChainNetWork {
   Sol,
   Eth,
   Kas,
}

/// 使用的那个区块链后端
enum BackEnd {
   SolanaNative,
   SolanaAnchor,
   EthereumSolidity,
   Kaspa,
}





export namespace solana_native {
   /// 来自vite.confg.ts中的define , 完成对前端 读取公钥,但私钥隐藏
   declare const SolanaNativePubkey: PublicKey;
   export function program_id(): PublicKey {
      return SolanaNativePubkey;
   }
}
export namespace ethereum_solidity {

}
export namespace solana_anchor {
   
}
