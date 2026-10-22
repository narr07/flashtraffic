<script setup lang="ts">
import { useTrafficState } from "~/composables/useTrafficState";

const { activeCampaign, addLog } = useTrafficState();
const toast = useToast();

const browserEngines = ["Chromium (Standard)", "Firefox (Gecko)", "WebKit (Safari Engine)"];
const selectedEngine = ref("Chromium (Standard)");

const viewportPresets = [
	{ label: "Full HD Desktop (1920 x 1080)", width: 1920, height: 1080 },
	{ label: "Laptop Standard (1366 x 768)", width: 1366, height: 768 },
	{ label: "MacBook Retina (1440 x 900)", width: 1440, height: 900 },
	{ label: "Mobile iPhone 15 Pro (393 x 852)", width: 393, height: 852 },
	{ label: "Mobile Samsung Galaxy S24 (412 x 915)", width: 412, height: 915 }
];

const selectedViewport = ref("Laptop Standard (1366 x 768)");

// Ensure defaults for Torpedo parameters
if (!activeCampaign.value.trafficMode) activeCampaign.value.trafficMode = "default";
if (!activeCampaign.value.sessionDurationType) activeCampaign.value.sessionDurationType = "variable";
if (!activeCampaign.value.sessionDurationConstant) activeCampaign.value.sessionDurationConstant = 5000;
if (!activeCampaign.value.sessionDurationMin) activeCampaign.value.sessionDurationMin = 4000;
if (!activeCampaign.value.sessionDurationMax) activeCampaign.value.sessionDurationMax = 10000;
if (!activeCampaign.value.trafficDelayType) activeCampaign.value.trafficDelayType = "variable";
if (!activeCampaign.value.trafficDelayConstant) activeCampaign.value.trafficDelayConstant = 1500;
if (!activeCampaign.value.trafficDelayMin) activeCampaign.value.trafficDelayMin = 2000;
if (!activeCampaign.value.trafficDelayMax) activeCampaign.value.trafficDelayMax = 5000;
if (!activeCampaign.value.pageOrder) activeCampaign.value.pageOrder = "random";
if (!activeCampaign.value.proxySwitching) activeCampaign.value.proxySwitching = "single_page";
if (activeCampaign.value.useRandomProxy === undefined) activeCampaign.value.useRandomProxy = true;
if (!activeCampaign.value.platformSwitching) activeCampaign.value.platformSwitching = "random_page";
if (!activeCampaign.value.specialMode) activeCampaign.value.specialMode = "surfing";
if (!activeCampaign.value.surfingCount) activeCampaign.value.surfingCount = 2;
const durationTypes = [
	{ value: "constant", label: "Constant", description: "Fixed duration" },
	{ value: "variable", label: "Variable", description: "Random jitter range" }
];
const pageOrders = [
	{ value: "in_order", label: "In Order", description: "Sequential order" },
	{ value: "random", label: "Random", description: "Randomized pick" }
];
const specialModes = [
	{ value: "repetition", label: "Repetition", description: "Repeat landing URLs" },
	{ value: "surfing", label: "Surfing", description: "Click internal links" }
];
const proxyRotations = [
	{ value: "single_page", label: "Single Page", description: "Fresh proxy each hit" },
	{ value: "cycle", label: "Once All Visited", description: "Rotate per URL cycle" }
];
const platformModes = [
	{ value: "single", label: "Single Platform", description: "Fixed OS & viewport" },
	{ value: "random_page", label: "Random Every Visit", description: "Randomize device types" }
];
const radioUi = {
	fieldset: "grid grid-cols-2 gap-2.5",
	item: "p-2.5 cursor-pointer",
	label: "text-xs font-medium",
	description: "text-[11px]"
};

