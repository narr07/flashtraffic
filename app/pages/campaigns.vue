<script setup lang="ts">
import { useTrafficState } from "~/composables/useTrafficState";

const { activeCampaign, addLog } = useTrafficState();
const toast = useToast();

const organicSources = [
	"Google",
	"Yahoo",
	"Bing",
	"AOL",
	"Baidu",
	"Yandex",
	"Ask",
	"DuckDuckGo",
	"Ecosia",
	"Brave",
	"Naver",
	"Seznam",
	"Qwant"
];

const referralSources = [
	"Facebook",
	"Twitter",
	"YouTube",
	"LinkedIn",
	"Instagram",
	"Pinterest",
	"Flickr",
	"Reddit",
	"TikTok",
	"Threads",
	"Custom website"
];

const engineIcons: Record<string, string> = {
	Google: "i-simple-icons-google",
	Yahoo: "i-simple-icons-yahoo",
	Bing: "i-simple-icons-microsoftbing",
	AOL: "i-simple-icons-aol",
	Baidu: "i-simple-icons-baidu",
	Yandex: "i-simple-icons-yandexcloud",
	Ask: "i-simple-icons-askfm",
	DuckDuckGo: "i-simple-icons-duckduckgo",
	Ecosia: "i-simple-icons-ecosia",
	Brave: "i-simple-icons-brave",
	Naver: "i-simple-icons-naver",
	Seznam: "i-lucide-search",
	Qwant: "i-simple-icons-qwant"
};

const referralIcons: Record<string, string> = {
	Facebook: "i-simple-icons-facebook",
	Twitter: "i-simple-icons-x",
	YouTube: "i-simple-icons-youtube",
	LinkedIn: "i-simple-icons-linkedin",
	Instagram: "i-simple-icons-instagram",
	Pinterest: "i-simple-icons-pinterest",
	Flickr: "i-simple-icons-flickr",
	Reddit: "i-simple-icons-reddit",
	TikTok: "i-simple-icons-tiktok",
	Threads: "i-simple-icons-threads",
	"Custom website": "i-lucide-globe"
};

// Ensure activeCampaign has defaults for new fields
if (!activeCampaign.value.searchEngines || activeCampaign.value.searchEngines.length === 0) {
	activeCampaign.value.searchEngines = ["Google"];
}
if (!activeCampaign.value.referralSources || activeCampaign.value.referralSources.length === 0) {
	activeCampaign.value.referralSources = ["Facebook"];
}
if (activeCampaign.value.customReferralUrl === undefined) {
	activeCampaign.value.customReferralUrl = "";
}
if (!activeCampaign.value.keywordList || activeCampaign.value.keywordList.length === 0) {
	activeCampaign.value.keywordList = activeCampaign.value.keywords
		? activeCampaign.value.keywords.split(/[\n,]/).map(k => k.trim()).filter(k => k.length > 0)
		: ["best web analytics tools 2026"];
}

// ========================
// Target URLs Logic
// ========================
const newUrlInput = ref("");
const searchUrlQuery = ref("");
const selectedUrls = ref<string[]>([]);

const addUrl = () => {
	const trimmed = newUrlInput.value.trim();
	if (trimmed && !activeCampaign.value.targetUrls.includes(trimmed)) {
		activeCampaign.value.targetUrls.push(trimmed);
		newUrlInput.value = "";
		addLog("INFO", `Added target URL: ${trimmed}`);
		toast.add({
			title: "URL Added",
			description: trimmed,
			color: "success",
			icon: "i-lucide-plus-circle"
		});
	}
};

const filteredUrls = computed(() => {
	if (!searchUrlQuery.value.trim()) return activeCampaign.value.targetUrls;
	const q = searchUrlQuery.value.toLowerCase();
	return activeCampaign.value.targetUrls.filter(u => u.toLowerCase().includes(q));
});

const isAllFilteredUrlsSelected = computed(() => {
	if (filteredUrls.value.length === 0) return false;
	return filteredUrls.value.every(u => selectedUrls.value.includes(u));
});

const isSomeFilteredUrlsSelected = computed(() => {
	if (filteredUrls.value.length === 0) return false;
	return filteredUrls.value.some(u => selectedUrls.value.includes(u)) && !isAllFilteredUrlsSelected.value;
});

const toggleSelectAllUrls = () => {
	if (isAllFilteredUrlsSelected.value) {
		selectedUrls.value = selectedUrls.value.filter(u => !filteredUrls.value.includes(u));
	} else {
		const newSet = new Set([...selectedUrls.value, ...filteredUrls.value]);
		selectedUrls.value = Array.from(newSet);
	}
};

const toggleSelectOneUrl = (url: string) => {
	if (selectedUrls.value.includes(url)) {
		selectedUrls.value = selectedUrls.value.filter(u => u !== url);
	} else {
		selectedUrls.value.push(url);
	}
};

const removeSingleUrl = (url: string) => {
	activeCampaign.value.targetUrls = activeCampaign.value.targetUrls.filter(u => u !== url);
	selectedUrls.value = selectedUrls.value.filter(u => u !== url);
	addLog("INFO", `Removed target URL: ${url}`);
	toast.add({
		title: "URL Removed",
		description: url,
		color: "neutral",
		icon: "i-lucide-trash"
	});
};

