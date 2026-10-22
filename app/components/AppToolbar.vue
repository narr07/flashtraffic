<script setup lang="ts">
const mounted = ref(false);
onMounted(() => {
	mounted.value = true;
});
</script>

<template>
	<Teleport v-if="mounted" to="#layout-toolbar">
		<slot />
	</Teleport>
</template>
