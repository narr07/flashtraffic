<script setup lang="ts">
import { useTrafficBackend } from "~/composables/useTrafficBackend";
import { useTrafficState } from "~/composables/useTrafficState";

const { browserStackConfig, addLog, saveBrowserStackConfig, clearBrowserStackConfig } = useTrafficState();
const backend = useTrafficBackend();
const toast = useToast();

const isTesting = ref(false);
const testResult = ref<string | null>(null);

const handleSaveCredentials = () => {
	if (!browserStackConfig.value.username.trim() || !browserStackConfig.value.accessKey.trim()) {
		toast.add({
			title: "Incomplete Credentials",
			description: "Please enter both Username and Access Key before saving.",
			color: "warning"
		});
		return;
	}
	saveBrowserStackConfig();
	addLog("SUCCESS", "BrowserStack credentials securely saved to local storage.");
	toast.add({
		title: "Settings & Credentials Saved",
		description: "Your BrowserStack credentials will remain saved across application restarts until deleted.",
		color: "success",
		icon: "i-lucide-shield-check"
	});
};

const handleClearCredentials = () => {
	clearBrowserStackConfig();
	testResult.value = null;
	addLog("INFO", "BrowserStack credentials removed from local storage.");
	toast.add({
		title: "Credentials Removed",
		description: "BrowserStack credentials have been deleted.",
		color: "neutral",
		icon: "i-lucide-trash-2"
	});
};

const openDashboard = async () => {
	try {
		const { openUrl } = await import("@tauri-apps/plugin-opener");
		await openUrl("https://live.browserstack.com/dashboard");
	} catch (e) {
		console.warn("Opener fallback:", e);
		window.open("https://live.browserstack.com/dashboard", "_blank");
	}
};

// Ensure defaults
if (!browserStackConfig.value.selectedDevices || browserStackConfig.value.selectedDevices.length === 0) {
	browserStackConfig.value.selectedDevices = [
		"iPhone 15 Pro (iOS 17)",
		"Samsung Galaxy S24 Ultra",
		"Windows 11 (Chrome 128)"
	];
}
if (browserStackConfig.value.enableCloud === undefined) {
	browserStackConfig.value.enableCloud = false;
}
if (!browserStackConfig.value.maxParallel) {
	browserStackConfig.value.maxParallel = 5;
}

interface DeviceItem {
	label: string
	category: "iOS" | "Android" | "Desktop"
	os: string
	device: string
	browser: string
	icon: string
	resolution: string
}