const trafficModes = [
	{
		id: "default",
		label: "Default",
		desc: "Balanced organic simulation",
		badge: "4-8s • Surf: 2",
		icon: "i-lucide-activity",
		detail: "Balanced natural simulation featuring gradual scrolling, reading pauses, and realistic human mouse physics. Ideal for continuous SEO health."
	},
	{
		id: "max_sessions",
		label: "Max Sessions",
		desc: "New cookies & IP each hit",
		badge: "2.5-5s • Fresh Session",
		icon: "i-lucide-users",
		detail: "Isolates every visit with fresh session cookies and storage clearing to boost New Users metrics in Google Analytics 4."
	},
	{
		id: "max_pageviews",
		label: "Max Pageviews",
		desc: "Deep multi-page browsing",
		badge: "5-12s • Surf: 4",
		icon: "i-lucide-layers",
		detail: "Crawls 3-5 internal domain sub-pages sequentially per visitor to maximize the Pages/Session KPI."
	},
	{
		id: "min_bounce",
		label: "Min Bounce Rate",
		desc: "Forced internal surfing (GA SEO)",
		badge: "6-15s • Low Bounce",
		icon: "i-lucide-trending-down",
		detail: "Mandates clicking at least 2 internal links before ending sessions, reducing Bounce Rate on GA4 dashboards."
	},
	{
		id: "max_bounce",
		label: "Max Bounce Rate",
		desc: "1 page quick exit (load test)",
		badge: "500ms • Instant New Session",
		icon: "i-lucide-log-out",
		detail: "Loads the target landing page and exits immediately without internal navigation. Useful for server load and bounce stress testing."
	}
];

const selectTrafficMode = (modeId: "default" | "max_sessions" | "max_pageviews" | "min_bounce" | "max_bounce") => {
	activeCampaign.value.trafficMode = modeId;

	switch (modeId) {
		case "default":
			activeCampaign.value.sessionDurationType = "variable";
			activeCampaign.value.sessionDurationMin = 4000;
			activeCampaign.value.sessionDurationMax = 8000;
			activeCampaign.value.trafficDelayType = "variable";
			activeCampaign.value.trafficDelayMin = 2000;
			activeCampaign.value.trafficDelayMax = 5000;
			activeCampaign.value.specialMode = "surfing";
			activeCampaign.value.surfingCount = 2;
			activeCampaign.value.allowSurfing = true;
			activeCampaign.value.proxySwitching = "single_page";
			activeCampaign.value.platformSwitching = "random_page";
			toast.add({
				title: "Default Strategy Selected",
				description: "Balanced 4-8s dwell time, human scrolling & 2 internal link visits applied.",
				color: "primary",
				icon: "i-lucide-activity"
			});
			break;

		case "max_sessions":
			activeCampaign.value.sessionDurationType = "variable";
			activeCampaign.value.sessionDurationMin = 2500;
			activeCampaign.value.sessionDurationMax = 5000;
			activeCampaign.value.trafficDelayType = "variable";
			activeCampaign.value.trafficDelayMin = 1500;
			activeCampaign.value.trafficDelayMax = 3500;
			activeCampaign.value.specialMode = "repetition";
			activeCampaign.value.surfingCount = 1;
			activeCampaign.value.allowSurfing = false;
			activeCampaign.value.proxySwitching = "single_page";
			activeCampaign.value.platformSwitching = "random_page";
			toast.add({
				title: "Max Sessions Strategy Selected",
				description: "100% fresh New Sessions per hit with isolated IP & browser context applied.",
				color: "primary",
				icon: "i-lucide-users"
			});
			break;

		case "max_pageviews":
			activeCampaign.value.sessionDurationType = "variable";
			activeCampaign.value.sessionDurationMin = 5000;
			activeCampaign.value.sessionDurationMax = 12000;
			activeCampaign.value.trafficDelayType = "variable";
			activeCampaign.value.trafficDelayMin = 2000;
			activeCampaign.value.trafficDelayMax = 4000;
			activeCampaign.value.specialMode = "surfing";
			activeCampaign.value.surfingCount = 4;
			activeCampaign.value.allowSurfing = true;
			toast.add({
				title: "Max Pageviews Strategy Selected",
				description: "Deep browsing configured: 4 internal sub-pages per session to maximize Pages/Session.",
				color: "primary",
				icon: "i-lucide-layers"
			});
			break;

		case "min_bounce":
			activeCampaign.value.sessionDurationType = "variable";
			activeCampaign.value.sessionDurationMin = 6000;
			activeCampaign.value.sessionDurationMax = 15000;
			activeCampaign.value.trafficDelayType = "variable";
			activeCampaign.value.trafficDelayMin = 3000;
			activeCampaign.value.trafficDelayMax = 6000;
			activeCampaign.value.specialMode = "surfing";
			activeCampaign.value.surfingCount = 2;
			activeCampaign.value.allowSurfing = true;
			toast.add({
				title: "Min Bounce Rate Strategy Selected",
				description: "Low bounce configuration: 6-15s dwell time with mandatory internal page navigation applied.",
				color: "primary",
				icon: "i-lucide-trending-down"
			});
			break;

		case "max_bounce":
			activeCampaign.value.sessionDurationType = "constant";
			activeCampaign.value.sessionDurationConstant = 500;
			activeCampaign.value.trafficDelayType = "constant";
			activeCampaign.value.trafficDelayConstant = 0;
			activeCampaign.value.specialMode = "repetition";
			activeCampaign.value.surfingCount = 1;
			activeCampaign.value.allowSurfing = false;
			toast.add({
				title: "Max Bounce Rate Strategy Selected",
				description: "Fast 500ms single-page exit with 0ms delay and 100% New Session per page applied.",
				color: "warning",
				icon: "i-lucide-log-out"
			});
			break;
	}
};

