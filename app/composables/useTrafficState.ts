export interface LogEntry {
	id: string
	time: string
	level: "INFO" | "SUCCESS" | "WARN" | "ERROR"
	text: string
}

export function useTrafficState() {
	const activeCampaign = useState("active_campaign", () => ({
		name: "Default Growth Campaign",
		targetUrls: ["https://example.com/"],
		trafficType: "Organic" as "Direct" | "Organic" | "Referral",
		trafficSource: "Google",
		searchEngines: ["Google"] as string[],
		referralSources: ["Facebook"] as string[],
		customReferralUrl: "",
		keywords: "best web analytics tools 2026",
		keywordList: ["best web analytics tools 2026"] as string[],
		utmCampaign: "spring_launch",
		utmMedium: "organic",
		utmSource: "google",
		headless: true,
		dwellTimeSeconds: 6,
		allowSurfing: true,
		maxSessions: 15,
		enableProxy: false,
		selectedProxies: [] as string[],

		// Torpedo Advanced Simulation Engine
		trafficMode: "default" as "default" | "max_sessions" | "max_pageviews" | "min_bounce" | "max_bounce",
		sessionDurationType: "variable" as "constant" | "variable",
		sessionDurationConstant: 5000,
		sessionDurationMin: 4000,
		sessionDurationMax: 10000,
		trafficDelayType: "variable" as "constant" | "variable",
		trafficDelayConstant: 1500,
		trafficDelayMin: 2000,
		trafficDelayMax: 5000,
		pageOrder: "random" as "in_order" | "random",
		proxySwitching: "single_page" as "single_page" | "cycle" | "time_based",
		useRandomProxy: true,
		platformSwitching: "random_page" as "single" | "random_page",
		specialMode: "surfing" as "repetition" | "surfing",
		surfingCount: 2
	}));

	const proxies = useState<string[]>("proxy_list", () => [
		"104.248.63.15:3128",
		"198.199.86.11:8080",
		"165.227.215.71:3128",
		"159.203.61.169:8080"
	]);

	const BROWSERSTACK_STORAGE_KEY = "flashtraffic_browserstack_credentials";

	const getInitialBrowserStackConfig = () => {
		const base = {
			username: "",
			accessKey: "",
			selectedOS: "Windows 11",
			selectedBrowser: "Chrome 128",
			selectedDevice: "Desktop Chrome",
			selectedDevices: [
				"iPhone 15 Pro (iOS 17)",
				"Samsung Galaxy S24 Ultra",
				"Windows 11 (Chrome 128)"
			] as string[],
			isConnected: false,
			planName: "Not Connected",
			maxParallel: 5,
			runningSessions: 0,
			enableCloud: false
		};

		if (typeof window !== "undefined" && window.localStorage) {
			try {
				const saved = window.localStorage.getItem(BROWSERSTACK_STORAGE_KEY);
				if (saved) {
					const parsed = JSON.parse(saved);
					return { ...base, ...parsed };
				}
			} catch (e) {
				console.warn("Failed to load saved BrowserStack credentials:", e);
			}
		}
		return base;
	};

	const browserStackConfig = useState("browserstack_config", () => getInitialBrowserStackConfig());

	const saveBrowserStackConfig = () => {
		if (typeof window !== "undefined" && window.localStorage) {
			try {
				const toSave = {
					username: browserStackConfig.value.username,
					accessKey: browserStackConfig.value.accessKey,
					selectedDevices: browserStackConfig.value.selectedDevices,
					enableCloud: browserStackConfig.value.enableCloud,
					isConnected: browserStackConfig.value.isConnected,
					planName: browserStackConfig.value.planName,
					maxParallel: browserStackConfig.value.maxParallel
				};
				window.localStorage.setItem(BROWSERSTACK_STORAGE_KEY, JSON.stringify(toSave));
			} catch (e) {
				console.warn("Failed to save BrowserStack credentials to localStorage:", e);
			}
		}
	};

	const clearBrowserStackConfig = () => {
		browserStackConfig.value.username = "";
		browserStackConfig.value.accessKey = "";
		browserStackConfig.value.isConnected = false;
		browserStackConfig.value.planName = "Not Connected";
		browserStackConfig.value.enableCloud = false;
		if (typeof window !== "undefined" && window.localStorage) {
			window.localStorage.removeItem(BROWSERSTACK_STORAGE_KEY);
		}
	};

	const liveMetrics = useState("live_metrics", () => ({
		totalVisits: 0,
		successfulVisits: 0,
		failedVisits: 0,
		activeVisitors: 0,
		statusText: "Ready to launch"
	}));

	const isRunning = useState("engine_is_running", () => false);

	const logs = useState<LogEntry[]>("traffic_logs", () => [
		{
			id: "1",
			time: new Date().toLocaleTimeString(),
			level: "INFO",
			text: "FlashTraffic engine initialized. Ready to start campaigns."
		}
	]);

	const torpedoRawLogs = useState<string[]>("torpedo_raw_logs", () => []);

	const addLog = (level: "INFO" | "SUCCESS" | "WARN" | "ERROR", text: string) => {
		logs.value.unshift({
			id: Math.random().toString(36).substring(2, 9),
			time: new Date().toLocaleTimeString(),
			level,
			text
		});
		if (logs.value.length > 150) {
			logs.value.pop();
		}
	};

	const clearLogs = () => {
		logs.value = [];
		torpedoRawLogs.value = [];
	};

	const exportSettingsJson = () => {
		const payload = {
			appName: "FlashTraffic",
			version: "1.0.0",
			exportedAt: new Date().toISOString(),
			browserStack: {
				username: browserStackConfig.value.username,
				accessKey: browserStackConfig.value.accessKey,
				selectedDevices: browserStackConfig.value.selectedDevices,
				enableCloud: browserStackConfig.value.enableCloud,
				maxParallel: browserStackConfig.value.maxParallel
			},
			campaign: { ...activeCampaign.value },
			proxies: [...proxies.value]
		};
		return JSON.stringify(payload, null, 2);
	};

	const importSettingsJson = (jsonStr: string): boolean => {
		try {
			const data = JSON.parse(jsonStr);
			if (data.browserStack) {
				if (data.browserStack.username !== undefined) browserStackConfig.value.username = data.browserStack.username;
				if (data.browserStack.accessKey !== undefined) browserStackConfig.value.accessKey = data.browserStack.accessKey;
				if (data.browserStack.selectedDevices) browserStackConfig.value.selectedDevices = data.browserStack.selectedDevices;
				if (data.browserStack.enableCloud !== undefined) browserStackConfig.value.enableCloud = data.browserStack.enableCloud;
				if (data.browserStack.maxParallel) browserStackConfig.value.maxParallel = data.browserStack.maxParallel;
				saveBrowserStackConfig();
			}
			if (data.campaign) {
				activeCampaign.value = { ...activeCampaign.value, ...data.campaign };
			}
			if (data.proxies && Array.isArray(data.proxies)) {
				proxies.value = data.proxies;
			}
			addLog("SUCCESS", "FlashTraffic configuration imported successfully.");
			return true;
		} catch (err: any) {
			console.error("Failed to import settings:", err);
			addLog("ERROR", `Failed to parse settings JSON: ${err.message}`);
			return false;
		}
	};

	const downloadSettingsFile = () => {
		if (typeof window === "undefined") return;
		const jsonStr = exportSettingsJson();
		const blob = new Blob([jsonStr], { type: "application/json" });
		const url = URL.createObjectURL(blob);
		const a = document.createElement("a");
		a.href = url;
		a.download = `flashtraffic-settings-${new Date().toISOString().slice(0, 10)}.json`;
		document.body.appendChild(a);
		a.click();
		document.body.removeChild(a);
		URL.revokeObjectURL(url);
		addLog("INFO", "Settings file exported.");
	};

	return {
		activeCampaign,
		proxies,
		browserStackConfig,
		liveMetrics,
		isRunning,
		logs,
		torpedoRawLogs,
		addLog,
		clearLogs,
		saveBrowserStackConfig,
		clearBrowserStackConfig,
		exportSettingsJson,
		importSettingsJson,
		downloadSettingsFile
	};
}
