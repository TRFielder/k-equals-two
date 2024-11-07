import type { RouteObject } from "react-router-dom";
import Welcome from "@/features/welcome";
console.log("Hi");
export const routes: RouteObject[] = [{ path: "/", element: <Welcome /> }];