const updateSettings = () => {
	addLog("SUCCESS", "Traffic simulation preferences & FlashTraffic settings saved.");
	toast.add({
		title: "Simulation Settings Saved",
		description: `Active Mode: ${activeCampaign.value.trafficMode.toUpperCase()} | Special Mode: ${activeCampaign.value.specialMode}`,
		color: "success",
		icon: "i-lucide-check-circle"
	});
};

const applyFastModePreset = () => {
	activeCampaign.value.trafficMode = "max_bounce";
	activeCampaign.value.sessionDurationType = "constant";
	activeCampaign.value.sessionDurationConstant = 500;
	activeCampaign.value.trafficDelayType = "constant";
	activeCampaign.value.trafficDelayConstant = 0;
	activeCampaign.value.specialMode = "repetition";
	activeCampaign.value.surfingCount = 1;
	activeCampaign.value.allowSurfing = false;
	toast.add({
		title: "⚡ Ultra-Fast Pageviews Preset Applied",
		description: "Max Bounce Rate, 500ms dwell, 0ms delay, and 100% New Session per page (No scroll).",
		color: "success",
		icon: "i-lucide-zap"
	});
};

const applyHumanRealisticPreset = () => {
	activeCampaign.value.trafficMode = "default";
	activeCampaign.value.sessionDurationType = "variable";
	activeCampaign.value.sessionDurationMin = 4000;
	activeCampaign.value.sessionDurationMax = 8000;
	activeCampaign.value.trafficDelayType = "variable";
	activeCampaign.value.trafficDelayMin = 2000;
	activeCampaign.value.trafficDelayMax = 5000;
	activeCampaign.value.specialMode = "surfing";
	activeCampaign.value.surfingCount = 2;
	activeCampaign.value.allowSurfing = true;
	toast.add({
		title: "🛡️ Realistic Human Simulation Applied",
		description: "4-8s randomized dwell, natural scroll pauses, and internal link surfing.",
		color: "primary",
		icon: "i-lucide-shield-check"
	});
};
</script>

