import { Connection } from "@solana/web3.js";


export namespace solana {

   export enum Net{
      /// 这是不可能的,要真money
      Main,
      /// 必须的测试网
      Test,
      /// 可能也是开发网
      Dev,
      /// 开发阶段用本地网
      Local,
   }
   const NET = Net.Local;
   // const CONNECTION = Connection.

}

export namespace ethereum {

}

export namespace kaspa {
   
}