const deviceList: DeviceItem[] = [
	// iOS Real Devices
	{ label: "iPhone 15 Pro (iOS 17)", category: "iOS", os: "iOS", device: "iPhone 15 Pro", browser: "Safari", icon: "i-lucide-smartphone", resolution: "1179 x 2556" },
	{ label: "iPhone 14 Pro Max (iOS 16)", category: "iOS", os: "iOS", device: "iPhone 14 Pro Max", browser: "Safari", icon: "i-lucide-smartphone", resolution: "1290 x 2796" },
	{ label: "iPhone 13 (iOS 15)", category: "iOS", os: "iOS", device: "iPhone 13", browser: "Safari", icon: "i-lucide-smartphone", resolution: "1170 x 2532" },
	{ label: "iPad Pro 12.9 (iOS 17)", category: "iOS", os: "iOS", device: "iPad Pro 12.9", browser: "Safari", icon: "i-lucide-tablet", resolution: "2048 x 2732" },

	// Android Real Devices
	{ label: "Samsung Galaxy S24 Ultra", category: "Android", os: "Android", device: "Samsung Galaxy S24 Ultra", browser: "Chrome", icon: "i-lucide-smartphone", resolution: "1440 x 3120" },
	{ label: "Samsung Galaxy S23 (Android 13)", category: "Android", os: "Android", device: "Samsung Galaxy S23", browser: "Chrome", icon: "i-lucide-smartphone", resolution: "1080 x 2340" },
	{ label: "Google Pixel 8 Pro (Android 14)", category: "Android", os: "Android", device: "Google Pixel 8 Pro", browser: "Chrome", icon: "i-lucide-smartphone", resolution: "1344 x 2992" },
	{ label: "OnePlus 11 (Android 13)", category: "Android", os: "Android", device: "OnePlus 11", browser: "Chrome", icon: "i-lucide-smartphone", resolution: "1440 x 3216" },
	{ label: "Xiaomi 13 (Android 13)", category: "Android", os: "Android", device: "Xiaomi 13", browser: "Chrome", icon: "i-lucide-smartphone", resolution: "1080 x 2400" },

	// Desktop OS & Browsers
	{ label: "Windows 11 (Chrome 128)", category: "Desktop", os: "Windows 11", device: "Desktop", browser: "Chrome", icon: "i-lucide-monitor", resolution: "1920 x 1080" },
	{ label: "Windows 11 (Firefox 130)", category: "Desktop", os: "Windows 11", device: "Desktop", browser: "Firefox", icon: "i-lucide-monitor", resolution: "1920 x 1080" },
	{ label: "Windows 10 (Edge 128)", category: "Desktop", os: "Windows 10", device: "Desktop", browser: "Edge", icon: "i-lucide-monitor", resolution: "1920 x 1080" },
	{ label: "macOS Sonoma (Safari 17)", category: "Desktop", os: "macOS", device: "Mac Desktop", browser: "Safari", icon: "i-lucide-laptop", resolution: "2560 x 1440" },
	{ label: "macOS Ventura (Chrome 128)", category: "Desktop", os: "macOS", device: "Mac Desktop", browser: "Chrome", icon: "i-lucide-laptop", resolution: "1920 x 1080" }
];

const categoryFilter = ref<"all" | "iOS" | "Android" | "Desktop">("all");
const deviceSearch = ref("");

const filteredDevices = computed(() => {
	let list = deviceList;
	if (categoryFilter.value !== "all") {
		list = list.filter(d => d.category === categoryFilter.value);
	}
	if (deviceSearch.value.trim()) {
		const q = deviceSearch.value.toLowerCase();
		list = list.filter(d => d.label.toLowerCase().includes(q) || d.browser.toLowerCase().includes(q));
	}
	return list;
});

// Selection actions
const isDeviceSelected = (label: string) => {
	return (browserStackConfig.value.selectedDevices || []).includes(label);
};

const toggleDevice = (label: string) => {
	const current = browserStackConfig.value.selectedDevices || [];
	if (current.includes(label)) {
		if (current.length > 1) {
			browserStackConfig.value.selectedDevices = current.filter((l: string) => l !== label);
		} else {
			toast.add({
				title: "At Least One Device Required",
				description: "You must keep at least 1 target cloud device selected.",
				color: "warning"
			});
		}
	} else {
		browserStackConfig.value.selectedDevices = [...current, label];
	}
};

const selectAllDevices = () => {
	browserStackConfig.value.selectedDevices = deviceList.map(d => d.label);
	toast.add({
		title: "All Devices Selected",
		description: `Selected all ${deviceList.length} mobile and desktop cloud targets.`,
		color: "success"
	});
};

const selectMobileOnly = () => {
	browserStackConfig.value.selectedDevices = deviceList
		.filter(d => d.category === "iOS" || d.category === "Android")
		.map(d => d.label);
	toast.add({
		title: "Mobile Real Devices Selected",
		description: `Selected ${browserStackConfig.value.selectedDevices.length} real iOS & Android targets.`,
		color: "success"
	});
};

const selectDesktopOnly = () => {
	browserStackConfig.value.selectedDevices = deviceList
		.filter(d => d.category === "Desktop")
		.map(d => d.label);
	toast.add({
		title: "Desktop Cloud Browsers Selected",
		description: `Selected ${browserStackConfig.value.selectedDevices.length} Windows & Mac targets.`,
		color: "success"
	});
};

