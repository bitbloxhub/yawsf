// @ts-check
import { defineConfig } from "astro/config"
import starlight from "@astrojs/starlight"
import starlightCatppuccin from "@catppuccin/starlight"

export default defineConfig({
	server: {
		host: "127.0.0.1",
		port: 24540,
	},
	integrations: [
		starlight({
			plugins: [starlightCatppuccin()],
			title: "YAWSF",
			description: "Build native Wayland desktop surfaces with web technologies.",
			customCss: ["@fontsource-variable/fira-code/wght.css", "./src/styles/custom.css"],
			sidebar: [
				{ label: "Overview", slug: "" },
				{
					label: "Guides",
					items: [
						{ label: "Getting started", slug: "guides/getting-started" },
						{ label: "Build a shell", slug: "guides/build-a-shell" },
						{ label: "Example shell", slug: "guides/example-shell" },
					],
				},
				{
					label: "Reference",
					items: [
						{ label: "CLI", slug: "reference/cli" },
						{ label: "Host API", slug: "reference/host-api" },
					],
				},
			],
		}),
	],
})
