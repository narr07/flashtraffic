<script setup lang="ts">
import { useTrafficBackend } from "~/composables/useTrafficBackend";
import { useTrafficState } from "~/composables/useTrafficState";

const { activeCampaign, proxies, liveMetrics, isRunning, logs, torpedoRawLogs, addLog, clearLogs } = useTrafficState();
const backend = useTrafficBackend();

const isStarting = ref(false);
const logViewMode = ref<"torpedo" | "events">("torpedo");
const isCopied = ref(false);

const averageDwellTimeDisplay = computed(() => {
	if (activeCampaign.value.sessionDurationType === "constant") {
		return (activeCampaign.value.sessionDurationConstant / 1000).toFixed(1).replace(/\.0$/, "");
	}
	const avgMs = (activeCampaign.value.sessionDurationMin + activeCampaign.value.sessionDurationMax) / 2;
	return (avgMs / 1000).toFixed(1).replace(/\.0$/, "");
});

const isFastModeActive = computed(() => {
	const avgMs = activeCampaign.value.sessionDurationType === "constant"
		? activeCampaign.value.sessionDurationConstant
		: (activeCampaign.value.sessionDurationMin + activeCampaign.value.sessionDurationMax) / 2;
	return avgMs <= 1500 || activeCampaign.value.trafficMode === "max_bounce" || activeCampaign.value.trafficMode === "max_sessions";
});

const averageDwellTimeSubtitle = computed(() => {
	if (isFastModeActive.value) {
		return "Fast Mode: Instant New Session (No scroll)";
	}
	return "Scroll & read pauses active";
});

const copyTorpedoLogs = async () => {
	const text = torpedoRawLogs.value.join("\n");
	if (!text) return;
	try {
		await navigator.clipboard.writeText(text);
		isCopied.value = true;
		setTimeout(() => (isCopied.value = false), 2000);
	} catch (e) {
		console.warn("Clipboard write failed:", e);
	}
};

const downloadTorpedoLogs = () => {
	const text = torpedoRawLogs.value.join("\n");
	if (!text) return;
	const blob = new Blob([text], { type: "text/plain;charset=utf-8" });
	const url = URL.createObjectURL(blob);
	const a = document.createElement("a");
	a.href = url;
	a.download = `flashtraffic-pro-log-${new Date().toISOString().slice(0, 10)}.log`;
	document.body.appendChild(a);
	a.click();
	document.body.removeChild(a);
	URL.revokeObjectURL(url);
};

const handleClearLogs = async () => {
	clearLogs();
	try {
		await backend.clearCampaignLogs();
	} catch (e) {
		console.warn("Failed to clear backend logs:", e);
	}
};
const successRate = computed(() =>
	liveMetrics.value.totalVisits
		? Math.round((liveMetrics.value.successfulVisits / liveMetrics.value.totalVisits) * 100)
		: 100
);

const quotaProgress = computed(() =>
	activeCampaign.value.maxSessions
		? Math.min(100, Math.round((liveMetrics.value.successfulVisits / activeCampaign.value.maxSessions) * 100))
		: 0
);

const kpis = computed(() => [
	{
		label: "Total Hits",
		icon: "i-lucide-bar-chart-2",
		color: "text-primary",
		value: liveMetrics.value.successfulVisits,
		suffix: `/ ${activeCampaign.value.maxSessions}`,
		sub: `${liveMetrics.value.failedVisits} failed / bounced`,
		hint: "Total completed browser sessions relative to campaign target quota."
	},
	{
		label: "Concurrent Visitors",
		icon: "i-lucide-users",
		color: "text-info",
		value: liveMetrics.value.activeVisitors,
		sub: "Simulated human sessions",
		hint: "Number of browser workers actively simulating visitors in real-time."
	},
	{
		label: "Success Rate",
		icon: "i-lucide-check-circle-2",
		color: "text-success",
		value: `${successRate.value}%`,
		sub: "DOM Content Loaded ratio",
		hint: "Ratio of sessions reaching DOMContentLoaded without timeouts or network errors."
	},
	{
		label: "Avg. Dwell Time",
		icon: "i-lucide-timer",
		color: "text-warning",
		value: `${averageDwellTimeDisplay.value}s`,
		sub: averageDwellTimeSubtitle.value,
		hint: "Active duration spent per page, including natural reading pauses and scroll steps."
	}
]);

const logTabs = [
	{ label: "Console", value: "torpedo", icon: "i-lucide-terminal" },
	{ label: "Events", value: "events", icon: "i-lucide-list" }
];