const handleTestAuth = async () => {
	if (!browserStackConfig.value.username || !browserStackConfig.value.accessKey) {
		toast.add({
			title: "Credentials Missing",
			description: "Please enter your BrowserStack Username and Access Key.",
			color: "warning"
		});
		return;
	}

	isTesting.value = true;
	testResult.value = null;
	addLog("INFO", "Verifying credentials with BrowserStack Automate API...");

	try {
		const plan = await backend.testBrowserStackCredentials(
			browserStackConfig.value.username,
			browserStackConfig.value.accessKey
		);
		browserStackConfig.value.isConnected = true;
		browserStackConfig.value.planName = plan.automate_plan || "Active Automate Plan";
		browserStackConfig.value.maxParallel = plan.team_parallel_sessions_max_allowed || 5;
		browserStackConfig.value.runningSessions = plan.parallel_sessions_running || 0;

		testResult.value = `Connected successfully! Plan: ${plan.automate_plan || "Active"} (Parallel Limit: ${plan.team_parallel_sessions_max_allowed || 5})`;
		saveBrowserStackConfig();
		addLog("SUCCESS", `BrowserStack authenticated & saved: ${browserStackConfig.value.planName} (Max Parallel: ${browserStackConfig.value.maxParallel})`);
		toast.add({
			title: "BrowserStack Authenticated & Saved",
			description: `Plan: ${browserStackConfig.value.planName} | Max Parallel: ${browserStackConfig.value.maxParallel} Sessions (Stored locally)`,
			color: "success",
			icon: "i-lucide-check-circle"
		});
	} catch (err: any) {
		browserStackConfig.value.isConnected = false;
		testResult.value = `Connection error: ${err}`;
		addLog("ERROR", `BrowserStack auth failed: ${err}`);
		toast.add({
			title: "Authentication Failed",
			description: String(err),
			color: "error",
			icon: "i-lucide-alert-triangle"
		});
	} finally {
		isTesting.value = false;
	}
};
</script>

