import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";
import * as fs from "node:fs";
import { Keypair } from "@solana/web3.js";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig({
   plugins: [tailwindcss(),solidPlugin()],
   server: {
      port: 3000,
   },
   build: {
      target: "esnext",
   },
   define: {
      SolanaNativePubkey: Keypair.fromSecretKey(
         Buffer.from(
            JSON.parse(
               fs.readFileSync(
                  "../solana_native/target/deploy/petals_solana_native-keypair.json",
                  "utf-8"
               )
            )
         )
      ).publicKey,
   },
});
