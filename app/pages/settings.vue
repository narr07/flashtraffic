<script setup lang="ts">
import { useTrafficState } from "~/composables/useTrafficState";

const {
	addLog,
	activeCampaign,
	proxies,
	browserStackConfig,
	downloadSettingsFile,
	importSettingsJson
} = useTrafficState();

const toast = useToast();

const minimizeToTray = ref(true);
const clearLogHourly = ref(true);

const scheduleEnabled = ref(false);
const scheduleStartTime = ref("08:00");

const fileInputRef = ref<HTMLInputElement | null>(null);

const triggerFileInput = () => {
	if (fileInputRef.value) {
		fileInputRef.value.click();
	}
};

const handleFileUpload = (event: Event) => {
	const target = event.target as HTMLInputElement;
	const file = target.files?.[0];
	if (!file) return;

	const reader = new FileReader();
	reader.onload = (e) => {
		const content = e.target?.result as string;
		if (content) {
			const success = importSettingsJson(content);
			if (success) {
				toast.add({
					title: "Settings Restored Successfully",
					description: `Loaded BrowserStack credentials, ${activeCampaign.value.targetUrls.length} URLs, and ${proxies.value.length} proxies.`,
					color: "success",
					icon: "i-lucide-check-circle"
				});
			} else {
				toast.add({
					title: "Import Failed",
					description: "The selected file is not a valid FlashTraffic configuration JSON.",
					color: "error",
					icon: "i-lucide-alert-triangle"
				});
			}
		}
		// Reset input
		target.value = "";
	};
	reader.readAsText(file);
};

const handleExport = () => {
	downloadSettingsFile();
	toast.add({
		title: "Configuration Exported",
		description: "Settings file downloaded. You can transfer this file to any other laptop.",
		color: "success",
		icon: "i-lucide-download"
	});
};

const saveSettings = () => {
	addLog("SUCCESS", "System preferences saved.");
	toast.add({
		title: "Preferences Saved",
		description: "System and background scheduler preferences updated.",
		color: "success"
	});
};
</script>

