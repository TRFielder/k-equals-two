import Welcome from "@/features/welcome"
import { render } from "@testing-library/react"
import { MemoryRouter } from "react-router-dom"

describe("Welcome", () => {
	test("Should render the component text correctly", () => {
		const { getByRole } = render(
			<MemoryRouter>
				<Welcome />
			</MemoryRouter>
		)

		expect(getByRole("heading", { name: "Welcome to k = 2" })).toBeVisible()
	})

	test("Should render the button correctly", () => {
		const { getByRole } = render(
			<MemoryRouter>
				<Welcome />
			</MemoryRouter>
		)

		expect(getByRole("link", { name: "Begin" })).toBeVisible()
	})
})
