<script setup lang="ts">
import { useTrafficBackend } from "~/composables/useTrafficBackend";
import { useTrafficState } from "~/composables/useTrafficState";

const { proxies, activeCampaign, addLog } = useTrafficState();
const backend = useTrafficBackend();
const toast = useToast();

interface ProxyRow {
	address: string
	protocol: "HTTP" | "HTTPS" | "SOCKS4" | "SOCKS5"
	credentials?: string
	latency: number | null
	status: "untested" | "alive" | "dead"
}

// Ensure activeCampaign defaults
if (activeCampaign.value.enableProxy === undefined) {
	activeCampaign.value.enableProxy = false;
}
if (!activeCampaign.value.selectedProxies) {
	activeCampaign.value.selectedProxies = [];
}

const proxyRows = ref<ProxyRow[]>(
	proxies.value.map((p: string) => {
		const parts = p.split(":");
		return {
			address: parts.slice(0, 2).join(":"),
			protocol: p.startsWith("socks5://") ? "SOCKS5" : p.startsWith("https://") ? "HTTPS" : "HTTP",
			credentials: parts.length >= 4 ? `${parts[2]}:••••` : undefined,
			latency: null,
			status: "untested"
		};
	})
);

// Selection state
const selectedAddresses = ref<string[]>([]);
const searchFilter = ref("");
const statusFilter = ref<"all" | "alive" | "dead" | "fast">("all");

// Add modes: 'url' | 'manual' | 'paste'
const addMode = ref<"url" | "manual" | "paste">("url");
const addModeItems = [
	{ value: "url", label: "URL / Text File", description: "Fetch free proxy lists or files" },
	{ value: "manual", label: "One by One", description: "Specific IP:Port & credentials" },
	{ value: "paste", label: "Raw Bulk Paste", description: "Paste raw proxy lines" }
];
const addModeUi = {
	fieldset: "grid grid-cols-1 sm:grid-cols-3 gap-2.5",
	item: "p-2.5 cursor-pointer",
	label: "text-xs font-semibold",
	description: "text-[11px]"
};

// URL / File input
const proxySourceUrl = ref("https://proxora.io/free-proxy-list");
const isFetchingUrl = ref(false);
const fileInputRef = ref<HTMLInputElement | null>(null);

// Manual one-by-one input
const manualIpPort = ref("");
const manualUser = ref("");
const manualPass = ref("");
const manualProtocol = ref<"HTTP" | "HTTPS" | "SOCKS4" | "SOCKS5">("HTTP");

// Bulk paste input
const bulkTextInput = ref("");

// Test settings (Torpedo style)
const testTargetMode = ref<"default" | "custom">("default");
const customTestUrl = ref("https://httpbin.org/ip");
const isTesting = ref(false);

// Filtered proxy rows
const filteredRows = computed(() => {
	let list = proxyRows.value;

	if (searchFilter.value.trim()) {
		const q = searchFilter.value.toLowerCase();
		list = list.filter(r => r.address.toLowerCase().includes(q) || (r.credentials && r.credentials.toLowerCase().includes(q)));
	}

	if (statusFilter.value === "alive") {
		list = list.filter(r => r.status === "alive");
	} else if (statusFilter.value === "dead") {
		list = list.filter(r => r.status === "dead");
	} else if (statusFilter.value === "fast") {
		list = list.filter(r => r.status === "alive" && r.latency !== null && r.latency < 1500);
	}

	return list;
});

const isAllFilteredSelected = computed(() => {
	if (filteredRows.value.length === 0) return false;
	return filteredRows.value.every(r => selectedAddresses.value.includes(r.address));
});

const isSomeFilteredSelected = computed(() => {
	if (filteredRows.value.length === 0) return false;
	return filteredRows.value.some(r => selectedAddresses.value.includes(r.address)) && !isAllFilteredSelected.value;
});

const toggleSelectAll = () => {
	if (isAllFilteredSelected.value) {
		const filteredAddrs = filteredRows.value.map(r => r.address);
		selectedAddresses.value = selectedAddresses.value.filter(a => !filteredAddrs.includes(a));
	} else {
		const newSet = new Set([...selectedAddresses.value, ...filteredRows.value.map(r => r.address)]);
		selectedAddresses.value = Array.from(newSet);
	}
};

