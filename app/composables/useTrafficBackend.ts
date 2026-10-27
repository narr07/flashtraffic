import { invoke } from "@tauri-apps/api/core";

export interface InstalledBrowser {
	id: string
	name: string
	path: string
	is_available: boolean
}

export interface CampaignPayload {
	name: string
	target_urls: string[]
	traffic_type: "Direct" | "Organic" | "Referral"
	traffic_source: string
	search_engines?: string[]
	referral_sources?: string[]
	custom_referral_url?: string
	keywords: string
	keyword_list?: string[]
	utm_campaign?: string
	utm_medium?: string
	utm_source?: string
	headless: boolean
	dwell_time_seconds: number
	allow_surfing: boolean
	proxies: string[]
	max_sessions: number
	traffic_mode?: string
	session_duration_type?: string
	session_duration_constant?: number
	session_duration_min?: number
	session_duration_max?: number
	traffic_delay_type?: string
	traffic_delay_constant?: number
	traffic_delay_min?: number
	traffic_delay_max?: number
	page_order?: string
	proxy_switching?: string
	use_random_proxy?: boolean
	platform_switching?: string
	special_mode?: string
	surfing_count?: number
	client_timestamp?: string

	// Engine & Browser Borrowing
	engine_type?: "system_browser" | "native_light"
	browser_channel?: string
	custom_browser_path?: string
}

export interface ProxyTestResult {
	address: string
	is_alive: boolean
	latency_ms: number
	error_msg?: string
}

export interface BrowserStackPlan {
	automate_plan?: string
	parallel_sessions_running?: number
	team_parallel_sessions_max_allowed?: number
	status: string
}

export interface CampaignMetrics {
	total_visits: number
	successful_visits: number
	failed_visits: number
	active_visitors: number
	current_status: string
}

export interface EngineStatus {
	is_running: boolean
	metrics: CampaignMetrics
	logs?: string[]
}

export function useTrafficBackend() {
	const isTauri = computed(() => {
		return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
	});

	const startCampaign = async (payload: CampaignPayload): Promise<string> => {
		if (isTauri.value) {
			return await invoke<string>("start_campaign", { payload });
		}
		// In-browser mock fallback
		return `[Simulated] Campaign '${payload.name}' started in browser mode`;
	};

	const stopCampaign = async (): Promise<void> => {
		if (isTauri.value) {
			await invoke<void>("stop_campaign");
		}
	};

	const testProxyList = async (proxies: string[], testUrl: string): Promise<ProxyTestResult[]> => {
		if (isTauri.value) {
			return await invoke<ProxyTestResult[]>("test_proxy_list", { proxies, testUrl });
		}
		// Browser fallback simulation
		return proxies.map((p, idx) => ({
			address: p,
			is_alive: idx % 4 !== 0,
			latency_ms: 120 + Math.floor(Math.random() * 450)
		}));
	};

	const fetchProxiesFromUrl = async (url: string): Promise<string[]> => {
		if (isTauri.value) {
			return await invoke<string[]>("fetch_proxies_from_url", { url });
		}
		return [
			"104.248.63.15:3128",
			"198.199.86.11:8080",
			"165.227.215.71:3128",
			"159.203.61.169:8080"
		];
	};

	const testBrowserStackCredentials = async (
		username: string,
		accessKey: string
	): Promise<BrowserStackPlan> => {
		if (isTauri.value) {
			return await invoke<BrowserStackPlan>("test_browserstack_credentials", {
				username,
				accessKey
			});
		}
		// Browser fallback simulation
		return {
			automate_plan: "Automate Pro (Simulated)",
			parallel_sessions_running: 1,
			team_parallel_sessions_max_allowed: 5,
			status: "Active (Mock)"
		};
	};

	const fetchSitemapUrls = async (targetDomain: string): Promise<string[]> => {
		if (isTauri.value) {
			return await invoke<string[]>("fetch_sitemap_urls", { targetDomain });
		}
		const d = targetDomain.replace(/\/$/, "");
		return [
			`${d}/`,
			`${d}/about`,
			`${d}/pricing`,
			`${d}/features`,
			`${d}/contact`,
			`${d}/blog`
		];
	};

	const getEngineStatus = async (): Promise<EngineStatus> => {
		if (isTauri.value) {
			return await invoke<EngineStatus>("get_engine_status");
		}
		return {
			is_running: false,
			metrics: {
				total_visits: 0,
				successful_visits: 0,
				failed_visits: 0,
				active_visitors: 0,
				current_status: "Ready (Mock)"
			}
		};
	};

	const clearCampaignLogs = async (): Promise<void> => {
		if (isTauri.value) {
			try {
				await invoke("clear_campaign_logs");
			} catch (e) {
				console.warn("clear_campaign_logs failed:", e);
			}
		}
	};

	const getInstalledBrowsers = async (): Promise<InstalledBrowser[]> => {
		if (isTauri.value) {
			return await invoke<InstalledBrowser[]>("get_installed_browsers");
		}
		return [
			{ id: "msedge", name: "Microsoft Edge (Bawaan Windows)", path: "C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe", is_available: true },
			{ id: "chrome", name: "Google Chrome", path: "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe", is_available: true },
			{ id: "brave", name: "Brave Browser", path: "C:\\Program Files\\BraveSoftware\\Brave-Browser\\Application\\brave.exe", is_available: false },
			{ id: "firefox", name: "Mozilla Firefox", path: "C:\\Program Files\\Mozilla Firefox\\firefox.exe", is_available: false }
		];
	};

	return {
		isTauri,
		startCampaign,
		stopCampaign,
		testProxyList,
		fetchProxiesFromUrl,
		testBrowserStackCredentials,
		fetchSitemapUrls,
		getEngineStatus,
		clearCampaignLogs,
		getInstalledBrowsers
	};
}