<template>
	<div class="max-w-5xl mx-auto space-y-6">
		<!-- Teleport Title to Row 1 Navbar -->
		<AppHeaderTitle>
			<div class="flex items-center gap-2">
				<span class="text-muted">Cloud</span>
				<span class="text-muted/60">/</span>
				<span class="flex items-center gap-1.5 font-bold text-highlighted">
					<UIcon name="i-lucide-cloud" class="size-4 text-primary" />
					BrowserStack Cloud Matrix
				</span>
			</div>
		</AppHeaderTitle>

		<!-- Teleport Actions to Row 1 Navbar -->
		<AppHeaderActions>
			<UButton
				color="primary"
				icon="i-lucide-external-link"
				size="xs"
				@click="openDashboard"
			>
				Live Dashboard
			</UButton>
		</AppHeaderActions>

		<!-- Teleport Toolbar to Row 2 Sub-Navbar -->
		<AppToolbar>
			<div class="flex items-center gap-3">
				<div class="flex items-center gap-2">
					<span class="text-xs font-medium text-highlighted">Route via Cloud:</span>
					<USwitch v-model="browserStackConfig.enableCloud" size="sm" />
				</div>
				<UBadge
					size="xs"
					:color="browserStackConfig.enableCloud ? 'success' : 'neutral'"
					variant="subtle"
				>
					{{ browserStackConfig.enableCloud ? 'Cloud Execution Active' : 'Local Playwright Mode' }}
				</UBadge>
			</div>

			<div class="flex items-center gap-2 font-mono text-[11px] text-muted">
				<span>Selected: <strong class="text-primary">{{ (browserStackConfig.selectedDevices || []).length }}</strong> Devices</span>
				<span>•</span>
				<span>Max Parallel: <strong class="text-highlighted">{{ browserStackConfig.maxParallel }}</strong> Sessions</span>
			</div>
		</AppToolbar>

		<!-- API Credentials & Parallel Limit Status -->
		<div class="grid grid-cols-1 lg:grid-cols-3 gap-4">
			<!-- Credentials Form (2 cols) -->
			<UCard variant="subtle" class="lg:col-span-2 space-y-4">
				<template #header>
					<div class="flex items-center justify-between">
						<div class="flex items-center gap-2">
							<UIcon name="i-lucide-shield-check" class="size-4 text-primary" />
							<h3 class="text-sm font-semibold text-highlighted">BrowserStack API Credentials</h3>
						</div>
						<div class="flex items-center gap-2">
							<UBadge
								v-if="browserStackConfig.username"
								size="xs"
								color="info"
								variant="subtle"
								icon="i-lucide-save"
							>
								Saved Locally
							</UBadge>
							<UBadge
								size="xs"
								:color="browserStackConfig.isConnected ? 'success' : 'neutral'"
								variant="subtle"
							>
								{{ browserStackConfig.isConnected ? 'Authenticated' : 'Not Connected' }}
							</UBadge>
						</div>
					</div>
				</template>

				<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
					<UFormField label="BrowserStack Username" required>
						<UInput v-model="browserStackConfig.username" placeholder="e.g. johndoe_abc123" class="w-full" />
					</UFormField>

					<UFormField label="Access Key" required>
						<UInput
							v-model="browserStackConfig.accessKey"
							type="password"
							placeholder="••••••••••••••••••••"
							class="w-full"
						/>
					</UFormField>
				</div>

				<p class="text-[11px] text-muted">
					Credentials are permanently remembered in your local app storage across reboots until you explicitly clear or update them.
				</p>

				<div class="space-y-3 pt-2 border-t border-default">
					<div class="grid grid-cols-1 sm:grid-cols-3 gap-2">
						<UButton
							color="primary"
							icon="i-lucide-key-round"
							size="xs"
							block
							:loading="isTesting"
							label="Test Connection"
							@click="handleTestAuth"
						/>

						<UButton
							color="primary"
							variant="outline"
							icon="i-lucide-save"
							size="xs"
							block
							label="Save Settings"
							@click="handleSaveCredentials"
						/>

						<UButton
							color="error"
							variant="ghost"
							icon="i-lucide-trash-2"
							size="xs"
							block
							label="Clear Credentials"
							:disabled="!browserStackConfig.username && !browserStackConfig.accessKey"
							@click="handleClearCredentials"
						/>
					</div>

					<div v-if="testResult" class="text-xs font-mono pt-1">
						<span :class="browserStackConfig.isConnected ? 'text-primary' : 'text-error'">
							{{ testResult }}
						</span>
					</div>
				</div>
			</UCard>

			<!-- Parallel Quota & Plan Limit Card (1 col) -->
			<UCard variant="subtle">
				<template #header>
					<div class="flex items-center gap-2">
						<UIcon name="i-lucide-cpu" class="size-4 text-primary" />
						<h3 class="text-sm font-semibold text-highlighted">Concurrency & Limits</h3>
					</div>
				</template>

				<div class="space-y-3 text-xs">
					<div class="p-3 rounded-lg bg-elevated/60 border border-default flex justify-between items-center">
						<span class="text-muted">Automate Plan</span>
						<span class="text-highlighted font-semibold">{{ browserStackConfig.planName || 'Unverified' }}</span>
					</div>

					<div class="p-3 rounded-lg bg-elevated/60 border border-default flex justify-between items-center">
						<span class="text-muted">Max Parallel Concurrency</span>
						<span class="text-primary font-bold text-sm">
							{{ browserStackConfig.maxParallel || 5 }} Sessions
						</span>
					</div>

					<div class="p-3 rounded-lg bg-elevated/60 border border-default flex justify-between items-center">
						<span class="text-muted">Selected Target Devices</span>
						<span class="text-warning font-bold">
							{{ (browserStackConfig.selectedDevices || []).length }} Devices Active
						</span>
					</div>

					<p class="text-[11px] text-muted leading-relaxed pt-1">
						FlashTraffic queue orchestrator partitions incoming visits across your selected devices while respecting your parallel session ceiling.
					</p>
				</div>
			</UCard>
		</div>

		<!-- Target Cloud Device Matrix (Multi-Select) -->
		<UCard variant="subtle" class="space-y-4">
			<template #header>
				<div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
					<div class="flex items-center gap-2">
						<UIcon name="i-lucide-grid" class="text-primary size-4" />
						<h3 class="text-sm font-semibold text-highlighted">Target Cloud Device Matrix (Multi-Select)</h3>
						<UBadge size="xs" color="primary" variant="subtle">
							{{ (browserStackConfig.selectedDevices || []).length }} / {{ deviceList.length }} Selected
						</UBadge>
					</div>

					<!-- Quick Filter Pills -->
					<div class="flex items-center gap-1.5">
						<UButton size="xs" variant="link" color="neutral" @click="selectAllDevices">Select All</UButton>
						<span class="text-muted">•</span>
						<UButton size="xs" variant="link" color="neutral" @click="selectMobileOnly">Mobile Only (9)</UButton>
						<span class="text-muted">•</span>
						<UButton size="xs" variant="link" color="neutral" @click="selectDesktopOnly">Desktop Only (5)</UButton>
					</div>
				</div>
			</template>

			<div class="space-y-4">
				<!-- Search & Category Filter Bar -->
				<div class="flex flex-col sm:flex-row items-stretch sm:items-center justify-between gap-3 pb-3 border-b border-default">
					<UInput
						v-model="deviceSearch"
						placeholder="Search device (e.g. iPhone, Pixel, Mac)..."
						icon="i-lucide-search"
						size="xs"
						class="max-w-xs"
					/>

					<div class="flex items-center gap-1">
						<UButton
							size="xs"
							:variant="categoryFilter === 'all' ? 'solid' : 'ghost'"
							:color="categoryFilter === 'all' ? 'primary' : 'neutral'"
							@click="categoryFilter = 'all'"
						>
							All ({{ deviceList.length }})
						</UButton>
						<UButton
							size="xs"
							:variant="categoryFilter === 'iOS' ? 'solid' : 'ghost'"
							:color="categoryFilter === 'iOS' ? 'primary' : 'neutral'"
							@click="categoryFilter = 'iOS'"
						>
							iOS (4)
						</UButton>
						<UButton
							size="xs"
							:variant="categoryFilter === 'Android' ? 'solid' : 'ghost'"
							:color="categoryFilter === 'Android' ? 'primary' : 'neutral'"
							@click="categoryFilter = 'Android'"
						>
							Android (5)
						</UButton>
						<UButton
							size="xs"
							:variant="categoryFilter === 'Desktop' ? 'solid' : 'ghost'"
							:color="categoryFilter === 'Desktop' ? 'primary' : 'neutral'"
							@click="categoryFilter = 'Desktop'"
						>
							Desktop (5)
						</UButton>
					</div>
				</div>

				<!-- Device Cards Grid -->
				<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
					<div
						v-for="d in filteredDevices"
						:key="d.label"
						class="p-3 rounded-xl border transition-all cursor-pointer flex items-start gap-3 select-none"
						:class="isDeviceSelected(d.label)
							? 'border-primary/50 bg-primary/10 shadow-sm'
							: 'border-default bg-elevated/40 hover:bg-elevated'"
						@click="toggleDevice(d.label)"
					>
						<UCheckbox
							:model-value="isDeviceSelected(d.label)"
							class="mt-1"
							@update:model-value="toggleDevice(d.label)"
						/>

						<div class="flex-1 min-w-0">
							<div class="flex items-center gap-1.5">
								<UIcon :name="d.icon" class="size-3.5 text-muted shrink-0" />
								<span class="text-xs font-semibold text-highlighted truncate">{{ d.label }}</span>
							</div>

							<div class="flex items-center gap-2 mt-1 text-[11px] text-muted">
								<span>{{ d.browser }}</span>
								<span class="text-muted">•</span>
								<span>{{ d.resolution }}</span>
							</div>

							<div class="mt-2 flex items-center gap-1.5">
								<UBadge
									size="xs"
									:color="d.category === 'iOS' ? 'primary' : d.category === 'Android' ? 'success' : 'neutral'"
									variant="subtle"
								>
									{{ d.category }}
								</UBadge>
								<span class="text-[10px] text-muted font-mono">Real Device</span>
							</div>
						</div>
					</div>
				</div>
			</div>
		</UCard>
	</div>
</template>
