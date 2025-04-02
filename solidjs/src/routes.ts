import { lazy } from "solid-js";
import type { RouteDefinition } from "@solidjs/router";

import Home from "./pages/home";
import Root from "./pages/root";
import Info from "./pages/info";
import Donation from "./pages/donation";
import AirDrop from "./pages/airdrop";
import Publish from "./pages/publish";


export const routes: RouteDefinition[] = [
   {
      path: "/",
      component: Root,
      children:[
         {
            path:"/",
            component:Home
         },
         {
            path:"/info",
            component:Info
         },{
            path:"/donation",
            component:Donation
         },{
            path:"/airdrop",
            component:AirDrop
         },{
            path:"/publish",
            component:Publish
         }
      ]
   },
   {
      path: "**",
      component: lazy(() => import("./errors/404")),
   },
];
