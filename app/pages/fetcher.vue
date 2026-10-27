<script setup lang="ts">
import { useTrafficBackend } from "~/composables/useTrafficBackend";
import { useTrafficState } from "~/composables/useTrafficState";

const { activeCampaign, addLog } = useTrafficState();
const backend = useTrafficBackend();
const toast = useToast();

const targetDomain = ref("https://permadi.dev");
const isCrawling = ref(false);
const discoveredUrls = ref<string[]>([]);
const searchQuery = ref("");
const selectedUrls = ref<string[]>([]);

const isUrlInCampaign = (url: string) => {
	return activeCampaign.value.targetUrls.includes(url);
};

const hasIndonesianRoutes = computed(() => {
	return discoveredUrls.value.some(u => u.includes("/id/"));
});

const filteredUrls = computed(() => {
	if (!searchQuery.value.trim()) return discoveredUrls.value;
	const q = searchQuery.value.toLowerCase().trim();
	return discoveredUrls.value.filter(u => u.toLowerCase().includes(q));
});

// Calculate selected URLs strictly within the current filtered view
const selectedFilteredUrls = computed(() => {
	return filteredUrls.value.filter(u => selectedUrls.value.includes(u));
});

// The actual number of selected items in the user's active view
const activeSelectionCount = computed(() => {
	return searchQuery.value.trim() ? selectedFilteredUrls.value.length : selectedUrls.value.length;
});

const isAllFilteredSelected = computed(() => {
	if (filteredUrls.value.length === 0) return false;
	return filteredUrls.value.every(u => selectedUrls.value.includes(u));
});

const isSomeFilteredSelected = computed(() => {
	if (filteredUrls.value.length === 0) return false;
	return filteredUrls.value.some(u => selectedUrls.value.includes(u)) && !isAllFilteredSelected.value;
});

const toggleSelectAllFiltered = () => {
	if (isAllFilteredSelected.value) {
		// Deselect all items in the current filter view
		const filteredSet = new Set(filteredUrls.value);
		selectedUrls.value = selectedUrls.value.filter(u => !filteredSet.has(u));
	} else {
		// Select all items in the current filter view
		const newSet = new Set([...selectedUrls.value, ...filteredUrls.value]);
		selectedUrls.value = Array.from(newSet);
	}
};

const toggleSelectOne = (url: string) => {
	if (selectedUrls.value.includes(url)) {
		selectedUrls.value = selectedUrls.value.filter(u => u !== url);
	} else {
		selectedUrls.value.push(url);
	}
};

const clearAllSelection = () => {
	selectedUrls.value = [];
};

const setQuickFilter = (query: string) => {
	searchQuery.value = query;
};

const handleCrawl = async () => {
	if (!targetDomain.value.trim()) return;
	isCrawling.value = true;
	addLog("INFO", `Starting deep link crawler for domain: ${targetDomain.value}...`);

	try {
		const urls = await backend.fetchSitemapUrls(targetDomain.value);
		discoveredUrls.value = urls;
		// Keep selection empty initially so user is in complete control
		selectedUrls.value = [];
		searchQuery.value = "";
		addLog("SUCCESS", `Discovered ${urls.length} internal links from ${targetDomain.value}.`);
		toast.add({
			title: "Website Crawl Complete",
			description: `Found ${urls.length} unique pages from ${targetDomain.value}. Select pages or click Import All.`,
			color: "success",
			icon: "i-lucide-globe"
		});
	} catch (err: any) {
		addLog("ERROR", `Crawler error: ${err}`);
		toast.add({
			title: "Crawler Failed",
			description: String(err),
			color: "error",
			icon: "i-lucide-alert-triangle"
		});
	} finally {
		isCrawling.value = false;
	}
};

const importUrls = (urlsToImport: string[], label: string) => {
	let added = 0;
	for (const url of urlsToImport) {
		if (!activeCampaign.value.targetUrls.includes(url)) {
			activeCampaign.value.targetUrls.push(url);
			added++;
		}
	}
	addLog("SUCCESS", `Added ${added} crawled URLs into active campaign.`);

	if (added > 0) {
		toast.add({
			title: "URLs Imported to Campaign",
			description: `Successfully added ${added} landing pages (${label}) to '${activeCampaign.value.name}' targeting pool.`,
			color: "success",
			icon: "i-lucide-check-circle-2"
		});
	} else {
		toast.add({
			title: "Already Up to Date",
			description: "All selected URLs are already present in the active campaign.",
			color: "info",
			icon: "i-lucide-info"
		});
	}
};

const importSelected = () => {
	// If a search filter is active, import ONLY the selected items that match the filter!
	const toImport = searchQuery.value.trim() ? selectedFilteredUrls.value : selectedUrls.value;
	if (toImport.length === 0) return;
	importUrls(toImport, `${toImport.length} Selected`);
};