const removeSelectedUrls = () => {
	const count = selectedUrls.value.length;
	if (count === 0) return;

	activeCampaign.value.targetUrls = activeCampaign.value.targetUrls.filter(
		u => !selectedUrls.value.includes(u)
	);
	selectedUrls.value = [];
	addLog("INFO", `Removed ${count} selected URLs from campaign.`);
	toast.add({
		title: "Selected URLs Deleted",
		description: `Successfully deleted ${count} landing page URLs from campaign.`,
		color: "neutral",
		icon: "i-lucide-trash-2"
	});
};

const clearAllUrls = () => {
	const count = activeCampaign.value.targetUrls.length;
	if (count === 0) return;
	activeCampaign.value.targetUrls = [];
	selectedUrls.value = [];
	addLog("WARN", `Cleared all ${count} target URLs from campaign.`);
	toast.add({
		title: "All URLs Cleared",
		description: `Removed ${count} URLs from campaign targeting pool.`,
		color: "warning",
		icon: "i-lucide-trash"
	});
};

// ========================
// Search Engines Multi-Select Logic
// ========================
const toggleEngine = (eng: string) => {
	const current = activeCampaign.value.searchEngines || [];
	if (current.includes(eng)) {
		if (current.length > 1) {
			activeCampaign.value.searchEngines = current.filter(e => e !== eng);
		} else {
			toast.add({
				title: "At Least One Engine Required",
				description: "You must keep at least 1 search engine selected.",
				color: "warning"
			});
		}
	} else {
		activeCampaign.value.searchEngines = [...current, eng];
	}
	if (activeCampaign.value.searchEngines.length > 0) {
		activeCampaign.value.trafficSource = activeCampaign.value.searchEngines[0] || "Google";
	}
};

const selectAllEngines = () => {
	activeCampaign.value.searchEngines = [...organicSources];
	activeCampaign.value.trafficSource = organicSources[0] || "Google";
};

const selectGoogleOnly = () => {
	activeCampaign.value.searchEngines = ["Google"];
	activeCampaign.value.trafficSource = "Google";
};

// ========================
// Social Media / Referral Multi-Select Logic
// ========================
const toggleReferralSource = (source: string) => {
	const current = activeCampaign.value.referralSources || [];
	if (current.includes(source)) {
		if (current.length > 1) {
			activeCampaign.value.referralSources = current.filter(s => s !== source);
		} else {
			toast.add({
				title: "At Least One Source Required",
				description: "You must keep at least 1 referral source selected.",
				color: "warning"
			});
		}
	} else {
		activeCampaign.value.referralSources = [...current, source];
	}
	if (activeCampaign.value.referralSources.length > 0) {
		activeCampaign.value.trafficSource = activeCampaign.value.referralSources[0] || "Facebook";
	}
};

const selectAllReferralSources = () => {
	activeCampaign.value.referralSources = referralSources.filter(s => s !== "Custom website");
	activeCampaign.value.trafficSource = referralSources[0] || "Facebook";
};

const selectSocialMajorOnly = () => {
	activeCampaign.value.referralSources = ["Facebook", "Twitter", "Instagram", "YouTube"];
	activeCampaign.value.trafficSource = "Facebook";
};

// ========================
// Keywords Pool & Import/Export Logic
// ========================
const newKeywordInput = ref("");
const searchKeywordQuery = ref("");
const selectedKeywords = ref<string[]>([]);
const keywordFileInput = ref<HTMLInputElement | null>(null);

const syncKeywordsString = () => {
	activeCampaign.value.keywords = activeCampaign.value.keywordList.join("\n");
};

const addKeyword = () => {
	const trimmed = newKeywordInput.value.trim();
	if (trimmed && !activeCampaign.value.keywordList.includes(trimmed)) {
		activeCampaign.value.keywordList.push(trimmed);
		newKeywordInput.value = "";
		syncKeywordsString();
		addLog("INFO", `Added target keyword: "${trimmed}"`);
		toast.add({
			title: "Keyword Added",
			description: trimmed,
			color: "success",
			icon: "i-lucide-plus-circle"
		});
	}
};

const filteredKeywords = computed(() => {
	if (!searchKeywordQuery.value.trim()) return activeCampaign.value.keywordList;
	const q = searchKeywordQuery.value.toLowerCase();
	return activeCampaign.value.keywordList.filter(k => k.toLowerCase().includes(q));
});

const isAllFilteredKeywordsSelected = computed(() => {
	if (filteredKeywords.value.length === 0) return false;
	return filteredKeywords.value.every(k => selectedKeywords.value.includes(k));
});

const isSomeFilteredKeywordsSelected = computed(() => {
	if (filteredKeywords.value.length === 0) return false;
	return filteredKeywords.value.some(k => selectedKeywords.value.includes(k)) && !isAllFilteredKeywordsSelected.value;
});

const toggleSelectAllKeywords = () => {
	if (isAllFilteredKeywordsSelected.value) {
		selectedKeywords.value = selectedKeywords.value.filter(k => !filteredKeywords.value.includes(k));
	} else {
		const newSet = new Set([...selectedKeywords.value, ...filteredKeywords.value]);
		selectedKeywords.value = Array.from(newSet);
	}
};

const toggleSelectOneKeyword = (kw: string) => {
	if (selectedKeywords.value.includes(kw)) {
		selectedKeywords.value = selectedKeywords.value.filter(k => k !== kw);
	} else {
		selectedKeywords.value.push(kw);
	}
};

