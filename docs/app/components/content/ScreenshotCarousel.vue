<script setup lang="ts">
const config = useRuntimeConfig()
const base = config.app.baseURL || '/'

interface Slide {
  src: string
  title: string
  description: string
}

const slides: Slide[] = [
  {
    src: `${base}screenshots/flashtraffic_dashboard_verified.png`,
    title: 'Real-time Simulation Dashboard',
    description: 'Live hits telemetry, concurrent session counter, success rate progress, and live torpedo console.'
  },
  {
    src: `${base}screenshots/flashtraffic_campaigns_2row.png`,
    title: 'Campaign Studio & Matrix Distribution',
    description: 'Configure target URLs, attribution channels, multi-engine keyword search rotation, and GA4 UTM tags.'
  },
  {
    src: `${base}screenshots/flashtraffic_browser_2row_navbar.png`,
    title: 'Playwright Browser Engine',
    description: 'Natural human cubic Bézier curves, randomized dwell times, headless/headed toggles, and multi-page surfing.'
  },
  {
    src: `${base}screenshots/flashtraffic_proxies_2row.png`,
    title: 'Proxy Orchestrator & Live Verification',
    description: 'Auto-extract proxy nodes from URLs/text, measure latency, prune dead endpoints, and rotate global origins.'
  },
  {
    src: `${base}screenshots/flashtraffic_cloud_polished.png`,
    title: 'BrowserStack Cloud Device Matrix',
    description: 'Dispatch real human sessions to physical iPhones, Android devices, and global desktop operating systems.'
  },
  {
    src: `${base}screenshots/flashtraffic_fetcher_polished.png`,
    title: 'Deep Internal Link Crawler',
    description: 'Scan domains and sitemaps to discover internal URLs and import them directly into target campaign pools.'
  },
  {
    src: `${base}screenshots/flashtraffic_settings_polished.png`,
    title: 'Portable Config & Background Engine',
    description: 'Export and restore configuration across laptops via portable JSON with background system tray support.'
  }
]
</script>

<template>
  <div class="my-8 overflow-hidden rounded-2xl border border-default bg-elevated/40 p-3 shadow-lg">
    <UCarousel
      :items="slides"
      loop
      arrows
      dots
      :autoplay="{ delay: 4500, stopOnMouseEnter: true, stopOnInteraction: false }"
      class="w-full"
    >
      <template #default="{ item }">
        <figure class="w-full flex flex-col items-center">
          <img
            :src="item.src"
            :alt="item.title"
            class="w-full rounded-xl object-cover border border-default shadow-sm"
          >
          <figcaption class="mt-3 text-center px-4">
            <h4 class="text-sm font-semibold text-highlighted">{{ item.title }}</h4>
            <p class="text-xs text-muted mt-0.5">{{ item.description }}</p>
          </figcaption>
        </figure>
      </template>
    </UCarousel>
  </div>
</template>
