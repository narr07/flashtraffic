import { spawn } from "node:child_process";
import { createServer } from "node:net";

const DEFAULT_PORT = 3000;

function isPortAvailable(port: number): Promise<boolean> {
	return new Promise((resolve) => {
		const server = createServer();
		server.once("error", () => resolve(false));
		server.once("listening", () => {
			server.close(() => resolve(true));
		});
		server.listen(port, "0.0.0.0");
	});
}

async function findAvailablePort(startPort: number): Promise<number> {
	for (let port = startPort; port < startPort + 100; port++) {
		if (await isPortAvailable(port)) {
			return port;
		}
	}
	throw new Error(`No available port found in range ${startPort}-${startPort + 99}`);
}

async function main(): Promise<void> {
	const port = await findAvailablePort(DEFAULT_PORT);

	const args: string[] = ["run", "tauri", "dev"];

	// Only override config if port 3000 is occupied
	if (port !== DEFAULT_PORT) {
		console.log(`Port ${DEFAULT_PORT} is busy, switching to port ${port}`);
		const tauriConfig = JSON.stringify({
			build: {
				devUrl: `http://localhost:${port}`,
				beforeDevCommand: `bun run dev --port ${port}`
			}
		});

		// On Windows shell (cmd.exe), inner double quotes must be escaped with \"
		const escaped = process.platform === "win32"
			? `"${tauriConfig.replace(/"/g, '\\"')}"`
			: tauriConfig;

		args.push("--config", escaped);
	}

	const tauri = spawn("bun", args, {
		stdio: "inherit",
		shell: process.platform === "win32",
		cwd: process.cwd()
	});

	tauri.on("close", (code) => {
		process.exit(code ?? 0);
	});
}

main();
