import { lazy } from "solid-js";
import type { RouteDefinition } from "@solidjs/router";

import Home from "./pages/home";
import Root from "./pages/root";
import Info from "./pages/info";


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
         }
      ]
   },
   {
      path: "**",
      component: lazy(() => import("./errors/404")),
   },
];
