import router from "@/lib/router";
import { RouterProvider } from "react-router-dom";

function App() {
	return (
		<main className="bg-gray-400">
			<RouterProvider router={router} />
		</main>
	);
}

export default App;
