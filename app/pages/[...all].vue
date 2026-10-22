<script setup lang="ts">
definePageMeta({
	layout: "blank"
});
</script>

<template>
	<div class="grid place-items-center py-16 md:py-32">
		<div class="flex flex-col items-center gap-y-4 md:gap-y-6 text-center max-w-md px-4">
			<div class="p-3 rounded-2xl bg-primary/10 text-primary border border-primary/20">
				<UIcon name="i-lucide-compass" class="size-8" />
			</div>
			<p class="text-primary font-semibold text-xl tracking-tight">
				404 • Page Not Found
			</p>
			<div class="space-y-2">
				<h1 class="text-2xl font-bold tracking-tight text-highlighted sm:text-3xl">
					Lost in Traffic?
				</h1>
				<p class="text-sm text-muted leading-relaxed">
					Sorry, the page or route you are attempting to navigate to does not exist in FlashTraffic.
				</p>
			</div>
			<UButton to="/" variant="subtle" size="sm" color="primary" icon="i-lucide-arrow-left">
				Back to Dashboard
			</UButton>
		</div>
	</div>
</template>