const logClass = (line: string) => {
	if (line.startsWith("#####") || line.startsWith("\t#####")) return "text-primary font-bold";
	if (line.startsWith("===") || line.startsWith("---")) return "text-dimmed";
	if (line.startsWith("Configuration Details")) return "text-info font-semibold";
	if (line.startsWith("[Traffic]:")) return "text-warning font-bold mt-2";
	if (line.startsWith("Visual Window opened")) return "text-success font-semibold";
	if (line.startsWith("Current session duration:")) return "text-primary font-medium";
	if (line.startsWith("Current platform:") || line.startsWith("Current proxy:")) return "text-muted";
	return "text-default";
};

const config = [
	{
		label: "Engine",
		value: () => (activeCampaign.value.engineType === "native_light" ? "Ultra-Lightweight Rust" : `System (${activeCampaign.value.browserChannel === "msedge" ? "Edge" : activeCampaign.value.browserChannel === "chrome" ? "Chrome" : activeCampaign.value.browserChannel === "brave" ? "Brave" : activeCampaign.value.browserChannel === "custom" ? "Custom" : "Edge"})`),
		hint: "Selected execution engine"
	},
	{
		label: "Mode",
		value: () => (activeCampaign.value.engineType === "native_light" ? "Silent (Background)" : (activeCampaign.value.headless ? "Headless (Silent)" : "Visual Window (Terbuka)")),
		hint: "Headless runs in background; Visual Window opens real browser window."
	},
	{ label: "Physics", value: () => "Bezier & Jitter", hint: "Randomized cubic Bézier cursor curves with natural human micro-jitter." },
	{ label: "Internal Surfing", value: () => (activeCampaign.value.allowSurfing ? "Enabled" : "Disabled"), hint: "Discovers and clicks internal domain links to increase pageviews." },
	{ label: "Referrer", value: () => activeCampaign.value.trafficSource, hint: "Injected organic search referrer headers and keyword parameters." }
];
const handleStart = async () => {
	if (isRunning.value) return;
	isStarting.value = true;
	addLog("INFO", `Initializing campaign '${activeCampaign.value.name}'...`);

	try {
		const now = new Date();
		const pad = (n: number) => n.toString().padStart(2, "0");
		const dateStr = `${pad(now.getDate())}/${pad(now.getMonth() + 1)}/${now.getFullYear()} ${pad(now.getHours())}:${pad(now.getMinutes())}:${pad(now.getSeconds())}`;

		const res = await backend.startCampaign({
			name: activeCampaign.value.name,
			target_urls: activeCampaign.value.targetUrls,
			traffic_type: activeCampaign.value.trafficType,
			traffic_source: activeCampaign.value.trafficSource,
			search_engines: activeCampaign.value.searchEngines,
			referral_sources: activeCampaign.value.referralSources,
			custom_referral_url: activeCampaign.value.customReferralUrl,
			keywords: activeCampaign.value.keywords,
			keyword_list: activeCampaign.value.keywordList,
			utm_campaign: activeCampaign.value.utmCampaign,
			utm_medium: activeCampaign.value.utmMedium,
			utm_source: activeCampaign.value.utmSource,
			headless: activeCampaign.value.headless,
			dwell_time_seconds: activeCampaign.value.dwellTimeSeconds,
			allow_surfing: activeCampaign.value.allowSurfing,
			proxies: activeCampaign.value.enableProxy
				? (activeCampaign.value.selectedProxies.length > 0 ? activeCampaign.value.selectedProxies : proxies.value)
				: [],
			max_sessions: activeCampaign.value.maxSessions,
			traffic_mode: activeCampaign.value.trafficMode,
			session_duration_type: activeCampaign.value.sessionDurationType,
			session_duration_constant: activeCampaign.value.sessionDurationConstant,
			session_duration_min: activeCampaign.value.sessionDurationMin,
			session_duration_max: activeCampaign.value.sessionDurationMax,
			traffic_delay_type: activeCampaign.value.trafficDelayType,
			traffic_delay_constant: activeCampaign.value.trafficDelayConstant,
			traffic_delay_min: activeCampaign.value.trafficDelayMin,
			traffic_delay_max: activeCampaign.value.trafficDelayMax,
			page_order: activeCampaign.value.pageOrder,
			proxy_switching: activeCampaign.value.proxySwitching,
			use_random_proxy: activeCampaign.value.useRandomProxy,
			platform_switching: activeCampaign.value.platformSwitching,
			special_mode: activeCampaign.value.specialMode,
			surfing_count: activeCampaign.value.surfingCount,
			client_timestamp: dateStr,
			engine_type: activeCampaign.value.engineType,
			browser_channel: activeCampaign.value.browserChannel,
			custom_browser_path: activeCampaign.value.customBrowserPath
		});

		isRunning.value = true;
		liveMetrics.value.activeVisitors = 1;
		liveMetrics.value.statusText = "Traffic simulation in progress";
		addLog("SUCCESS", res);
	} catch (err: any) {
		addLog("ERROR", `Failed to start campaign: ${err}`);
	} finally {
		isStarting.value = false;
	}
};

