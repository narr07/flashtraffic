import { invoke } from "@tauri-apps/api/core";

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
		// Browser fallback simulation
		return [
			"130.110.103.245:3128",
			"3.211.120.181:443",
			"103.237.102.191:11111",
			"176.111.37.216:39811",
			"78.186.146.140:3310"
		];
	};

	const testBrowserStackCredentials = async (username: string, accessKey: string): Promise<BrowserStackPlan> => {
		if (isTauri.value) {
			return await invoke<BrowserStackPlan>("test_browserstack_credentials", {
				username,
				accessKey
			});
		}
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

	return {
		isTauri,
		startCampaign,
		stopCampaign,
		testProxyList,
		fetchProxiesFromUrl,
		testBrowserStackCredentials,
		fetchSitemapUrls,
		getEngineStatus,
		clearCampaignLogs
	};
}
