/**
 * FlashTraffic Playwright Simulation Worker
 * Simulates natural human-like web traffic, mouse physics, and multi-page surfing.
 */

const { chromium } = require("playwright");

// Cubic Bezier calculation for human-like mouse curve trajectory
function bezier(p0, p1, p2, p3, t) {
	const cX = 3 * (p1.x - p0.x);
	const bX = 3 * (p2.x - p1.x) - cX;
	const aX = p3.x - p0.x - cX - bX;

	const cY = 3 * (p1.y - p0.y);
	const bY = 3 * (p2.y - p1.y) - cY;
	const aY = p3.y - p0.y - cY - bY;

	const x = aX * Math.pow(t, 3) + bX * Math.pow(t, 2) + cX * t + p0.x;
	const y = aY * Math.pow(t, 3) + bY * Math.pow(t, 2) + cY * t + p0.y;

	return { x: Math.round(x), y: Math.round(y) };
}

async function simulateHumanMouse(page, startX, startY, endX, endY) {
	const p0 = { x: startX, y: startY };
	const p3 = { x: endX, y: endY };
	// Random control points for natural arc
	const p1 = {
		x: startX + (endX - startX) * 0.25 + (Math.random() - 0.5) * 50,
		y: startY + (endY - startY) * 0.1 + (Math.random() - 0.5) * 50
	};
	const p2 = {
		x: startX + (endX - startX) * 0.75 + (Math.random() - 0.5) * 50,
		y: startY + (endY - startY) * 0.9 + (Math.random() - 0.5) * 50
	};

	const steps = 15;
	for (let i = 1; i <= steps; i++) {
		const pt = bezier(p0, p1, p2, p3, i / steps);
		await page.mouse.move(pt.x, pt.y);
		await new Promise(r => setTimeout(r, 10 + Math.random() * 15));
	}
}

async function simulateHumanScroll(page, totalHeight) {
	let currentScroll = 0;
	const targetScroll = Math.min(totalHeight * (0.4 + Math.random() * 0.5), totalHeight);

	while (currentScroll < targetScroll) {
		const step = 80 + Math.floor(Math.random() * 120);
		currentScroll += step;
		await page.evaluate((y) => window.scrollTo({ top: y, behavior: "smooth" }), currentScroll);
		
		// Reading pause
		const pause = 250 + Math.floor(Math.random() * 600);
		await new Promise(r => setTimeout(r, pause));
		
		// Random slight scroll up (human re-reading)
		if (Math.random() < 0.2 && currentScroll > 200) {
			currentScroll -= 60;
			await page.evaluate((y) => window.scrollTo({ top: y, behavior: "smooth" }), currentScroll);
			await new Promise(r => setTimeout(r, 300));
		}
	}
}