const toggleSelectRow = (addr: string) => {
	if (selectedAddresses.value.includes(addr)) {
		selectedAddresses.value = selectedAddresses.value.filter(a => a !== addr);
	} else {
		selectedAddresses.value.push(addr);
	}
};

// ========================
// Proxy Addition Methods
// ========================
const syncProxiesState = () => {
	proxies.value = proxyRows.value.map(r => r.address);
};

// 1. Fetch from URL (e.g. proxora.io/free-proxy-list)
const handleFetchFromUrl = async () => {
	const url = proxySourceUrl.value.trim();
	if (!url) return;

	isFetchingUrl.value = true;
	addLog("INFO", `Fetching and extracting proxies from: ${url}...`);

	try {
		const extracted = await backend.fetchProxiesFromUrl(url);
		let addedCount = 0;

		for (const p of extracted) {
			if (!proxyRows.value.some(r => r.address === p)) {
				proxyRows.value.push({
					address: p,
					protocol: "HTTP",
					latency: null,
					status: "untested"
				});
				addedCount++;
			}
		}

		syncProxiesState();
		addLog("SUCCESS", `Extracted ${extracted.length} proxies from URL, ${addedCount} new nodes added.`);
		toast.add({
			title: "Proxies Extracted Successfully",
			description: `Found ${extracted.length} nodes from URL (${addedCount} newly added to pool).`,
			color: "success",
			icon: "i-lucide-check-circle"
		});
	} catch (err: any) {
		addLog("ERROR", `Failed to fetch proxies: ${err}`);
		toast.add({
			title: "Proxy Extraction Failed",
			description: String(err),
			color: "error",
			icon: "i-lucide-alert-triangle"
		});
	} finally {
		isFetchingUrl.value = false;
	}
};

// 2. Import from local text file (.txt, .csv)
const triggerFileInput = () => {
	fileInputRef.value?.click();
};

const handleFileImport = (event: Event) => {
	const target = event.target as HTMLInputElement;
	const file = target.files?.[0];
	if (!file) return;

	const reader = new FileReader();
	reader.onload = (e) => {
		const text = e.target?.result as string;
		if (!text) return;

		const lines = text.split(/\r?\n/);
		let addedCount = 0;

		for (const line of lines) {
			const trimmed = line.trim();
			// match IP:PORT pattern
			const match = trimmed.match(/\b(?:\d{1,3}\.){3}\d{1,3}:\d{2,5}\b/);
			if (match) {
				const addr = match[0];
				if (!proxyRows.value.some(r => r.address === addr)) {
					const parts = trimmed.split(":");
					proxyRows.value.push({
						address: addr,
						protocol: "HTTP",
						credentials: parts.length >= 4 ? `${parts[2]}:••••` : undefined,
						latency: null,
						status: "untested"
					});
					addedCount++;
				}
			}
		}

		syncProxiesState();
		toast.add({
			title: "Proxies Imported from File",
			description: `Imported ${addedCount} proxies from '${file.name}'.`,
			color: "success",
			icon: "i-lucide-file-text"
		});
		target.value = "";
	};
	reader.readAsText(file);
};

// 3. Add one by one
const handleAddManual = () => {
	const trimmed = manualIpPort.value.trim();
	if (!trimmed) return;

	if (proxyRows.value.some(r => r.address === trimmed)) {
		toast.add({
			title: "Proxy Already Exists",
			description: `Endpoint ${trimmed} is already in the pool.`,
			color: "warning"
		});
		return;
	}

	proxyRows.value.unshift({
		address: trimmed,
		protocol: manualProtocol.value,
		credentials: manualUser.value ? `${manualUser.value}:••••` : undefined,
		latency: null,
		status: "untested"
	});

	syncProxiesState();
	manualIpPort.value = "";
	manualUser.value = "";
	manualPass.value = "";

	toast.add({
		title: "Proxy Added",
		description: trimmed,
		color: "success",
		icon: "i-lucide-plus"
	});
};