<template>
	<div class="max-w-5xl mx-auto space-y-6">
		<!-- Teleport Title to Row 1 Navbar -->
		<AppHeaderTitle>
			<div class="flex items-center gap-2">
				<span class="text-muted">Engine</span>
				<span class="text-muted/60">/</span>
				<span class="flex items-center gap-1.5 font-bold text-highlighted">
					<UIcon name="i-lucide-cpu" class="size-4 text-primary" />
					Traffic Engine & Behavior
				</span>
			</div>
		</AppHeaderTitle>

		<!-- Teleport Save Button to Row 1 Navbar Actions -->
		<AppHeaderActions>
			<UButton icon="i-lucide-save" size="xs" color="primary" @click="updateSettings">
				Save Settings
			</UButton>
		</AppHeaderActions>

		<!-- Teleport Toolbar Controls to Row 2 Sub-Navbar -->
		<AppToolbar>
			<div class="flex items-center gap-2">
				<span class="text-muted">Presets:</span>
				<UButton color="warning" variant="subtle" icon="i-lucide-zap" size="xs" @click="applyFastModePreset">
					Ultra-Fast
				</UButton>
				<UButton color="neutral" variant="subtle" icon="i-lucide-shield-check" size="xs" @click="applyHumanRealisticPreset">
					Realistic Human
				</UButton>
			</div>

			<div class="flex items-center gap-2 font-mono text-[11px] text-muted">
				<span>Simulation Mode:</span>
				<UBadge size="xs" variant="subtle" color="primary">
					{{ activeCampaign.trafficMode.toUpperCase() }}
				</UBadge>
			</div>
		</AppToolbar>

		<!-- Traffic modes -->
		<UCard variant="subtle">
			<template #header>
				<div class="flex items-center justify-between">
					<div class="flex items-center gap-2">
						<UIcon name="i-lucide-sliders-horizontal" class="size-4 text-primary" />
						<h3 class="text-sm font-semibold text-highlighted">Traffic Modes</h3>
					</div>
					<UBadge size="xs" variant="subtle" class="font-mono">
						{{ activeCampaign.trafficMode.toUpperCase() }}
					</UBadge>
				</div>
			</template>

			<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-5 gap-3">
				<UTooltip v-for="mode in trafficModes" :key="mode.id" :text="mode.detail" :content="{ side: 'top' }">
					<button
						type="button"
						class="w-full text-left p-3 rounded-lg ring ring-inset transition flex flex-col gap-1.5 cursor-pointer"
						:class="activeCampaign.trafficMode === mode.id
							? 'bg-primary/10 ring-primary'
							: 'bg-default ring-default hover:bg-elevated'"
						@click="selectTrafficMode(mode.id as any)"
					>
						<div class="flex items-center justify-between">
							<UIcon
								:name="mode.icon"
								class="size-4"
								:class="activeCampaign.trafficMode === mode.id ? 'text-primary' : 'text-muted'"
							/>
							<UIcon v-if="activeCampaign.trafficMode === mode.id" name="i-lucide-check-circle-2" class="size-4 text-primary" />
						</div>
						<div class="text-xs font-semibold text-highlighted">{{ mode.label }}</div>
						<div class="text-[11px] text-muted leading-tight">{{ mode.desc }}</div>
						<UBadge size="xs" variant="subtle" :color="activeCampaign.trafficMode === mode.id ? 'primary' : 'neutral'" class="font-mono w-fit">
							{{ mode.badge }}
						</UBadge>
					</button>
				</UTooltip>
			</div>
		</UCard>

		<!-- Duration & Delay -->
		<div class="grid grid-cols-1 md:grid-cols-2 gap-5">
			<UCard variant="subtle">
				<template #header>
					<div class="flex items-center gap-2">
						<UIcon name="i-lucide-clock" class="size-4 text-primary" />
						<h3 class="text-sm font-semibold text-highlighted">Session Duration</h3>
					</div>
				</template>
				<div class="space-y-4">
					<URadioGroup
						v-model="activeCampaign.sessionDurationType"
						:items="durationTypes"
						variant="card"
						orientation="horizontal"
						:ui="radioUi"
					/>
					<UFormField v-if="activeCampaign.sessionDurationType === 'constant'" label="Fixed Duration" help="In milliseconds">
						<UInput v-model.number="activeCampaign.sessionDurationConstant" type="number" :min="100" :step="100" class="w-full">
							<template #trailing><span class="text-xs text-muted">ms</span></template>
						</UInput>
					</UFormField>
					<div v-else class="grid grid-cols-2 gap-3">
						<UFormField label="Min">
							<UInput v-model.number="activeCampaign.sessionDurationMin" type="number" :min="100" :step="100" class="w-full">
								<template #trailing><span class="text-xs text-muted">ms</span></template>
							</UInput>
						</UFormField>
						<UFormField label="Max">
							<UInput v-model.number="activeCampaign.sessionDurationMax" type="number" :min="200" :step="100" class="w-full">
								<template #trailing><span class="text-xs text-muted">ms</span></template>
							</UInput>
						</UFormField>
					</div>
					<UAlert
						v-if="activeCampaign.sessionDurationType === 'constant' && activeCampaign.sessionDurationConstant <= 1500"
						color="success"
						variant="subtle"
						icon="i-lucide-zap"
						title="Fast Mode Active"
						description="Closes target pages immediately upon DOMContentLoaded without scroll pauses."
					/>
				</div>
			</UCard>

			<UCard variant="subtle">
				<template #header>
					<div class="flex items-center gap-2">
						<UIcon name="i-lucide-hourglass" class="size-4 text-primary" />
						<h3 class="text-sm font-semibold text-highlighted">Traffic Delay</h3>
					</div>
				</template>
				<div class="space-y-4">
					<URadioGroup
						v-model="activeCampaign.trafficDelayType"
						:items="durationTypes"
						variant="card"
						orientation="horizontal"
						:ui="radioUi"
					/>
					<UFormField v-if="activeCampaign.trafficDelayType === 'constant'" label="Fixed Delay" help="Cooldown between hits">
						<UInput v-model.number="activeCampaign.trafficDelayConstant" type="number" :min="0" :step="500" class="w-full">
							<template #trailing><span class="text-xs text-muted">ms</span></template>
						</UInput>
					</UFormField>
					<div v-else class="grid grid-cols-2 gap-3">
						<UFormField label="Min">
							<UInput v-model.number="activeCampaign.trafficDelayMin" type="number" :min="0" :step="500" class="w-full">
								<template #trailing><span class="text-xs text-muted">ms</span></template>
							</UInput>
						</UFormField>
						<UFormField label="Max">
							<UInput v-model.number="activeCampaign.trafficDelayMax" type="number" :min="1000" :step="500" class="w-full">
								<template #trailing><span class="text-xs text-muted">ms</span></template>
							</UInput>
						</UFormField>
					</div>
				</div>
			</UCard>
		</div>

		<!-- Navigation & Rotation -->
		<div class="grid grid-cols-1 md:grid-cols-2 gap-5">
			<UCard variant="subtle">
				<template #header>
					<div class="flex items-center gap-2">
						<UIcon name="i-lucide-shuffle" class="size-4 text-primary" />
						<h3 class="text-sm font-semibold text-highlighted">Page Order & Navigation</h3>
					</div>
				</template>
				<div class="space-y-4">
					<UFormField label="Target URL Order">
						<URadioGroup v-model="activeCampaign.pageOrder" :items="pageOrders" variant="card" orientation="horizontal" :ui="radioUi" />
					</UFormField>
					<USeparator />
					<UFormField label="Special Mode">
						<URadioGroup v-model="activeCampaign.specialMode" :items="specialModes" variant="card" orientation="horizontal" :ui="radioUi" />
					</UFormField>
					<div class="p-3 rounded-lg bg-elevated/50 space-y-2">
						<div class="flex items-center justify-between text-xs">
							<span class="text-default">
								{{ activeCampaign.specialMode === 'surfing' ? 'Surfing depth' : 'Repetition' }}
							</span>
							<UBadge variant="subtle" class="font-mono">{{ activeCampaign.surfingCount }}x</UBadge>
						</div>
						<USlider v-model="activeCampaign.surfingCount" :min="1" :max="5" />
					</div>
				</div>
			</UCard>

			<UCard variant="subtle">
				<template #header>
					<div class="flex items-center gap-2">
						<UIcon name="i-lucide-network" class="size-4 text-primary" />
						<h3 class="text-sm font-semibold text-highlighted">Proxy & Platform</h3>
					</div>
				</template>
				<div class="space-y-4">
					<UFormField label="Proxy Rotation Trigger">
						<URadioGroup v-model="activeCampaign.proxySwitching" :items="proxyRotations" variant="card" orientation="horizontal" :ui="radioUi" />
					</UFormField>
					<UCheckbox v-model="activeCampaign.useRandomProxy" label="Randomize proxy pick from active proxy list" />
					<USeparator />
					<UFormField label="Platform / User-Agent">
						<URadioGroup v-model="activeCampaign.platformSwitching" :items="platformModes" variant="card" orientation="horizontal" :ui="radioUi" />
					</UFormField>
				</div>
			</UCard>
		</div>

		<!-- Browser core -->
		<UCard variant="subtle">
			<template #header>
				<div class="flex items-center gap-2">
					<UIcon name="i-lucide-globe" class="size-4 text-primary" />
					<h3 class="text-sm font-semibold text-highlighted">Browser Core</h3>
				</div>
			</template>
			<div class="space-y-4">
				<div class="flex items-center justify-between gap-4 p-3 rounded-lg bg-elevated/50">
					<div>
						<div class="text-sm font-medium text-highlighted">Headless Mode</div>
						<p class="text-xs text-muted">Runs silently in background without UI windows (fast and lightweight).</p>
					</div>
					<USwitch v-model="activeCampaign.headless" />
				</div>
				<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
					<UFormField label="Browser Core">
						<USelect v-model="selectedEngine" :items="browserEngines" class="w-full" />
					</UFormField>
					<UFormField label="Viewport">
						<USelect v-model="selectedViewport" :items="viewportPresets.map(v => v.label)" class="w-full" />
					</UFormField>
				</div>
			</div>
		</UCard>
	</div>
</template>