async function runSession() {
	// Parse input arguments from CLI or environment (supports Base64 and direct JSON)
	const args = process.argv.slice(2);
	let config = {};

	const envCfg = process.env.FLASHTRAFFIC_CONFIG;
	if (envCfg) {
		try {
			config = JSON.parse(envCfg);
		} catch (e) {
			console.warn("[WARN] Failed to parse FLASHTRAFFIC_CONFIG env:", e.message);
		}
	} else if (args.length > 0) {
		const rawArg = args[0].trim();
		try {
			if (rawArg.startsWith("{") || rawArg.startsWith("[")) {
				config = JSON.parse(rawArg);
			} else {
				// Try base64 decoding first
				const decoded = Buffer.from(rawArg, "base64").toString("utf-8");
				config = JSON.parse(decoded);
			}
		} catch (e) {
			try {
				config = JSON.parse(rawArg);
			} catch (parseErr) {
				console.error("[ERROR] Failed to parse config JSON:", parseErr.message);
				process.exit(1);
			}
		}
	}

	const {
		url = "https://example.com",
		referrer = "",
		userAgent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36",
		viewport = { width: 1366, height: 768 },
		headless = true,
		dwellTimeMs = 5000,
		proxy = null,
		allowInternalSurfing = true,
		surfingCount = 2,
		trafficMode = "default"
	} = config;

	console.log(`[INFO] Starting visit to: ${url} (Mode: ${trafficMode})`);
	console.log(`[INFO] Referrer: ${referrer || "(Direct)"}`);

	const launchOptions = {
		headless: headless,
		args: [
			"--no-sandbox",
			"--disable-blink-features=AutomationControlled",
			"--disable-infobars"
		]
	};

	if (proxy) {
		launchOptions.proxy = {
			server: proxy.server,
			username: proxy.username,
			password: proxy.password
		};
	}

	let browser;
	try {
		browser = await chromium.launch(launchOptions);
	} catch (launchErr) {
		console.warn(`[WARN] Standard chromium launch failed (${launchErr.message}). Attempting system browser fallback (msedge)...`);
		try {
			browser = await chromium.launch({ ...launchOptions, channel: "msedge" });
		} catch (edgeErr) {
			console.warn(`[WARN] msedge fallback failed. Attempting chrome...`);
			try {
				browser = await chromium.launch({ ...launchOptions, channel: "chrome" });
			} catch (chromeErr) {
				console.error(`[FATAL] All browser launch strategies exhausted: ${chromeErr.message}`);
				process.exit(1);
			}
		}
	}

	try {
		const context = await browser.newContext({
			userAgent,
			viewport,
			locale: "en-US",
			timezoneId: "America/New_York",
			permissions: ["geolocation"],
			extraHTTPHeaders: referrer ? { "Referer": referrer } : {}
		});

		// Evasion script
		await context.addInitScript(() => {
			Object.defineProperty(navigator, "webdriver", { get: () => undefined });
			window.chrome = { runtime: {} };
		});

		const page = await context.newPage();
		page.setDefaultTimeout(30000);

		const response = await page.goto(url, { waitUntil: "domcontentloaded" });
		const status = response ? response.status() : 0;
		console.log(`[SUCCESS] Page loaded. HTTP Status: ${status}`);

		const isFastMode = trafficMode === "max_bounce" || trafficMode === "max_sessions" || dwellTimeMs <= 1500;

		if (!isFastMode) {
			// Simulate natural mouse motion
			await simulateHumanMouse(page, 100, 100, 450, 300);

			// Human reading and scrolling
			const bodyHeight = await page.evaluate(() => document.body.scrollHeight || 1000);
			await simulateHumanScroll(page, bodyHeight);

			// Multi-page surfing inside target domain (Torpedo surfing mode)
			if (allowInternalSurfing && surfingCount > 0) {
				try {
					const currentHost = new URL(url).hostname;
					const visitedUrls = new Set([url]);

					for (let step = 1; step <= surfingCount; step++) {
						const internalLinks = await page.$$eval("a[href]", (anchors, host) => {
							return anchors
								.map(a => a.href)
								.filter(h => h.startsWith("http") && h.includes(host) && !h.includes("#") && !h.endsWith(".pdf") && !h.endsWith(".zip") && !h.endsWith(".png") && !h.endsWith(".jpg"));
						}, currentHost);

						const unvisited = internalLinks.filter(l => !visitedUrls.has(l));
						if (unvisited.length > 0) {
							const nextUrl = unvisited[Math.floor(Math.random() * unvisited.length)];
							visitedUrls.add(nextUrl);
							console.log(`[INFO] Surfing internal page (${step}/${surfingCount}): ${nextUrl}`);

							// Reading pause before click
							await new Promise(r => setTimeout(r, 800 + Math.random() * 1200));
							await page.goto(nextUrl, { waitUntil: "domcontentloaded" });
							await simulateHumanMouse(page, 200, 250, 500, 380);
							await simulateHumanScroll(page, 900);
						} else {
							break;
						}
					}
				} catch (surfErr) {
					console.log(`[WARN] Internal surfing skipped: ${surfErr.message}`);
				}
			}

			// Remaining dwell time
			if (dwellTimeMs > 3000) {
				await new Promise(r => setTimeout(r, dwellTimeMs - 2500));
			}
		} else {
			console.log(`[FAST] Fast Pageview Mode active: completed DOMContentLoaded instantly with zero scroll delays.`);
			if (dwellTimeMs > 100) {
				await new Promise(r => setTimeout(r, Math.min(dwellTimeMs, 500)));
			}
		}

		console.log(`[DONE] Session completed successfully for: ${url}`);
	} catch (err) {
		console.error(`[ERROR] Session failed: ${err.message}`);
	} finally {
		if (browser) {
			await browser.close();
		}
	}
}

runSession();
