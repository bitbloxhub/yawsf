/** @type {import("prettier").Config} */
const config = {
	useTabs: true,
	singleQuote: false,
	trailingComma: "all",
	semi: false,
	printWidth: 100,
	plugins: ["prettier-plugin-astro"],
	overrides: [{ files: "*.astro", options: { parser: "astro" } }],
}

export default config