<template>
	<div class="max-w-5xl mx-auto space-y-6">
		<!-- Hidden file input for importing config -->
		<input
			ref="fileInputRef"
			type="file"
			accept=".json"
			class="hidden"
			@change="handleFileUpload"
		>

		<!-- Teleport Title to Row 1 Navbar -->
		<AppHeaderTitle>
			<div class="flex items-center gap-2">
				<span class="text-muted">Settings</span>
				<span class="text-muted/60">/</span>
				<span class="flex items-center gap-1.5 font-bold text-highlighted">
					<UIcon name="i-lucide-settings-2" class="size-4 text-primary" />
					System Preferences & Portability
				</span>
			</div>
		</AppHeaderTitle>

		<!-- Teleport Actions to Row 1 Navbar -->
		<AppHeaderActions>
			<UButton color="primary" icon="i-lucide-save" size="xs" @click="saveSettings">
				Save Preferences
			</UButton>
		</AppHeaderActions>

		<!-- Teleport Toolbar to Row 2 Sub-Navbar -->
		<AppToolbar>
			<div class="flex items-center gap-2 font-mono text-[11px] text-muted">
				<span>Config: <strong class="text-highlighted">JSON v1.0</strong></span>
				<span>•</span>
				<span>Active: <strong class="text-primary">{{ activeCampaign.name }}</strong></span>
				<span>•</span>
				<span>Proxies: <strong class="text-success">{{ proxies.length }}</strong></span>
			</div>

			<div class="flex items-center gap-2">
				<UButton
					size="xs"
					variant="subtle"
					color="neutral"
					icon="i-lucide-upload"
					@click="triggerFileInput"
				>
					Open Settings (.json)
				</UButton>
				<UButton
					size="xs"
					variant="subtle"
					color="neutral"
					icon="i-lucide-download"
					@click="handleExport"
				>
					Export (.json)
				</UButton>
			</div>
		</AppToolbar>

		<!-- Multi-Machine Portability (Export & Import / Open Settings) -->
		<UCard variant="subtle" class="space-y-4">
			<template #header>
				<div class="flex items-center justify-between">
					<UPopover mode="hover" :open-delay="200" :close-delay="150" arrow :content="{ side: 'bottom', align: 'start', sideOffset: 8 }">
						<div class="flex items-center gap-2 cursor-pointer">
							<UIcon name="i-lucide-laptop" class="size-4 text-primary" />
							<h3 class="text-sm font-semibold text-highlighted">Multi-Laptop Sync & Backup (Export / Open Settings)</h3>
							<UIcon name="i-lucide-info" class="size-3.5 text-muted" />
						</div>
						<template #content>
							<div class="p-3.5 max-w-72 text-xs space-y-1.5 bg-elevated border border-default rounded-lg shadow-xl">
								<div class="font-semibold text-highlighted">Portable Settings Architecture</div>
								<p class="text-muted leading-relaxed text-[11px]">
									Allows seamless configuration portability across any Windows laptop or workstation without having to re-configure campaigns or API keys.
								</p>
							</div>
						</template>
					</UPopover>
					<UBadge size="xs" color="primary" variant="subtle">
						Portable Config v1.0
					</UBadge>
				</div>
			</template>

			<div class="space-y-4">
				<p class="text-xs text-muted leading-relaxed">
					To run FlashTraffic on another laptop or PC without re-entering credentials and campaign parameters:
					export your configuration to a <code class="text-primary font-mono">.json</code> file, transfer the file (via USB drive, cloud, or network),
					and click <strong>Open / Import Settings</strong> on the target machine.
				</p>

				<!-- Current Configuration Summary Pills with Hover Popovers -->
				<div class="grid grid-cols-2 sm:grid-cols-4 gap-2.5 text-xs">
					<UPopover mode="hover" :open-delay="200" :close-delay="150" arrow :content="{ side: 'top', align: 'center', sideOffset: 8 }">
						<div class="p-3 rounded-xl bg-elevated/60 border border-default cursor-pointer transition hover:border-accented">
							<div class="text-[11px] text-muted">BrowserStack</div>
							<div class="font-medium text-highlighted truncate mt-0.5">
								{{ browserStackConfig.username ? browserStackConfig.username : '(Not set)' }}
							</div>
						</div>
						<template #content>
							<div class="p-3 max-w-56 text-xs bg-elevated border border-default rounded-lg shadow-xl text-muted">
								Cloud automation credentials and parallel worker allocation.
							</div>
						</template>
					</UPopover>

					<UPopover mode="hover" :open-delay="200" :close-delay="150" arrow :content="{ side: 'top', align: 'center', sideOffset: 8 }">
						<div class="p-3 rounded-xl bg-elevated/60 border border-default cursor-pointer transition hover:border-accented">
							<div class="text-[11px] text-muted">Target URLs</div>
							<div class="font-medium text-highlighted truncate mt-0.5">
								{{ activeCampaign.targetUrls.length }} URL(s)
							</div>
						</div>
						<template #content>
							<div class="p-3 max-w-56 text-xs bg-elevated border border-default rounded-lg shadow-xl text-muted">
								All destination landing pages and article URLs configured in this campaign.
							</div>
						</template>
					</UPopover>

					<UPopover mode="hover" :open-delay="200" :close-delay="150" arrow :content="{ side: 'top', align: 'center', sideOffset: 8 }">
						<div class="p-3 rounded-xl bg-elevated/60 border border-default cursor-pointer transition hover:border-accented">
							<div class="text-[11px] text-muted">Proxy List</div>
							<div class="font-medium text-highlighted truncate mt-0.5">
								{{ proxies.length }} Proxies
							</div>
						</div>
						<template #content>
							<div class="p-3 max-w-56 text-xs bg-elevated border border-default rounded-lg shadow-xl text-muted">
								Active rotating proxy IP pool used for geo-distribution.
							</div>
						</template>
					</UPopover>

					<UPopover mode="hover" :open-delay="200" :close-delay="150" arrow :content="{ side: 'top', align: 'center', sideOffset: 8 }">
						<div class="p-3 rounded-xl bg-elevated/60 border border-default cursor-pointer transition hover:border-accented">
							<div class="text-[11px] text-muted">Simulation Mode</div>
							<div class="font-medium text-primary truncate mt-0.5 uppercase">
								{{ activeCampaign.trafficMode || 'DEFAULT' }}
							</div>
						</div>
						<template #content>
							<div class="p-3 max-w-56 text-xs bg-elevated border border-default rounded-lg shadow-xl text-muted">
								Current optimization strategy (Min Bounce, Max Pageviews, Max Sessions, etc.).
							</div>
						</template>
					</UPopover>
				</div>

				<!-- Action Buttons -->
				<div class="flex flex-wrap items-center gap-3 pt-2 border-t border-default">
					<UButton
						color="primary"
						icon="i-lucide-download"
						size="xs"
						label="Export Configuration (.json)"
						@click="handleExport"
					/>

					<UButton
						color="neutral"
						variant="outline"
						icon="i-lucide-folder-open"
						size="xs"
						label="Open / Import Settings (.json)"
						@click="triggerFileInput"
					/>
				</div>
			</div>
		</UCard>

		<!-- Background & System Tray -->
		<UCard variant="subtle" class="space-y-4">
			<template #header>
				<div class="flex items-center gap-2">
					<UIcon name="i-lucide-cpu" class="size-4 text-primary" />
					<h3 class="text-sm font-semibold text-highlighted">System Tray & Background Execution</h3>
				</div>
			</template>

			<div class="space-y-4">
				<div class="flex items-center justify-between p-3 rounded-xl bg-elevated/50 border border-default">
					<div>
						<div class="text-sm font-medium text-highlighted">Minimize to System Tray</div>
						<p class="text-xs text-muted">Keep traffic campaigns running in background when window is closed or minimized.</p>
					</div>
					<USwitch v-model="minimizeToTray" />
				</div>

				<div class="flex items-center justify-between p-3 rounded-xl bg-elevated/50 border border-default">
					<div>
						<div class="text-sm font-medium text-highlighted">Clear Execution Log Hourly</div>
						<p class="text-xs text-muted">Automatically trims event log buffer to prevent excessive UI memory usage.</p>
					</div>
					<USwitch v-model="clearLogHourly" />
				</div>
			</div>
		</UCard>

		<!-- Task Scheduler -->
		<UCard variant="subtle" class="space-y-4">
			<template #header>
				<div class="flex items-center gap-2">
					<UIcon name="i-lucide-calendar" class="size-4 text-primary" />
					<h3 class="text-sm font-semibold text-highlighted">Automated Campaign Scheduler</h3>
				</div>
			</template>

			<div class="space-y-4">
				<div class="flex items-center justify-between p-3 rounded-xl bg-elevated/50 border border-default">
					<div>
						<div class="text-sm font-medium text-highlighted">Scheduled Daily Start Time</div>
						<p class="text-xs text-muted">Automatically start the active campaign at a specified daily hour.</p>
					</div>
					<USwitch v-model="scheduleEnabled" />
				</div>

				<div v-if="scheduleEnabled" class="grid grid-cols-1 md:grid-cols-2 gap-4 pt-2">
					<UFormField label="Daily Start Time (24h)">
						<UInput v-model="scheduleStartTime" type="time" class="w-full" />
					</UFormField>
				</div>
			</div>
		</UCard>

		<!-- Safety & Clean Exit -->
		<UCard variant="subtle" class="space-y-4">
			<template #header>
				<div class="flex items-center gap-2">
					<UIcon name="i-lucide-shield-alert" class="size-4 text-primary" />
					<h3 class="text-sm font-semibold text-highlighted">Safety & Resource Protection</h3>
				</div>
			</template>

			<div class="space-y-3 text-xs">
				<div class="p-3 rounded-lg bg-elevated/50 border border-default flex items-center justify-between">
					<span class="text-muted">Anti-Orphan Process Watchdog:</span>
					<UBadge size="xs" color="primary" variant="subtle">Active (Tokio PID Tracker)</UBadge>
				</div>
				<div class="p-3 rounded-lg bg-elevated/50 border border-default flex items-center justify-between">
					<span class="text-muted">Isolated Network Contexts:</span>
					<UBadge size="xs" color="primary" variant="subtle">Enabled (No WinInet Override)</UBadge>
				</div>
			</div>
		</UCard>
	</div>
</template>