const handleStop = async () => {
	addLog("WARN", "Stopping traffic engine...");
	try {
		await backend.stopCampaign();
		isRunning.value = false;
		liveMetrics.value.activeVisitors = 0;
		liveMetrics.value.statusText = "Simulation stopped";
		addLog("INFO", "Traffic simulation terminated cleanly.");
	} catch (err: any) {
		addLog("ERROR", `Failed to stop campaign: ${err}`);
	}
};

// Periodic status poll
let interval: any = null;
onMounted(() => {
	interval = setInterval(async () => {
		try {
			const status = await backend.getEngineStatus();
			if (status.logs && status.logs.length > 0) {
				torpedoRawLogs.value = status.logs;
			}
			if (isRunning.value) {
				liveMetrics.value.totalVisits = status.metrics.total_visits;
				liveMetrics.value.successfulVisits = status.metrics.successful_visits;
				liveMetrics.value.failedVisits = status.metrics.failed_visits;
				liveMetrics.value.activeVisitors = status.metrics.active_visitors;
				if (!status.is_running && isRunning.value && status.metrics.total_visits > 0) {
					isRunning.value = false;
					addLog("SUCCESS", "Campaign completed all targeted sessions.");
				}
			}
		} catch {
			// Ignore polling error in dev mode
		}
	}, 1200);
});

onUnmounted(() => {
	if (interval) clearInterval(interval);
});
</script>