const removeSingleKeyword = (kw: string) => {
	activeCampaign.value.keywordList = activeCampaign.value.keywordList.filter(k => k !== kw);
	selectedKeywords.value = selectedKeywords.value.filter(k => k !== kw);
	syncKeywordsString();
	addLog("INFO", `Removed keyword: "${kw}"`);
	toast.add({
		title: "Keyword Removed",
		description: kw,
		color: "neutral",
		icon: "i-lucide-trash"
	});
};

const removeSelectedKeywords = () => {
	const count = selectedKeywords.value.length;
	if (count === 0) return;

	activeCampaign.value.keywordList = activeCampaign.value.keywordList.filter(
		k => !selectedKeywords.value.includes(k)
	);
	selectedKeywords.value = [];
	syncKeywordsString();
	addLog("INFO", `Removed ${count} selected keywords.`);
	toast.add({
		title: "Selected Keywords Deleted",
		description: `Deleted ${count} search keywords from campaign.`,
		color: "neutral",
		icon: "i-lucide-trash-2"
	});
};

const clearAllKeywords = () => {
	const count = activeCampaign.value.keywordList.length;
	if (count === 0) return;
	activeCampaign.value.keywordList = [];
	selectedKeywords.value = [];
	syncKeywordsString();
	addLog("WARN", `Cleared all ${count} keywords from campaign.`);
	toast.add({
		title: "Keywords Cleared",
		description: `Removed ${count} keywords from search pool.`,
		color: "warning",
		icon: "i-lucide-trash"
	});
};

// Import file (.txt, .csv, .md)
const triggerKeywordImport = () => {
	keywordFileInput.value?.click();
};