// 4. Raw text paste
const handleBulkPaste = () => {
	const lines = bulkTextInput.value.split(/\r?\n/);
	let added = 0;

	for (const line of lines) {
		const match = line.trim().match(/\b(?:\d{1,3}\.){3}\d{1,3}:\d{2,5}\b/);
		if (match) {
			const addr = match[0];
			if (!proxyRows.value.some(r => r.address === addr)) {
				proxyRows.value.push({
					address: addr,
					protocol: "HTTP",
					latency: null,
					status: "untested"
				});
				added++;
			}
		}
	}

	syncProxiesState();
	bulkTextInput.value = "";
	addMode.value = "url";
	toast.add({
		title: "Bulk Import Done",
		description: `Added ${added} proxies from text paste.`,
		color: "success"
	});
};

// ========================
// Testing & Ping Actions
// ========================
const executeTest = async (addressesToTest: string[]) => {
	if (addressesToTest.length === 0) return;
	isTesting.value = true;

	const testUrl = testTargetMode.value === "custom" && customTestUrl.value.trim()
		? customTestUrl.value.trim()
		: "https://httpbin.org/ip";

	addLog("INFO", `Pinging ${addressesToTest.length} proxies against ${testUrl}...`);

	try {
		const results = await backend.testProxyList(addressesToTest, testUrl);
		for (const res of results) {
			const found = proxyRows.value.find(r => r.address === res.address);
			if (found) {
				found.latency = res.latency_ms;
				found.status = res.is_alive ? "alive" : "dead";
			}
		}

		const aliveCount = results.filter(r => r.is_alive).length;
		addLog("SUCCESS", `Proxy testing finished: ${aliveCount}/${addressesToTest.length} healthy.`);
		toast.add({
			title: "Proxy Latency Ping Completed",
			description: `${aliveCount} alive, ${addressesToTest.length - aliveCount} dead or unreachable.`,
			color: aliveCount > 0 ? "success" : "warning",
			icon: "i-lucide-activity"
		});
	} catch (err: any) {
		addLog("ERROR", `Proxy test failed: ${err}`);
		toast.add({
			title: "Proxy Test Error",
			description: String(err),
			color: "error"
		});
	} finally {
		isTesting.value = false;
	}
};

const handleTestAll = () => {
	executeTest(proxyRows.value.map(r => r.address));
};

const handleTestSelected = () => {
	executeTest(selectedAddresses.value);
};

const handleTestSingle = (addr: string) => {
	executeTest([addr]);
};

// ========================
// Deletion & Cleaning Actions
// ========================
const removeSingleProxy = (addr: string) => {
	proxyRows.value = proxyRows.value.filter(r => r.address !== addr);
	selectedAddresses.value = selectedAddresses.value.filter(a => a !== addr);
	syncProxiesState();
};

const removeSelectedProxies = () => {
	const count = selectedAddresses.value.length;
	if (count === 0) return;

	proxyRows.value = proxyRows.value.filter(r => !selectedAddresses.value.includes(r.address));
	selectedAddresses.value = [];
	syncProxiesState();

	toast.add({
		title: "Selected Proxies Removed",
		description: `Deleted ${count} proxy nodes from list.`,
		color: "neutral",
		icon: "i-lucide-trash-2"
	});
};

const removeDeadProxies = () => {
	const deadCount = proxyRows.value.filter(r => r.status === "dead").length;
	if (deadCount === 0) {
		toast.add({
			title: "No Dead Proxies",
			description: "There are no proxies marked as dead.",
			color: "neutral"
		});
		return;
	}

	proxyRows.value = proxyRows.value.filter(r => r.status !== "dead");
	selectedAddresses.value = selectedAddresses.value.filter(a => proxyRows.value.some(r => r.address === a));
	syncProxiesState();

	toast.add({
		title: "Dead Proxies Pruned",
		description: `Cleaned ${deadCount} dead/unresponsive proxy endpoints.`,
		color: "success",
		icon: "i-lucide-shield-check"
	});
};

const clearAllProxies = () => {
	const count = proxyRows.value.length;
	if (count === 0) return;

	proxyRows.value = [];
	selectedAddresses.value = [];
	syncProxiesState();

	toast.add({
		title: "Proxy Pool Cleared",
		description: `Removed all ${count} proxies.`,
		color: "warning",
		icon: "i-lucide-trash"
	});
};

