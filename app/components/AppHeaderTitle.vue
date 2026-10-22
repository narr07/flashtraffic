<script setup lang="ts">
const mounted = ref(false);

onMounted(() => {
	const el = document.getElementById("layout-title");
	if (el) el.innerHTML = "";
	mounted.value = true;
});

onUnmounted(() => {
	const el = document.getElementById("layout-title");
	if (el) el.innerHTML = "";
});
</script>

<template>
	<Teleport v-if="mounted" to="#layout-title">
		<slot />
	</Teleport>
</template>
