import { createSignal } from "solid-js";
import { readFile } from "node:fs/promises";

export default function Home() {
   return (
      <>
      home
      <div>
         封面
      </div>
      <div>
         对指定 Token 进行捐款
      </div>
      <div>
         可捐款列表
      </div>
      </>
   );
}