const importAll = () => {
	if (discoveredUrls.value.length === 0) return;
	importUrls(discoveredUrls.value, `All ${discoveredUrls.value.length} Pages`);
};

const importSingle = (url: string) => {
	importUrls([url], "1 Page");
};
</script>

<template>
	<div class="max-w-5xl mx-auto space-y-6">
		<!-- Teleport Title to Row 1 Navbar -->
		<AppHeaderTitle>
			<div class="flex items-center gap-2">
				<span class="text-muted">Crawler</span>
				<span class="text-muted/60">/</span>
				<span class="flex items-center gap-1.5 font-bold text-highlighted">
					<UIcon name="i-lucide-globe" class="size-4 text-primary" />
					Internal Link Crawler
				</span>
			</div>
		</AppHeaderTitle>

		<!-- Teleport Actions to Row 1 Navbar -->
		<AppHeaderActions>
			<UButton
				to="/campaigns"
				variant="outline"
				color="neutral"
				icon="i-lucide-arrow-right"
				size="xs"
			>
				View Campaign
			</UButton>
		</AppHeaderActions>

		<!-- Teleport Toolbar to Row 2 Sub-Navbar -->
		<AppToolbar>
			<div class="flex items-center gap-2 font-mono text-[11px] text-muted">
				<span>Target: <strong class="text-highlighted">{{ targetDomain || 'None' }}</strong></span>
				<span>•</span>
				<span>Discovered: <strong class="text-primary">{{ discoveredUrls.length }}</strong> URLs</span>
				<span v-if="activeSelectionCount > 0">•</span>
				<span v-if="activeSelectionCount > 0">
					Selected: <strong class="text-success">{{ activeSelectionCount }}</strong>
					<span v-if="searchQuery.trim()" class="text-muted/70"> (filtered)</span>
				</span>
			</div>

			<div class="flex items-center gap-2">
				<UButton
					v-if="activeSelectionCount > 0"
					color="primary"
					icon="i-lucide-check-circle"
					size="xs"
					@click="importSelected"
				>
					Import Selected ({{ activeSelectionCount }})
				</UButton>
				<UButton
					v-if="discoveredUrls.length > 0"
					color="primary"
					variant="subtle"
					icon="i-lucide-plus-circle"
					size="xs"
					@click="importAll"
				>
					Import All ({{ discoveredUrls.length }})
				</UButton>
			</div>
		</AppToolbar>

		<!-- Scanner Card -->
		<UCard variant="subtle">
			<template #header>
				<div class="flex items-center gap-2">
					<UIcon name="i-lucide-search" class="size-4 text-primary" />
					<h3 class="text-sm font-semibold text-highlighted">Target Domain Scanner</h3>
				</div>
			</template>

			<div class="flex gap-2">
				<UInput
					v-model="targetDomain"
					placeholder="https://yourwebsite.com"
					class="flex-1"
					icon="i-lucide-globe"
					@keydown.enter="handleCrawl"
				/>
				<UButton
					color="primary"
					icon="i-lucide-search"
					:loading="isCrawling"
					@click="handleCrawl"
				>
					Scan Website
				</UButton>
			</div>
		</UCard>

		<!-- Discovered Links Table Card -->
		<UCard v-if="discoveredUrls.length > 0" variant="subtle">
			<template #header>
				<div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
					<div class="flex items-center gap-2">
						<UIcon name="i-lucide-list" class="size-4 text-primary" />
						<h3 class="text-sm font-semibold text-highlighted">Discovered Pages Table</h3>
						<UBadge size="xs" color="primary" variant="subtle">{{ discoveredUrls.length }} Total</UBadge>
						<UBadge v-if="activeSelectionCount > 0" size="xs" color="success" variant="subtle">
							{{ activeSelectionCount }} Selected
						</UBadge>
					</div>

					<div class="flex flex-wrap items-center gap-2">
						<UButton
							size="xs"
							color="primary"
							icon="i-lucide-check-square"
							:disabled="activeSelectionCount === 0"
							@click="importSelected"
						>
							Import Selected ({{ activeSelectionCount }})
						</UButton>
						<UButton
							size="xs"
							variant="outline"
							color="neutral"
							icon="i-lucide-plus-circle"
							@click="importAll"
						>
							Import All ({{ discoveredUrls.length }})
						</UButton>
						<UButton
							size="xs"
							variant="ghost"
							color="neutral"
							to="/campaigns"
							icon="i-lucide-arrow-right"
						>
							View in Campaign
						</UButton>
					</div>
				</div>
			</template>

			<div class="space-y-4">
				<!-- Filter Bar with Locale Pills -->
				<div class="flex flex-col gap-2 pb-3 border-b border-default">
					<div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
						<div class="flex items-center gap-2 flex-1 max-w-md">
							<UInput
								v-model="searchQuery"
								placeholder="Filter pages by keyword or path (e.g. id/blog/)..."
								icon="i-lucide-filter"
								size="xs"
								class="flex-1"
							/>
							<UButton
								v-if="searchQuery"
								size="xs"
								variant="ghost"
								color="neutral"
								icon="i-lucide-x"
								@click="searchQuery = ''"
							/>
						</div>

						<div class="flex items-center gap-2 text-xs">
							<UButton
								size="xs"
								variant="link"
								color="primary"
								@click="toggleSelectAllFiltered"
							>
								{{ isAllFilteredSelected ? 'Deselect Filtered' : 'Select All Filtered' }}
							</UButton>
							<span v-if="selectedUrls.length > 0" class="text-muted/60">•</span>
							<UButton
								v-if="selectedUrls.length > 0"
								size="xs"
								variant="link"
								color="error"
								@click="clearAllSelection"
							>
								Clear All ({{ selectedUrls.length }})
							</UButton>
						</div>
					</div>

					<!-- Quick Language/Path Filter Pills -->
					<div v-if="hasIndonesianRoutes" class="flex items-center gap-1.5 text-xs pt-1">
						<span class="text-muted text-[11px]">Quick Filter:</span>
						<UButton
							size="xs"
							:variant="searchQuery === '' ? 'solid' : 'subtle'"
							color="neutral"
							class="text-[11px] py-0.5 px-2"
							@click="setQuickFilter('')"
						>
							Semua ({{ discoveredUrls.length }})
						</UButton>
						<UButton
							size="xs"
							:variant="searchQuery === 'id/' ? 'solid' : 'subtle'"
							color="primary"
							class="text-[11px] py-0.5 px-2"
							@click="setQuickFilter('id/')"
						>
							🇮🇩 Hanya /id/ (ID)
						</UButton>
						<UButton
							size="xs"
							:variant="searchQuery === 'blog/' ? 'solid' : 'subtle'"
							color="neutral"
							class="text-[11px] py-0.5 px-2"
							@click="setQuickFilter('blog/')"
						>
							Semua Blog
						</UButton>
					</div>
				</div>

				<!-- Interactive Table -->
				<div class="overflow-x-auto max-h-[460px] overflow-y-auto rounded-lg border border-default">
					<table class="w-full text-left text-xs font-mono">
						<thead class="sticky top-0 bg-elevated border-b border-default text-muted z-10">
							<tr>
								<th class="py-2.5 px-3 w-10">
									<UCheckbox
										:model-value="isAllFilteredSelected ? true : isSomeFilteredSelected ? 'indeterminate' : false"
										@update:model-value="toggleSelectAllFiltered"
									/>
								</th>
								<th class="py-2.5 px-3 w-12 text-muted">#</th>
								<th class="py-2.5 px-3">Page URL Path</th>
								<th class="py-2.5 px-3 w-28 text-center">Campaign Status</th>
								<th class="py-2.5 px-3 w-24 text-right">Action</th>
							</tr>
						</thead>
						<tbody class="divide-y divide-default bg-default">
							<tr
								v-for="(url, idx) in filteredUrls"
								:key="url"
								class="hover:bg-elevated/50 transition-colors"
								:class="selectedUrls.includes(url) ? 'bg-primary/10' : ''"
							>
								<td class="py-2.5 px-3">
									<UCheckbox
										:model-value="selectedUrls.includes(url)"
										@update:model-value="toggleSelectOne(url)"
									/>
								</td>
								<td class="py-2.5 px-3 text-muted">{{ idx + 1 }}</td>
								<td class="py-2.5 px-3">
									<span class="text-highlighted font-medium truncate block max-w-[500px]" :title="url">
										{{ url }}
									</span>
								</td>
								<td class="py-2.5 px-3 text-center">
									<UBadge
										size="xs"
										:variant="isUrlInCampaign(url) ? 'solid' : 'subtle'"
										:color="isUrlInCampaign(url) ? 'success' : 'neutral'"
									>
										{{ isUrlInCampaign(url) ? 'Imported' : 'Ready' }}
									</UBadge>
								</td>
								<td class="py-2.5 px-3 text-right">
									<UButton
										v-if="!isUrlInCampaign(url)"
										size="xs"
										variant="ghost"
										color="primary"
										icon="i-lucide-plus"
										label="Import"
										@click="importSingle(url)"
									/>
									<span v-else class="text-[11px] text-muted font-sans">In Campaign</span>
								</td>
							</tr>
							<tr v-if="filteredUrls.length === 0">
								<td colspan="5" class="py-8 text-center text-muted font-sans">
									No pages match your filter query.
								</td>
							</tr>
						</tbody>
					</table>
				</div>

				<!-- Footer Selection Summary -->
				<div class="flex items-center justify-between text-xs text-muted pt-2 border-t border-default">
					<span>
						Showing {{ filteredUrls.length }} of {{ discoveredUrls.length }} pages
					</span>
					<span>
						{{ activeSelectionCount }} item(s) selected for import
					</span>
				</div>
			</div>
		</UCard>
	</div>
</template>
