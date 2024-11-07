import { Link } from "react-router-dom"

const Welcome = () => (
	<article className="flex flex-col items-center justify-evenly w-full h-screen">
		<h1 className="text-3xl font-bold text-black font-sans">
			Welcome to <em>k</em> = 2
		</h1>
		<Link to="create">
			<button
				type="button"
				className="bg-white hover:bg-gray-100 text-gray-800 font-semibold py-2 px-4 border border-gray-400 rounded shadow"
			>
				Begin
			</button>
		</Link>
	</article>
)

export default Welcome