<template>
	<div class="space-y-6 max-w-7xl mx-auto">
		<!-- Teleport Title to Row 1 Navbar -->
		<AppHeaderTitle>
			<div class="flex items-center gap-2">
				<span class="text-muted">Platform</span>
				<span class="text-muted/60">/</span>
				<span class="flex items-center gap-1.5 font-bold text-highlighted">
					<UIcon name="i-lucide-layout-dashboard" class="size-4 text-primary" />
					Overview Dashboard
				</span>
			</div>
		</AppHeaderTitle>

		<!-- Hero header -->
		<UCard variant="subtle">
			<div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
				<div class="space-y-1.5 min-w-0">
					<div class="flex items-center gap-2 flex-wrap">
						<h2 class="text-xl font-bold text-highlighted truncate">{{ activeCampaign.name }}</h2>
						<UBadge :color="isRunning ? 'success' : 'neutral'" variant="subtle" size="sm">
							<span class="size-1.5 rounded-full bg-current" :class="{ 'animate-pulse': isRunning }" />
							{{ isRunning ? 'Running' : 'Idle' }}
						</UBadge>
						<UBadge color="primary" variant="outline" size="sm">{{ activeCampaign.trafficType }}</UBadge>
					</div>
					<p class="text-xs text-muted font-mono truncate">
						{{ activeCampaign.targetUrls[0] || 'No URL configured' }}
						<span v-if="activeCampaign.trafficType === 'Organic'"> · "{{ activeCampaign.keywords }}"</span>
					</p>
				</div>

				<div class="flex items-center gap-2">
					<UButton v-if="!isRunning" icon="i-lucide-play" :loading="isStarting" @click="handleStart">
						Start Campaign
					</UButton>
					<UButton v-else color="error" icon="i-lucide-square" @click="handleStop">Stop Engine</UButton>
					<UButton to="/campaigns" variant="outline" color="neutral" icon="i-lucide-sliders-horizontal">
						Tune Settings
					</UButton>
				</div>
			</div>

			<div class="mt-4 space-y-1">
				<div class="flex justify-between text-xs text-muted">
					<span>Quota progress</span>
					<span class="font-mono">{{ quotaProgress }}%</span>
				</div>
				<UProgress v-model="quotaProgress" size="sm" :animation="isRunning ? 'carousel' : undefined" />
			</div>
		</UCard>

		<!-- KPI -->
		<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
			<UCard v-for="kpi in kpis" :key="kpi.label" variant="subtle" class="hover:ring-primary/40 transition">
				<div class="flex items-center justify-between text-xs text-muted mb-3">
					<span class="flex items-center gap-1.5">
						{{ kpi.label }}
						<UTooltip :text="kpi.hint">
							<UIcon name="i-lucide-info" class="size-3 text-dimmed cursor-help" />
						</UTooltip>
					</span>
					<div class="p-1.5 rounded-md bg-elevated">
						<UIcon :name="kpi.icon" class="size-4" :class="kpi.color" />
					</div>
				</div>
				<div class="text-2xl font-bold font-mono text-highlighted">
					{{ kpi.value }}
					<span v-if="kpi.suffix" class="text-xs font-normal text-dimmed">{{ kpi.suffix }}</span>
				</div>
				<p class="mt-1 text-xs text-muted">{{ kpi.sub }}</p>
			</UCard>
		</div>

		<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
			<!-- Console -->
			<UCard
				class="lg:col-span-2"
				variant="subtle"
				:ui="{ root: 'flex flex-col h-[480px]', body: 'flex-1 min-h-0 !p-0 sm:!p-0' }"
			>
				<template #header>
					<div class="flex flex-wrap items-center justify-between gap-3">
						<div class="flex items-center gap-2">
							<UIcon name="i-lucide-terminal" class="size-4 text-primary" />
							<h3 class="text-sm font-semibold text-highlighted">Execution Console</h3>
							<UBadge size="xs" variant="subtle">
								{{ logViewMode === 'torpedo' ? `${torpedoRawLogs.length} lines` : `${logs.length} events` }}
							</UBadge>
						</div>
						<div class="flex items-center gap-1.5">
							<UTabs v-model="logViewMode" :items="logTabs" :content="false" size="xs" variant="pill" />
							<UTooltip :text="isCopied ? 'Copied!' : 'Copy log'">
								<UButton size="xs" variant="ghost" color="neutral" :icon="isCopied ? 'i-lucide-check' : 'i-lucide-copy'" @click="copyTorpedoLogs" />
							</UTooltip>
							<UTooltip text="Download .log">
								<UButton size="xs" variant="ghost" color="neutral" icon="i-lucide-download" @click="downloadTorpedoLogs" />
							</UTooltip>
							<UTooltip text="Clear logs">
								<UButton size="xs" variant="ghost" color="error" icon="i-lucide-trash" @click="handleClearLogs" />
							</UTooltip>
						</div>
					</div>
				</template>

				<div class="h-full overflow-y-auto p-4 font-mono text-[11px] leading-relaxed select-text">
					<template v-if="logViewMode === 'torpedo'">
						<div v-if="torpedoRawLogs.length" class="space-y-0.5">
							<div v-for="(line, idx) in torpedoRawLogs" :key="idx" :class="logClass(line)">{{ line }}</div>
						</div>
						<UEmpty
							v-else
							icon="i-lucide-terminal"
							title="Console log is empty"
							description="Click Start Campaign above to stream real-time execution logs."
						/>
					</template>

					<template v-else>
						<div v-if="logs.length" class="space-y-1">
							<div v-for="log in logs" :key="log.id" class="flex items-start gap-2.5 py-1 px-2 rounded hover:bg-elevated">
								<span class="text-dimmed shrink-0">{{ log.time }}</span>
								<UBadge
									size="xs"
									variant="subtle"
									class="shrink-0"
									:color="log.level === 'SUCCESS' ? 'success' : log.level === 'WARN' ? 'warning' : log.level === 'ERROR' ? 'error' : 'neutral'"
								>
									{{ log.level }}
								</UBadge>
								<span class="text-default break-all">{{ log.text }}</span>
							</div>
						</div>
						<UEmpty v-else icon="i-lucide-list" title="No events recorded" description="Start a campaign to stream events in real-time." />
					</template>
				</div>
			</UCard>

			<!-- Simulator -->
			<UCard variant="subtle" :ui="{ root: 'h-[480px] flex flex-col', body: 'flex-1 overflow-y-auto' }">
				<template #header>
					<div class="flex items-center gap-2">
						<UIcon name="i-lucide-monitor" class="size-4 text-muted" />
						<h3 class="text-sm font-semibold text-highlighted">Browser Simulator</h3>
					</div>
				</template>

				<div class="space-y-4">
					<ul class="divide-y divide-default rounded-lg border border-default">
						<li v-for="item in config" :key="item.label" class="flex items-center justify-between px-3 py-2.5 text-xs">
							<span class="flex items-center gap-1 text-muted">
								{{ item.label }}
								<UTooltip :text="item.hint">
									<UIcon name="i-lucide-info" class="size-3 text-dimmed cursor-help" />
								</UTooltip>
							</span>
							<span class="font-medium text-highlighted">{{ item.value() }}</span>
						</li>
					</ul>

					<UAlert
						color="primary"
						variant="subtle"
						icon="i-lucide-shield-check"
						title="Stealth Antidetect Active"
						description="navigator.webdriver is masked. Viewport, User-Agent, and Client Hints match native desktop profiles."
					/>
				</div>
			</UCard>
		</div>
	</div>
</template>
