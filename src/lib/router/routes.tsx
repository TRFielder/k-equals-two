import type { RouteObject } from "react-router-dom"
import Welcome from "@/features/welcome"
import Create from "@/features/create"

export const routes: RouteObject[] = [
	{ path: "/", element: <Welcome /> },
	{ path: "/create", element: <Create /> },
]
