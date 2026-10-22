<script setup lang="ts">
interface ReleaseAsset {
  name: string
  browser_download_url: string
  size: number
}

interface ReleaseData {
  tag_name: string
  name: string
  assets: ReleaseAsset[]
  html_url: string
}

const { data: release, status } = await useFetch<ReleaseData>('https://api.github.com/repos/narr07/flashtraffic/releases/latest', {
  server: false,
  lazy: true
})

const exeAsset = computed(() => release.value?.assets?.find(a => a.name.endsWith('.exe') && !a.name.includes('.sig')))
const msiAsset = computed(() => release.value?.assets?.find(a => a.name.endsWith('.msi') && !a.name.includes('.sig')))

const formatSize = (bytes: number) => {
  if (!bytes) return ''
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}
</script>

<template>
  <div class="my-6 rounded-2xl border border-default bg-elevated/50 p-6 backdrop-blur">
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
      <div class="space-y-1">
        <div class="flex items-center gap-2">
          <UIcon name="i-lucide-download" class="size-5 text-primary" />
          <h3 class="text-base font-bold text-highlighted">
            Download FlashTraffic for Windows
          </h3>
          <UBadge v-if="release?.tag_name" size="xs" color="primary" variant="subtle">
            {{ release.tag_name }}
          </UBadge>
        </div>
        <p class="text-xs text-muted">
          Official Windows 10/11 standalone desktop installer. Runs completely local and offline.
        </p>
      </div>

      <div class="flex flex-wrap items-center gap-2">
        <!-- Direct Exe Installer Download -->
        <UButton
          v-if="exeAsset"
          :to="exeAsset.browser_download_url"
          color="primary"
          icon="i-lucide-download"
          size="sm"
          external
        >
          Download .exe ({{ formatSize(exeAsset.size) }})
        </UButton>

        <!-- MSI Installer Download -->
        <UButton
          v-if="msiAsset"
          :to="msiAsset.browser_download_url"
          variant="subtle"
          color="neutral"
          icon="i-lucide-box"
          size="sm"
          external
        >
          Download .msi ({{ formatSize(msiAsset.size) }})
        </UButton>

        <!-- Fallback to Releases Page -->
        <UButton
          v-if="!exeAsset"
          to="https://github.com/narr07/flashtraffic/releases"
          color="primary"
          icon="i-lucide-github"
          size="sm"
          external
        >
          View Releases on GitHub
        </UButton>
      </div>
    </div>

    <!-- SmartScreen Note -->
    <div class="mt-4 pt-4 border-t border-default/60 flex items-start gap-2.5 text-xs text-muted">
      <UIcon name="i-lucide-shield-alert" class="size-4 text-warning shrink-0 mt-0.5" />
      <span>
        <strong>SmartScreen Notice:</strong> As FlashTraffic is newly published, Windows Defender SmartScreen may display an <em>"Unknown Publisher"</em> dialog on first run. Click <strong>"More info"</strong> &rarr; <strong>"Run anyway"</strong>. FlashTraffic is open source and built transparently via GitHub Actions.
      </span>
    </div>
  </div>
</template>