const handleKeywordFileImport = (event: Event) => {
	const target = event.target as HTMLInputElement;
	const file = target.files?.[0];
	if (!file) return;

	const reader = new FileReader();
	reader.onload = (e) => {
		const text = e.target?.result as string;
		if (!text) return;

		const imported: string[] = [];
		const filename = file.name.toLowerCase();

		if (filename.endsWith(".csv")) {
			// Split by rows and commas, trim and remove quotes
			const lines = text.split(/\r?\n/);
			for (const line of lines) {
				const parts = line.split(",").map(p => p.trim().replace(/^["']|["']$/g, ""));
				for (const part of parts) {
					if (part && !["keyword", "keywords", "query"].includes(part.toLowerCase())) {
						imported.push(part);
					}
				}
			}
		} else if (filename.endsWith(".md")) {
			// Handle markdown bullet points (- or *) or table cells (|) or headings
			const lines = text.split(/\r?\n/);
			for (const line of lines) {
				let trimmed = line.trim();
				if (trimmed.startsWith("- ") || trimmed.startsWith("* ")) {
					trimmed = trimmed.substring(2).trim();
				} else if (trimmed.startsWith("|") && trimmed.endsWith("|")) {
					const cells = trimmed.split("|").map(c => c.trim()).filter(c => c.length > 0 && !c.includes("---"));
					for (const cell of cells) {
						if (cell && !["keyword", "keywords"].includes(cell.toLowerCase())) {
							imported.push(cell);
						}
					}
					continue;
				} else if (trimmed.startsWith("#") || trimmed.startsWith("```")) {
					continue;
				}
				if (trimmed) {
					imported.push(trimmed);
				}
			}
		} else {
			// Plain .txt: one per line
			const lines = text.split(/\r?\n/).map(l => l.trim()).filter(l => l.length > 0);
			imported.push(...lines);
		}

		let addedCount = 0;
		for (const kw of imported) {
			if (!activeCampaign.value.keywordList.includes(kw)) {
				activeCampaign.value.keywordList.push(kw);
				addedCount++;
			}
		}

		syncKeywordsString();
		toast.add({
			title: "Keywords Imported",
			description: `Imported ${addedCount} new keywords from '${file.name}' (Total: ${activeCampaign.value.keywordList.length}).`,
			color: "success",
			icon: "i-lucide-file-up"
		});

		target.value = ""; // Reset
	};

	reader.readAsText(file);
};

// Export file (.txt, .csv, .md)
const exportKeywords = (format: "txt" | "csv" | "md") => {
	if (activeCampaign.value.keywordList.length === 0) {
		toast.add({
			title: "No Keywords to Export",
			description: "Keyword list is currently empty.",
			color: "warning"
		});
		return;
	}

	let content = "";
	let filename = `flashtraffic-keywords-${Date.now()}`;
	let mimeType = "text/plain";

	if (format === "csv") {
		content = `Keyword\n${activeCampaign.value.keywordList.map(k => `"${k.replace(/"/g, "\"\"")}"`).join("\n")}`;
		filename += ".csv";
		mimeType = "text/csv;charset=utf-8;";
	} else if (format === "md") {
		content = `# FlashTraffic Search Keywords (${activeCampaign.value.keywordList.length})\n\n${
			activeCampaign.value.keywordList.map(k => `- ${k}`).join("\n")}\n`;
		filename += ".md";
		mimeType = "text/markdown;charset=utf-8;";
	} else {
		content = activeCampaign.value.keywordList.join("\n");
		filename += ".txt";
		mimeType = "text/plain;charset=utf-8;";
	}

	const blob = new Blob([content], { type: mimeType });
	const url = URL.createObjectURL(blob);
	const link = document.createElement("a");
	link.href = url;
	link.setAttribute("download", filename);
	document.body.appendChild(link);
	link.click();
	document.body.removeChild(link);
	URL.revokeObjectURL(url);

	toast.add({
		title: "Keywords Exported",
		description: `Saved ${activeCampaign.value.keywordList.length} keywords as ${filename}`,
		color: "success",
		icon: "i-lucide-download"
	});
};

// ========================
// Combinatorial Permutation Calculator
// ========================
const totalMatrixPermutations = computed(() => {
	const urls = activeCampaign.value.targetUrls.length || 0;
	if (activeCampaign.value.trafficType === "Organic") {
		const engines = (activeCampaign.value.searchEngines || ["Google"]).length || 1;
		const keywords = activeCampaign.value.keywordList.length || 1;
		return urls * engines * keywords;
	} else if (activeCampaign.value.trafficType === "Referral") {
		const sources = (activeCampaign.value.referralSources || ["Facebook"]).length || 1;
		return urls * sources;
	}
	return urls;
});

const syncQuotaToMatrix = () => {
	activeCampaign.value.maxSessions = totalMatrixPermutations.value;
	toast.add({
		title: "Target Quota Synced",
		description: `Updated campaign Target Sessions to ${totalMatrixPermutations.value} (matches exact full matrix size).`,
		color: "success",
		icon: "i-lucide-check-circle"
	});
};

const trafficChannelItems = [
	{
		value: "Organic",
		label: "Organic Search",
		description: "Simulate queries across Google, Bing, Yahoo & SEO engines",
		icon: "i-lucide-search"
	},
	{
		value: "Referral",
		label: "Social & Referral",
		description: "Simulate inbound clicks from Facebook, Twitter, Reddit & blogs",
		icon: "i-lucide-share-2"
	},
	{
		value: "Direct",
		label: "Direct Navigation",
		description: "Simulate clean direct browser address bar inputs & bookmarks",
		icon: "i-lucide-compass"
	}
];

const radioChannelUi = {
	fieldset: "grid grid-cols-1 sm:grid-cols-3 gap-2.5",
	item: "p-3 cursor-pointer",
	label: "text-xs font-semibold",
	description: "text-[11px]"
};

const saveCampaign = () => {
	addLog("SUCCESS", `Campaign "${activeCampaign.value.name}" configuration saved.`);
	toast.add({
		title: "Campaign Saved",
		description: `Saved ${activeCampaign.value.targetUrls.length} URLs with ${activeCampaign.value.trafficType} channel.`,
		color: "success",
		icon: "i-lucide-check-circle"
	});
};
</script>

<template>
	<div class="max-w-5xl mx-auto space-y-6">
		<!-- Teleport Title to Row 1 Navbar -->
		<AppHeaderTitle>
			<div class="flex items-center gap-2">
				<span class="text-muted">Campaign</span>
				<span class="text-muted/60">/</span>
				<span class="flex items-center gap-1.5 font-bold text-highlighted">
					<UIcon name="i-lucide-megaphone" class="size-4 text-primary" />
					Campaign Studio
				</span>
			</div>
		</AppHeaderTitle>

		<!-- Teleport Action to Row 1 Navbar -->
		<AppHeaderActions>
			<UButton icon="i-lucide-save" size="xs" color="primary" @click="saveCampaign">
				Save Campaign
			</UButton>
		</AppHeaderActions>

		<!-- Teleport Toolbar to Row 2 Sub-Navbar -->
		<AppToolbar>
			<div class="flex items-center gap-2 font-mono text-[11px] text-muted">
				<span>Targets: <strong class="text-highlighted">{{ activeCampaign.targetUrls.length }}</strong> URLs</span>
				<span>•</span>
				<span>Channel: <UBadge size="xs" variant="subtle" color="primary">{{ activeCampaign.trafficType }}</UBadge></span>
				<span>•</span>
				<span>Total Permutations: <strong class="text-highlighted">{{ totalMatrixPermutations }}</strong></span>
			</div>

			<div class="flex items-center gap-2">
				<UButton
					v-if="activeCampaign.trafficType !== 'Direct'"
					color="primary"
					variant="subtle"
					icon="i-lucide-cpu"
					size="xs"
					@click="syncQuotaToMatrix"
				>
					Set Quota to Matrix ({{ totalMatrixPermutations }})
				</UButton>
			</div>
		</AppToolbar>

		<!-- Campaign General Info -->
		<UCard variant="subtle">
			<template #header>
				<div class="flex items-center gap-2">
					<UIcon name="i-lucide-file-text" class="size-4 text-primary" />
					<h3 class="text-sm font-semibold text-highlighted">General Information</h3>
				</div>
			</template>

			<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
				<UFormField label="Campaign Name" required>
					<UInput v-model="activeCampaign.name" placeholder="e.g. Q4 SEO Growth" class="w-full" />
				</UFormField>

				<UFormField label="Target Sessions (Quota)" required hint="Maximum hits delivered before auto-stopping">
					<UInput v-model.number="activeCampaign.maxSessions" type="number" :min="1" :max="100000" class="w-full" />
				</UFormField>
			</div>
		</UCard>

		<!-- Combinatorial Matrix Summary Banner -->
		<div class="p-4 rounded-xl border border-primary/30 bg-primary/10 flex flex-col md:flex-row md:items-center justify-between gap-4">
			<div class="space-y-1">
				<div class="flex items-center gap-2">
					<UIcon name="i-lucide-cpu" class="text-primary size-4" />
					<span class="text-xs font-bold text-highlighted uppercase tracking-wider">Combinatorial Matrix Distribution</span>
				</div>
				<p class="text-xs text-muted font-mono">
					{{ activeCampaign.targetUrls.length }} URLs
					<template v-if="activeCampaign.trafficType === 'Organic'">
						× {{ (activeCampaign.searchEngines || []).length }} Engines
						× {{ activeCampaign.keywordList.length }} Keywords
					</template>
					<template v-else-if="activeCampaign.trafficType === 'Referral'">
						× {{ (activeCampaign.referralSources || []).length }} Referral Sources
					</template>
					=
					<strong class="text-primary text-sm font-bold">{{ totalMatrixPermutations }} Total Unique Visits</strong>
				</p>
				<p class="text-[11px] text-muted">
					<template v-if="activeCampaign.trafficType === 'Organic'">
						Each landing page is visited once per keyword from each selected search engine, guaranteeing authentic organic spread.
					</template>
					<template v-else-if="activeCampaign.trafficType === 'Referral'">
						Each landing page is visited once from each selected social/referral platform, simulating authentic viral clicks.
					</template>
					<template v-else>
						Direct visits with clean navigation directly to target landing pages.
					</template>
				</p>
			</div>

			<UButton
				size="xs"
				color="primary"
				icon="i-lucide-check-circle"
				@click="syncQuotaToMatrix"
			>
				Set Quota to Matrix ({{ totalMatrixPermutations }})
			</UButton>
		</div>

		<!-- Target URLs List Card with Table -->
		<UCard variant="subtle">
			<template #header>
				<div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
					<div class="flex items-center gap-2">
						<UIcon name="i-lucide-link" class="size-4 text-primary" />
						<h3 class="text-sm font-semibold text-highlighted">Target URLs (Landing Pages)</h3>
						<UBadge size="xs" color="primary" variant="subtle">{{ activeCampaign.targetUrls.length }} URLs</UBadge>
						<UBadge v-if="selectedUrls.length > 0" size="xs" color="error" variant="subtle">
							{{ selectedUrls.length }} Selected
						</UBadge>
					</div>

					<div class="flex flex-wrap items-center gap-2">
						<UButton
							v-if="selectedUrls.length > 0"
							size="xs"
							color="error"
							icon="i-lucide-trash-2"
							@click="removeSelectedUrls"
						>
							Delete Selected ({{ selectedUrls.length }})
						</UButton>

						<UButton
							v-if="activeCampaign.targetUrls.length > 0"
							size="xs"
							variant="ghost"
							color="neutral"
							icon="i-lucide-trash"
							@click="clearAllUrls"
						>
							Clear All
						</UButton>

						<UButton
							size="xs"
							variant="outline"
							color="primary"
							to="/fetcher"
							icon="i-lucide-globe"
						>
							Crawl More URLs
						</UButton>
					</div>
				</div>
			</template>

			<div class="space-y-4">
				<!-- New URL Input Bar -->
				<div class="flex gap-2">
					<UInput
						v-model="newUrlInput"
						placeholder="https://example.com/page-to-test"
						class="flex-1"
						icon="i-lucide-link"
						@keydown.enter="addUrl"
					/>
					<UButton color="primary" icon="i-lucide-plus" @click="addUrl">
						Add URL
					</UButton>
				</div>

				<!-- Filter Bar (shown when URLs exist) -->
				<div v-if="activeCampaign.targetUrls.length > 4" class="flex items-center justify-between gap-3 pt-2 pb-1 border-t border-default">
					<UInput
						v-model="searchUrlQuery"
						placeholder="Filter landing pages by path..."
						icon="i-lucide-filter"
						size="xs"
						class="max-w-xs flex-1"
					/>

					<UButton
						size="xs"
						variant="link"
						color="neutral"
						@click="toggleSelectAllUrls"
					>
						{{ isAllFilteredUrlsSelected ? 'Deselect All' : 'Select All Filtered' }}
					</UButton>
				</div>

				<!-- Interactive URLs Table -->
				<div class="overflow-x-auto max-h-[380px] overflow-y-auto rounded-lg border border-default">
					<table class="w-full text-left text-xs font-mono">
						<thead class="sticky top-0 bg-elevated border-b border-default text-muted z-10">
							<tr>
								<th class="py-2.5 px-3 w-10">
									<UCheckbox
										:model-value="isAllFilteredUrlsSelected ? true : isSomeFilteredUrlsSelected ? 'indeterminate' : false"
										@update:model-value="toggleSelectAllUrls"
									/>
								</th>
								<th class="py-2.5 px-3 w-12 text-muted">#</th>
								<th class="py-2.5 px-3">Landing Page URL</th>
								<th class="py-2.5 px-3 w-20 text-right">Action</th>
							</tr>
						</thead>
						<tbody class="divide-y divide-default bg-default">
							<tr
								v-for="(url, idx) in filteredUrls"
								:key="url"
								class="hover:bg-elevated/50 transition-colors"
								:class="selectedUrls.includes(url) ? 'bg-error/10' : ''"
							>
								<td class="py-2.5 px-3">
									<UCheckbox
										:model-value="selectedUrls.includes(url)"
										@update:model-value="toggleSelectOneUrl(url)"
									/>
								</td>
								<td class="py-2.5 px-3 text-muted">{{ idx + 1 }}</td>
								<td class="py-2.5 px-3">
									<span class="text-highlighted truncate block max-w-[550px]" :title="url">
										{{ url }}
									</span>
								</td>
								<td class="py-2.5 px-3 text-right">
									<UButton
										size="xs"
										color="error"
										variant="ghost"
										icon="i-lucide-trash"
										@click="removeSingleUrl(url)"
									/>
								</td>
							</tr>
							<tr v-if="filteredUrls.length === 0">
								<td colspan="4" class="py-8 text-center text-muted font-sans">
									{{ activeCampaign.targetUrls.length === 0 ? 'No target URLs configured. Add a URL above or import from Link Crawler.' : 'No URLs match your filter query.' }}
								</td>
							</tr>
						</tbody>
					</table>
				</div>

				<!-- Footer Summary -->
				<div v-if="activeCampaign.targetUrls.length > 0" class="flex items-center justify-between text-xs text-muted pt-2 border-t border-default">
					<span>
						Total: {{ activeCampaign.targetUrls.length }} landing pages configured
					</span>
					<span v-if="selectedUrls.length > 0" class="text-error font-medium">
						{{ selectedUrls.length }} URL(s) selected for deletion
					</span>
				</div>
			</div>
		</UCard>

		<!-- Traffic Attribution & Channel -->
		<UCard variant="subtle">
			<template #header>
				<div class="flex items-center gap-2">
					<UIcon name="i-lucide-git-fork" class="size-4 text-primary" />
					<h3 class="text-sm font-semibold text-highlighted">Attribution & Referral Engine</h3>
				</div>
			</template>

			<div class="space-y-4">
				<UFormField label="Traffic Channel">
					<URadioGroup
						v-model="activeCampaign.trafficType"
						:items="trafficChannelItems"
						variant="card"
						orientation="horizontal"
						:ui="radioChannelUi"
					/>
				</UFormField>

				<!-- Multi-Select Search Engines (Organic) -->
				<div v-if="activeCampaign.trafficType === 'Organic'" class="space-y-3 pt-2 border-t border-default">
					<div class="flex flex-col sm:flex-row sm:items-center justify-between gap-2">
						<div>
							<h4 class="text-xs font-semibold text-highlighted">Target Search Engines (Multi-Select)</h4>
							<p class="text-[11px] text-muted">Visitors will be partitioned across all checked search engines.</p>
						</div>
						<div class="flex items-center gap-2">
							<UButton size="xs" variant="link" color="neutral" @click="selectAllEngines">Select All ({{ organicSources.length }})</UButton>
							<span class="text-muted">•</span>
							<UButton size="xs" variant="link" color="neutral" @click="selectGoogleOnly">Google Only</UButton>
						</div>
					</div>

					<div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-2.5">
						<div
							v-for="eng in organicSources"
							:key="eng"
							class="flex items-center gap-2.5 p-2.5 rounded-lg border transition-colors cursor-pointer"
							:class="(activeCampaign.searchEngines || []).includes(eng)
								? 'border-primary/50 bg-primary/10'
								: 'border-default bg-elevated/40 hover:bg-elevated'"
							@click="toggleEngine(eng)"
						>
							<UCheckbox
								:model-value="(activeCampaign.searchEngines || []).includes(eng)"
								@update:model-value="toggleEngine(eng)"
							/>
							<UIcon :name="engineIcons[eng] || 'i-lucide-search'" class="size-4 shrink-0 text-primary" />
							<span class="text-xs font-medium text-highlighted truncate">{{ eng }}</span>
						</div>
					</div>
				</div>

				<!-- Multi-Select Referral / Social Media Sources -->
				<div v-else-if="activeCampaign.trafficType === 'Referral'" class="space-y-4 pt-2 border-t border-default">
					<div class="flex flex-col sm:flex-row sm:items-center justify-between gap-2">
						<div>
							<h4 class="text-xs font-semibold text-highlighted">Social Media & Referral Sources (Multi-Select)</h4>
							<p class="text-[11px] text-muted">Visits simulate clicks from links shared on social networks or blogs.</p>
						</div>
						<div class="flex items-center gap-2">
							<UButton size="xs" variant="link" color="neutral" @click="selectAllReferralSources">Select All Social</UButton>
							<span class="text-muted">•</span>
							<UButton size="xs" variant="link" color="neutral" @click="selectSocialMajorOnly">Major Only (4)</UButton>
						</div>
					</div>

					<div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-2.5">
						<div
							v-for="source in referralSources"
							:key="source"
							class="flex items-center gap-2.5 p-2.5 rounded-lg border transition-colors cursor-pointer"
							:class="(activeCampaign.referralSources || []).includes(source)
								? 'border-primary/50 bg-primary/10'
								: 'border-default bg-elevated/40 hover:bg-elevated'"
							@click="toggleReferralSource(source)"
						>
							<UCheckbox
								:model-value="(activeCampaign.referralSources || []).includes(source)"
								@update:model-value="toggleReferralSource(source)"
							/>
							<UIcon :name="referralIcons[source] || 'i-lucide-globe'" class="size-4 shrink-0 text-primary" />
							<span class="text-xs font-medium text-highlighted truncate">{{ source }}</span>
						</div>
					</div>

					<!-- Custom Referral URL Input (if Custom website checked) -->
					<div v-if="(activeCampaign.referralSources || []).includes('Custom website')" class="pt-2">
						<UFormField label="Custom Referral Website URL" hint="Enter specific blog or partner site to simulate incoming clicks from">
							<UInput
								v-model="activeCampaign.customReferralUrl"
								placeholder="https://partner-portal.com/review-post"
								icon="i-lucide-globe"
								class="w-full"
							/>
						</UFormField>
					</div>
				</div>
			</div>
		</UCard>

		<!-- Search Keywords Pool (UTable + Multi-Select + Import/Export TXT/CSV/MD) -->
		<UCard v-if="activeCampaign.trafficType === 'Organic'" variant="subtle">
			<template #header>
				<div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
					<div class="flex items-center gap-2">
						<UIcon name="i-lucide-search" class="size-4 text-primary" />
						<h3 class="text-sm font-semibold text-highlighted">Search Keywords Pool</h3>
						<UBadge size="xs" color="primary" variant="subtle">{{ activeCampaign.keywordList.length }} Keywords</UBadge>
						<UBadge v-if="selectedKeywords.length > 0" size="xs" color="error" variant="subtle">
							{{ selectedKeywords.length }} Selected
						</UBadge>
					</div>

					<div class="flex flex-wrap items-center gap-2">
						<!-- Hidden File Input for .txt, .csv, .md -->
						<input
							ref="keywordFileInput"
							type="file"
							accept=".txt,.csv,.md"
							class="hidden"
							@change="handleKeywordFileImport"
						>

						<UButton
							size="xs"
							color="primary"
							variant="outline"
							icon="i-lucide-upload"
							@click="triggerKeywordImport"
						>
							Import File (.txt, .csv, .md)
						</UButton>

						<!-- Export Dropdown -->
						<div class="flex items-center gap-1 border border-default rounded-lg p-0.5 bg-elevated/50">
							<span class="text-[11px] text-muted px-1.5 font-medium">Export:</span>
							<UButton size="xs" variant="ghost" color="neutral" @click="exportKeywords('txt')">TXT</UButton>
							<UButton size="xs" variant="ghost" color="neutral" @click="exportKeywords('csv')">CSV</UButton>
							<UButton size="xs" variant="ghost" color="neutral" @click="exportKeywords('md')">MD</UButton>
						</div>

						<UButton
							v-if="selectedKeywords.length > 0"
							size="xs"
							color="error"
							icon="i-lucide-trash-2"
							@click="removeSelectedKeywords"
						>
							Delete Selected ({{ selectedKeywords.length }})
						</UButton>

						<UButton
							v-if="activeCampaign.keywordList.length > 0"
							size="xs"
							variant="ghost"
							color="neutral"
							icon="i-lucide-trash"
							@click="clearAllKeywords"
						>
							Clear All
						</UButton>
					</div>
				</div>
			</template>

			<div class="space-y-4">
				<!-- Add Keyword Input -->
				<div class="flex gap-2">
					<UInput
						v-model="newKeywordInput"
						placeholder="e.g. frontend developer portfolio, best saas tools"
						class="flex-1"
						icon="i-lucide-search"
						@keydown.enter="addKeyword"
					/>
					<UButton color="primary" icon="i-lucide-plus" @click="addKeyword">
						Add Keyword
					</UButton>
				</div>

				<!-- Filter Bar (when keywords exist) -->
				<div v-if="activeCampaign.keywordList.length > 4" class="flex items-center justify-between gap-3 pt-2 pb-1 border-t border-default">
					<UInput
						v-model="searchKeywordQuery"
						placeholder="Filter keywords..."
						icon="i-lucide-filter"
						size="xs"
						class="max-w-xs flex-1"
					/>

					<UButton
						size="xs"
						variant="link"
						color="neutral"
						@click="toggleSelectAllKeywords"
					>
						{{ isAllFilteredKeywordsSelected ? 'Deselect All' : 'Select All Filtered' }}
					</UButton>
				</div>

				<!-- Interactive Keywords Table -->
				<div class="overflow-x-auto max-h-[380px] overflow-y-auto rounded-lg border border-default">
					<table class="w-full text-left text-xs font-mono">
						<thead class="sticky top-0 bg-elevated border-b border-default text-muted z-10">
							<tr>
								<th class="py-2.5 px-3 w-10">
									<UCheckbox
										:model-value="isAllFilteredKeywordsSelected ? true : isSomeFilteredKeywordsSelected ? 'indeterminate' : false"
										@update:model-value="toggleSelectAllKeywords"
									/>
								</th>
								<th class="py-2.5 px-3 w-12 text-muted">#</th>
								<th class="py-2.5 px-3">Organic Search Term (Query)</th>
								<th class="py-2.5 px-3 w-20 text-right">Action</th>
							</tr>
						</thead>
						<tbody class="divide-y divide-default bg-default">
							<tr
								v-for="(kw, idx) in filteredKeywords"
								:key="kw"
								class="hover:bg-elevated/50 transition-colors"
								:class="selectedKeywords.includes(kw) ? 'bg-error/10' : ''"
							>
								<td class="py-2.5 px-3">
									<UCheckbox
										:model-value="selectedKeywords.includes(kw)"
										@update:model-value="toggleSelectOneKeyword(kw)"
									/>
								</td>
								<td class="py-2.5 px-3 text-muted">{{ idx + 1 }}</td>
								<td class="py-2.5 px-3 font-medium text-highlighted">
									"{{ kw }}"
								</td>
								<td class="py-2.5 px-3 text-right">
									<UButton
										size="xs"
										color="error"
										variant="ghost"
										icon="i-lucide-trash"
										@click="removeSingleKeyword(kw)"
									/>
								</td>
							</tr>
							<tr v-if="filteredKeywords.length === 0">
								<td colspan="4" class="py-8 text-center text-muted font-sans">
									{{ activeCampaign.keywordList.length === 0 ? 'No keywords added. Enter keywords above or click "Import File (.txt, .csv, .md)".' : 'No keywords match your filter query.' }}
								</td>
							</tr>
						</tbody>
					</table>
				</div>

				<!-- Footer Summary -->
				<div v-if="activeCampaign.keywordList.length > 0" class="flex items-center justify-between text-xs text-muted pt-2 border-t border-default">
					<span>
						Total: {{ activeCampaign.keywordList.length }} search terms in rotation pool
					</span>
					<span v-if="selectedKeywords.length > 0" class="text-error font-medium">
						{{ selectedKeywords.length }} keyword(s) selected for deletion
					</span>
				</div>
			</div>
		</UCard>

		<!-- Google Analytics (GA4) UTM Tags -->
		<UCard variant="subtle">
			<template #header>
				<div class="flex flex-col sm:flex-row sm:items-center justify-between gap-2">
					<div class="flex items-center gap-2">
						<UIcon name="i-lucide-tags" class="size-4 text-primary" />
						<h3 class="text-sm font-semibold text-highlighted">Google Analytics (GA4) UTM Attribution Tags</h3>
					</div>
					<UBadge size="xs" color="info" variant="subtle">
						Campaign Attribution
					</UBadge>
				</div>
			</template>

			<div class="space-y-4">
				<p class="text-xs text-muted leading-relaxed">
					UTM parameters attach tracking query strings to every simulated hit, enabling exact attribution in your <strong>Google Analytics 4 (GA4) Realtime & Traffic Acquisition</strong> dashboards under customized marketing campaigns and channels.
				</p>

				<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
					<UFormField
						label="utm_campaign"
						hint="Campaign or launch title"
						help="e.g. spring_launch, seo_boost"
					>
						<UInput v-model="activeCampaign.utmCampaign" placeholder="spring_launch" class="w-full" />
					</UFormField>
					<UFormField
						label="utm_medium"
						hint="Marketing channel / medium"
						help="e.g. organic, cpc, referral, social"
					>
						<UInput v-model="activeCampaign.utmMedium" placeholder="organic" class="w-full" />
					</UFormField>
					<UFormField
						label="utm_source"
						hint="Traffic source / provider"
						help="e.g. google, facebook, direct"
					>
						<UInput v-model="activeCampaign.utmSource" placeholder="google" class="w-full" />
					</UFormField>
				</div>

				<!-- Live Injected URL Preview -->
				<div class="p-3 rounded-lg bg-elevated/70 border border-default text-xs space-y-1 font-mono">
					<span class="text-[11px] text-muted block font-sans font-semibold">Injected GA4 Query String Preview:</span>
					<span class="text-primary truncate block text-xs">
						{{ (activeCampaign.targetUrls[0] || 'https://example.com/page') }}?utm_campaign={{ activeCampaign.utmCampaign || 'spring_launch' }}&utm_medium={{ activeCampaign.utmMedium || 'organic' }}&utm_source={{ activeCampaign.utmSource || 'google' }}
					</span>
				</div>
			</div>
		</UCard>

		<!-- Torpedo Simulation Strategy Preview -->
		<UCard variant="subtle">
			<div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
				<div class="flex items-center gap-3">
					<div class="p-2.5 rounded-xl bg-primary/10 border border-primary/20 text-primary">
						<UIcon name="i-lucide-gauge" class="size-6" />
					</div>
					<div>
						<div class="flex items-center gap-2">
							<h4 class="text-sm font-semibold text-highlighted">Simulation Engine Mode:</h4>
							<UBadge size="xs" color="primary" variant="subtle" class="font-mono">
								{{ (activeCampaign.trafficMode || 'default').toUpperCase() }}
							</UBadge>
							<UBadge size="xs" color="neutral" variant="subtle">
								{{ activeCampaign.specialMode === 'surfing' ? `Surfing (${activeCampaign.surfingCount || 2} Pages)` : 'Repetition' }}
							</UBadge>
						</div>
						<p class="text-xs text-muted mt-0.5">
							{{ activeCampaign.sessionDurationType === 'variable' ? `Variable duration (${activeCampaign.sessionDurationMin || 4000}-${activeCampaign.sessionDurationMax || 10000}ms)` : `Fixed duration (${activeCampaign.sessionDurationConstant || 5000}ms)` }}
							• Delay: {{ activeCampaign.trafficDelayType === 'variable' ? `${activeCampaign.trafficDelayMin || 2000}-${activeCampaign.trafficDelayMax || 5000}ms` : `${activeCampaign.trafficDelayConstant || 1500}ms` }}
							• Order: {{ activeCampaign.pageOrder === 'random' ? 'Random' : 'In-Order' }}
						</p>
					</div>
				</div>

				<UButton
					to="/browser"
					variant="subtle"
					color="primary"
					icon="i-lucide-sliders"
					size="xs"
				>
					Configure Engine
				</UButton>
			</div>
		</UCard>
	</div>
</template>
