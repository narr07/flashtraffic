export default defineAppConfig({
	app: {
		name: "FlashTraffic",
		description: "Modern Web Traffic Simulation & Analytics Load Testing Desktop Platform",
		version: "0.1.0"
	},
	ui: {
		colors: {
			primary: "emerald",
			neutral: "zinc"
		},
		button: {
			slots: {
				base: "cursor-pointer"
			}
		},
		formField: {
			slots: {
				root: "w-full"
			}
		},
		input: {
			slots: {
				root: "w-full"
			}
		},
		textarea: {
			slots: {
				root: "w-full",
				base: "resize-none"
			}
		}
	}
});
