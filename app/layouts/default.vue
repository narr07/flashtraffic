<script setup lang="ts">
import { useTrafficState } from "~/composables/useTrafficState";

const { isRunning, liveMetrics } = useTrafficState();

const navItems = [
	{
		label: "Dashboard",
		icon: "i-lucide-layout-dashboard",
		to: "/"
	},
	{
		label: "Campaign Studio",
		icon: "i-lucide-megaphone",
		to: "/campaigns"
	},
	{
		label: "Playwright Engine",
		icon: "i-lucide-bot",
		to: "/browser"
	},
	{
		label: "BrowserStack Cloud",
		icon: "i-lucide-cloud",
		to: "/cloud"
	},
	{
		label: "Proxy Orchestrator",
		icon: "i-lucide-shield-check",
		to: "/proxies"
	},
	{
		label: "Link Crawler",
		icon: "i-lucide-link-2",
		to: "/fetcher"
	},
	{
		label: "Settings",
		icon: "i-lucide-settings",
		to: "/settings"
	}
];
</script>

<template>
	<div class="flex h-screen w-screen overflow-hidden bg-default text-default font-sans">
		<!-- Sidebar Navigation -->
		<aside class="w-64 border-r border-default bg-elevated/40 backdrop-blur-md flex flex-col justify-between p-4 shrink-0">
			<div>
				<!-- Logo & Header -->
				<div class="flex items-center gap-3 px-2 py-3 mb-6 border-b border-default">
					<img
						src="/icon.png"
						alt="FlashTraffic Logo"
						class="h-9 w-9 rounded-xl object-contain shadow-sm ring-1 ring-default"
					>
					<div>
						<h1 class="text-base font-bold tracking-tight text-highlighted flex items-center gap-1.5">
							FlashTraffic
							<UBadge size="xs" variant="subtle" color="primary">v0.1</UBadge>
						</h1>
						<p class="text-[11px] text-muted">Traffic & Load Simulator</p>
					</div>
				</div>

				<!-- Navigation Links -->
				<nav class="space-y-1">
					<NuxtLink
						v-for="item in navItems"
						:key="item.to"
						:to="item.to"
						class="flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-colors"
						:class="[
							$route.path === item.to
								? 'bg-primary/10 text-primary ring-1 ring-primary/20'
								: 'text-muted hover:text-highlighted hover:bg-elevated/80',
						]"
					>
						<UIcon :name="item.icon" class="h-4 w-4 shrink-0" />
						<span>{{ item.label }}</span>
					</NuxtLink>
				</nav>
			</div>

			<!-- Engine Quick Status in Sidebar -->
			<div class="p-3 rounded-xl bg-default/80 border border-default space-y-2">
				<div class="flex items-center justify-between text-xs">
					<span class="text-muted">Status</span>
					<UBadge
						size="xs"
						:color="isRunning ? 'primary' : 'neutral'"
						variant="subtle"
					>
						{{ isRunning ? "Running" : "Idle" }}
					</UBadge>
				</div>
				<div class="flex items-center justify-between text-xs font-mono">
					<span class="text-muted">Hits</span>
					<span class="text-highlighted font-semibold">{{ liveMetrics.successfulVisits }} / {{ liveMetrics.totalVisits }}</span>
				</div>
				<UProgress
					v-if="isRunning"
					:value="liveMetrics.totalVisits ? (liveMetrics.successfulVisits / liveMetrics.totalVisits) * 100 : 0"
					size="xs"
					color="primary"
				/>
			</div>
		</aside>

		<!-- Main Content Area -->
		<div class="flex-1 flex flex-col min-w-0 overflow-hidden">
			<!-- Top Navbar (Unified 2-row Header) -->
			<header class="border-b border-default bg-elevated/40 backdrop-blur-sm shrink-0">
				<!-- Row 1: Navbar with Page Title & Actions -->
				<div class="h-14 px-6 flex items-center justify-between">
					<div class="flex items-center gap-3 min-w-0">
						<div id="layout-title" class="flex items-center gap-2 text-sm font-semibold text-highlighted">
							<span class="text-muted">Platform</span>
							<span class="text-muted/60">/</span>
							<span class="capitalize">{{ $route.path === '/' ? 'Dashboard' : $route.path.replace('/', '') }}</span>
						</div>
					</div>

					<div class="flex items-center gap-3">
						<div id="layout-actions" class="flex items-center gap-2" />

						<UBadge
							variant="subtle"
							size="sm"
							color="neutral"
							class="flex items-center gap-1.5"
						>
							<span class="h-2 w-2 rounded-full" :class="isRunning ? 'bg-primary animate-pulse' : 'bg-muted'" />
							{{ isRunning ? 'Engine Active' : 'Engine Ready' }}
						</UBadge>
					</div>
				</div>

				<!-- Row 2: Sub-Navbar / Page Toolbar (Docked edge-to-edge right below Row 1) -->
				<div
					id="layout-toolbar"
					class="empty:hidden border-t border-default/70 bg-elevated/20 px-6 py-2.5 flex flex-wrap items-center justify-between gap-3 text-xs"
				/>
			</header>

			<!-- Page Body Viewport -->
			<main class="flex-1 overflow-y-auto p-6 bg-default">
				<slot />
			</main>
		</div>
	</div>
</template>