// ========================
// Sync to Active Campaign
// ========================
const setActiveCampaignProxies = () => {
	const targets = selectedAddresses.value.length > 0
		? selectedAddresses.value
		: proxyRows.value.filter(r => r.status === "alive").map(r => r.address);

	if (targets.length === 0) {
		toast.add({
			title: "No Proxies to Assign",
			description: "Please select proxies or run tests to identify alive proxies.",
			color: "warning"
		});
		return;
	}

	activeCampaign.value.selectedProxies = [...targets];
	activeCampaign.value.enableProxy = true;

	toast.add({
		title: "Campaign Proxies Updated",
		description: `Assigned ${targets.length} proxies to Active Campaign with routing ENABLED.`,
		color: "success",
		icon: "i-lucide-check-circle-2"
	});
};
</script>

<template>
	<div class="max-w-5xl mx-auto space-y-6">
		<!-- Teleport Title to Row 1 Navbar -->
		<AppHeaderTitle>
			<div class="flex items-center gap-2">
				<span class="text-muted">Proxies</span>
				<span class="text-muted/60">/</span>
				<span class="flex items-center gap-1.5 font-bold text-highlighted">
					<UIcon name="i-lucide-shield-check" class="size-4 text-primary" />
					Proxy Orchestrator
				</span>
			</div>
		</AppHeaderTitle>

		<!-- Teleport Actions to Row 1 Navbar -->
		<AppHeaderActions>
			<UButton
				size="xs"
				color="primary"
				icon="i-lucide-zap"
				@click="setActiveCampaignProxies"
			>
				Set {{ selectedAddresses.length > 0 ? `${selectedAddresses.length} Selected` : 'Alive' }} to Campaign
			</UButton>
		</AppHeaderActions>

		<!-- Teleport Toolbar to Row 2 Sub-Navbar -->
		<AppToolbar>
			<div class="flex items-center gap-3">
				<div class="flex items-center gap-2">
					<span class="text-xs font-medium text-highlighted">Enable Proxy:</span>
					<USwitch v-model="activeCampaign.enableProxy" size="sm" />
				</div>
				<UBadge
					size="xs"
					:color="activeCampaign.enableProxy ? 'success' : 'neutral'"
					variant="subtle"
				>
					{{ activeCampaign.enableProxy ? 'Routing Active' : 'Direct (No Proxy)' }}
				</UBadge>
			</div>

			<div class="flex items-center gap-2 font-mono text-[11px] text-muted">
				<span>Pool: <strong class="text-highlighted">{{ proxyRows.length }}</strong> Nodes</span>
				<span>•</span>
				<span>Alive: <strong class="text-success">{{ proxyRows.filter(r => r.status === 'alive').length }}</strong></span>
				<span>•</span>
				<span>Selected: <strong class="text-primary">{{ selectedAddresses.length }}</strong></span>
			</div>
		</AppToolbar>

		<!-- Torpedo Style: Add Proxy Sources (URL / File / Manual) -->
		<UCard variant="subtle">
			<template #header>
				<div class="flex items-center gap-2">
					<UIcon name="i-lucide-download-cloud" class="text-primary size-4" />
					<h3 class="text-sm font-semibold text-highlighted">Proxy Source & Ingestion</h3>
				</div>
			</template>

			<div class="space-y-4">
				<URadioGroup
					v-model="addMode"
					:items="addModeItems"
					variant="card"
					orientation="horizontal"
					:ui="addModeUi"
				/>

				<!-- Hidden File Input for .txt/.csv -->
				<input
					ref="fileInputRef"
					type="file"
					accept=".txt,.csv"
					class="hidden"
					@change="handleFileImport"
				>

				<!-- 1. URL / File Source (Torpedo style) -->
				<div v-if="addMode === 'url'" class="space-y-4">
					<div class="flex flex-col sm:flex-row items-stretch sm:items-center gap-2">
						<UInput
							v-model="proxySourceUrl"
							placeholder="https://proxora.io/free-proxy-list"
							icon="i-lucide-globe"
							class="flex-1"
							:disabled="isFetchingUrl"
							@keydown.enter="handleFetchFromUrl"
						/>
						<UButton
							color="primary"
							icon="i-lucide-download"
							:loading="isFetchingUrl"
							@click="handleFetchFromUrl"
						>
							Fetch & Extract
						</UButton>
						<UButton
							variant="outline"
							color="neutral"
							icon="i-lucide-folder-open"
							@click="triggerFileInput"
						>
							Browse File (.txt, .csv)
						</UButton>
					</div>

					<!-- Quick Presets -->
					<div class="flex flex-wrap items-center gap-2 pt-1 text-xs">
						<span class="text-[11px] font-mono text-muted">Quick Proxy Sources:</span>
						<button
							type="button"
							class="px-2 py-0.5 rounded bg-elevated/60 hover:bg-elevated text-muted hover:text-highlighted transition-colors border border-default text-xs"
							@click="proxySourceUrl = 'https://fineproxy.org/id/free-proxy/'"
						>
							FineProxy (1000+)
						</button>
						<button
							type="button"
							class="px-2 py-0.5 rounded bg-elevated/60 hover:bg-elevated text-muted hover:text-highlighted transition-colors border border-default text-xs"
							@click="proxySourceUrl = 'https://roundproxies.com/free-proxy-list/'"
						>
							RoundProxies (100+)
						</button>
						<button
							type="button"
							class="px-2 py-0.5 rounded bg-elevated/60 hover:bg-elevated text-muted hover:text-highlighted transition-colors border border-default text-xs"
							@click="proxySourceUrl = 'https://www.freeproxy.world/'"
						>
							FreeProxy.world (50+)
						</button>
						<button
							type="button"
							class="px-2 py-0.5 rounded bg-elevated/60 hover:bg-elevated text-muted hover:text-highlighted transition-colors border border-default text-xs"
							@click="proxySourceUrl = 'https://proxora.io/free-proxy-list'"
						>
							Proxora.io (25+)
						</button>
						<button
							type="button"
							class="px-2 py-0.5 rounded bg-elevated/60 hover:bg-elevated text-muted hover:text-highlighted transition-colors border border-default text-xs"
							@click="proxySourceUrl = 'https://proxyscrape.com/free-proxy-list'"
						>
							ProxyScrape
						</button>
						<button
							type="button"
							class="px-2 py-0.5 rounded bg-elevated/60 hover:bg-elevated text-muted hover:text-highlighted transition-colors border border-default text-xs"
							@click="proxySourceUrl = 'https://www.proxynova.com/proxy-server-list/'"
						>
							ProxyNova
						</button>
						<button
							type="button"
							class="px-2 py-0.5 rounded bg-elevated/60 hover:bg-elevated text-muted hover:text-highlighted transition-colors border border-default text-xs"
							@click="proxySourceUrl = 'https://raw.githubusercontent.com/TheSpeedX/PROXY-List/master/http.txt'"
						>
							GitHub List
						</button>
					</div>
				</div>

				<!-- 2. Manual One by One (Torpedo style) -->
				<div v-else-if="addMode === 'manual'" class="space-y-4">
					<div class="grid grid-cols-1 sm:grid-cols-4 gap-3">
						<UFormField label="Proxy Endpoint" required hint="IP:Port">
							<UInput v-model="manualIpPort" placeholder="192.168.1.1:8080" class="w-full" />
						</UFormField>
						<UFormField label="Username (Optional)">
							<UInput v-model="manualUser" placeholder="proxy_user" class="w-full" />
						</UFormField>
						<UFormField label="Password (Optional)">
							<UInput v-model="manualPass" type="password" placeholder="••••••••" class="w-full" />
						</UFormField>
						<UFormField label="Protocol">
							<USelect
								v-model="manualProtocol"
								:items="['HTTP', 'HTTPS', 'SOCKS4', 'SOCKS5']"
								class="w-full"
							/>
						</UFormField>
					</div>
					<div class="flex justify-end">
						<UButton color="primary" icon="i-lucide-plus" @click="handleAddManual">
							Add Proxy Node
						</UButton>
					</div>
				</div>

				<!-- 3. Raw Text Paste -->
				<div v-else-if="addMode === 'paste'" class="space-y-3">
					<UTextarea
						v-model="bulkTextInput"
						:rows="5"
						placeholder="Paste proxy list (one per line, e.g. 192.168.1.1:8080 or 10.0.0.1:3128:user:pass)"
						class="font-mono text-xs w-full"
					/>
					<div class="flex justify-end">
						<UButton color="primary" icon="i-lucide-file-input" @click="handleBulkPaste">
							Import All Parsed
						</UButton>
					</div>
				</div>
			</div>
		</UCard>

		<!-- Torpedo Style: Proxy Testing Options & Filter Controls -->
		<UCard variant="subtle">
			<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
				<!-- Test Target URL Config -->
				<div class="space-y-2">
					<h4 class="text-xs font-semibold text-highlighted">Proxy Test Target URL</h4>
					<div class="flex items-center gap-2">
						<UButton
							size="xs"
							:variant="testTargetMode === 'default' ? 'solid' : 'outline'"
							:color="testTargetMode === 'default' ? 'primary' : 'neutral'"
							@click="testTargetMode = 'default'"
						>
							Default Check (httpbin.org/ip)
						</UButton>
						<UButton
							size="xs"
							:variant="testTargetMode === 'custom' ? 'solid' : 'outline'"
							:color="testTargetMode === 'custom' ? 'primary' : 'neutral'"
							@click="testTargetMode = 'custom'"
						>
							Check on Custom URL
						</UButton>
					</div>

					<UInput
						v-if="testTargetMode === 'custom'"
						v-model="customTestUrl"
						placeholder="https://example.com/target-landing-page"
						size="xs"
						icon="i-lucide-link"
						class="w-full"
					/>
				</div>

				<!-- Test Actions & Sensitivity -->
				<div class="flex flex-col justify-end space-y-2">
					<h4 class="text-xs font-semibold text-highlighted">Verification Engine</h4>
					<div class="flex flex-wrap items-center gap-2">
						<UButton
							color="primary"
							icon="i-lucide-activity"
							size="xs"
							:loading="isTesting"
							@click="handleTestAll"
						>
							Test All Latencies ({{ proxyRows.length }})
						</UButton>

						<UButton
							v-if="selectedAddresses.length > 0"
							color="neutral"
							variant="outline"
							size="xs"
							:loading="isTesting"
							@click="handleTestSelected"
						>
							Test Selected ({{ selectedAddresses.length }})
						</UButton>

						<UButton
							variant="ghost"
							color="warning"
							size="xs"
							icon="i-lucide-trash"
							@click="removeDeadProxies"
						>
							Prune Dead
						</UButton>
					</div>
				</div>
			</div>
		</UCard>

		<!-- Interactive Proxy Pool Table -->
		<UCard variant="subtle">
			<template #header>
				<div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
					<div class="flex items-center gap-2">
						<UIcon name="i-lucide-list" class="size-4 text-primary" />
						<h3 class="text-sm font-semibold text-highlighted">Configured Proxy Pool</h3>
						<UBadge size="xs" color="primary" variant="subtle">{{ proxyRows.length }} Nodes</UBadge>
						<UBadge v-if="selectedAddresses.length > 0" size="xs" color="error" variant="subtle">
							{{ selectedAddresses.length }} Selected
						</UBadge>
					</div>

					<!-- Table Actions -->
					<div class="flex flex-wrap items-center gap-2">
						<UButton
							v-if="selectedAddresses.length > 0"
							size="xs"
							color="error"
							icon="i-lucide-trash-2"
							@click="removeSelectedProxies"
						>
							Delete Selected ({{ selectedAddresses.length }})
						</UButton>

						<UButton
							v-if="proxyRows.length > 0"
							size="xs"
							variant="ghost"
							color="neutral"
							icon="i-lucide-trash"
							@click="clearAllProxies"
						>
							Clear All
						</UButton>
					</div>
				</div>
			</template>

			<div class="space-y-4">
				<!-- Filter Bar -->
				<div class="flex flex-col sm:flex-row items-stretch sm:items-center justify-between gap-3 pb-2 border-b border-default">
					<UInput
						v-model="searchFilter"
						placeholder="Filter IP or port..."
						icon="i-lucide-search"
						size="xs"
						class="max-w-xs"
					/>

					<!-- Status Pills Filter -->
					<div class="flex items-center gap-1">
						<UButton
							size="xs"
							:variant="statusFilter === 'all' ? 'solid' : 'ghost'"
							:color="statusFilter === 'all' ? 'primary' : 'neutral'"
							@click="statusFilter = 'all'"
						>
							All ({{ proxyRows.length }})
						</UButton>
						<UButton
							size="xs"
							:variant="statusFilter === 'alive' ? 'solid' : 'ghost'"
							:color="statusFilter === 'alive' ? 'success' : 'neutral'"
							@click="statusFilter = 'alive'"
						>
							Alive ({{ proxyRows.filter(r => r.status === 'alive').length }})
						</UButton>
						<UButton
							size="xs"
							:variant="statusFilter === 'dead' ? 'solid' : 'ghost'"
							:color="statusFilter === 'dead' ? 'error' : 'neutral'"
							@click="statusFilter = 'dead'"
						>
							Dead ({{ proxyRows.filter(r => r.status === 'dead').length }})
						</UButton>
						<UButton
							size="xs"
							:variant="statusFilter === 'fast' ? 'solid' : 'ghost'"
							:color="statusFilter === 'fast' ? 'warning' : 'neutral'"
							@click="statusFilter = 'fast'"
						>
							Fast &lt;1.5s
						</UButton>
					</div>
				</div>

				<!-- Proxy Table -->
				<div class="overflow-x-auto max-h-[460px] overflow-y-auto rounded-lg border border-default">
					<table class="w-full text-left text-xs font-mono">
						<thead class="sticky top-0 bg-elevated border-b border-default text-muted z-10">
							<tr>
								<th class="py-2.5 px-3 w-10">
									<UCheckbox
										:model-value="isAllFilteredSelected ? true : isSomeFilteredSelected ? 'indeterminate' : false"
										@update:model-value="toggleSelectAll"
									/>
								</th>
								<th class="py-2.5 px-3 w-12 text-muted">#</th>
								<th class="py-2.5 px-3">Proxy Endpoint</th>
								<th class="py-2.5 px-3">Protocol</th>
								<th class="py-2.5 px-3">Credentials</th>
								<th class="py-2.5 px-3">Latency</th>
								<th class="py-2.5 px-3">Status</th>
								<th class="py-2.5 px-3 text-right">Action</th>
							</tr>
						</thead>
						<tbody class="divide-y divide-default bg-default">
							<tr
								v-for="(row, idx) in filteredRows"
								:key="row.address"
								class="hover:bg-elevated/50 transition-colors"
								:class="selectedAddresses.includes(row.address) ? 'bg-primary/10' : ''"
							>
								<td class="py-2.5 px-3">
									<UCheckbox
										:model-value="selectedAddresses.includes(row.address)"
										@update:model-value="toggleSelectRow(row.address)"
									/>
								</td>
								<td class="py-2.5 px-3 text-muted">{{ idx + 1 }}</td>
								<td class="py-2.5 px-3 text-highlighted font-semibold">{{ row.address }}</td>
								<td class="py-2.5 px-3 text-muted">
									<UBadge size="xs" variant="outline" color="neutral">
										{{ row.protocol }}
									</UBadge>
								</td>
								<td class="py-2.5 px-3 text-muted">
									<span v-if="row.credentials" class="text-warning">{{ row.credentials }}</span>
									<span v-else class="text-muted/60">None</span>
								</td>
								<td class="py-2.5 px-3">
									<span
										v-if="row.latency !== null"
										:class="row.latency < 500 ? 'text-primary font-semibold' : row.latency < 1500 ? 'text-warning' : 'text-muted'"
									>
										{{ row.latency }} ms
									</span>
									<span v-else class="text-muted/60">—</span>
								</td>
								<td class="py-2.5 px-3">
									<UBadge
										size="xs"
										:color="row.status === 'alive' ? 'success' : row.status === 'dead' ? 'error' : 'neutral'"
										variant="subtle"
									>
										{{ row.status.toUpperCase() }}
									</UBadge>
								</td>
								<td class="py-2.5 px-3 text-right">
									<div class="flex items-center justify-end gap-1">
										<UButton
											size="xs"
											color="primary"
											variant="ghost"
											icon="i-lucide-activity"
											title="Ping This Proxy"
											@click="handleTestSingle(row.address)"
										/>
										<UButton
											size="xs"
											color="error"
											variant="ghost"
											icon="i-lucide-trash"
											title="Remove Proxy"
											@click="removeSingleProxy(row.address)"
										/>
									</div>
								</td>
							</tr>
							<tr v-if="filteredRows.length === 0">
								<td colspan="8" class="py-12 text-center text-muted">
									No proxies found matching criteria. Click "Fetch & Extract" or "Browse File" above to ingest proxy nodes.
								</td>
							</tr>
						</tbody>
					</table>
				</div>
			</div>
		</UCard>
	</div>
</template>
