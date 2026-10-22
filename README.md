# FlashTraffic

FlashTraffic is a modern, high-performance desktop web traffic simulation and analytics load testing platform built on **Nuxt 4**, **Nuxt UI v4**, and **Tauri 2 (Rust)**.

Equipped with a dual-engine architecture, FlashTraffic bridges **Playwright** for natural, humanized local browser simulation (Bezier mouse physics, smooth scrolling, dwell time, and multi-page internal surfing) and **BrowserStack** for cloud-based traffic testing across real mobile and desktop devices globally.

---

## Key Features

- **Multi-Channel Attribution**: Simulate Direct visits, Organic Search traffic (with keyword query injection for Google, Bing, Yahoo, DuckDuckGo), and Referral traffic (social media or custom referral URLs).
- **Built-in UTM Campaign Studio**: Generate and test Google Analytics 4 (GA4) attribution tags (`utm_campaign`, `utm_medium`, `utm_source`, `utm_term`, `utm_content`).
- **Playwright Local Automation**:
  - Headless and Headed mode toggling.
  - Realistic Bezier mouse trajectories and natural reading pauses.
  - Automated internal link crawling and multi-page surfing to achieve targeted bounce rates.
  - Antidetection stealth: masked `navigator.webdriver`, modern Client Hints, and touch event emulation.
- **BrowserStack Cloud Integration**:
  - Connect with BrowserStack Automate API (`https://live.browserstack.com/dashboard`).
  - Dispatch traffic sessions to real iOS, Android, macOS, and Windows hardware devices.
- **Proxy & Geolocation Orchestrator**:
  - Supports HTTP, HTTPS, SOCKS4, and SOCKS5 proxies.
  - Concurrent async latency pings and auto-filtering of dead proxies.
  - Isolated browser context routing with zero modification to operating system global network settings.
- **Live Diagnostics & Terminal**: Real-time counter of concurrent visitors, success rate ratios, and interactive event logging.
- **Internal Link Crawler**: Built-in website scanner to discover and queue internal landing pages in a single click.

---

## Architecture Overview

```
Frontend: Nuxt 4 + Nuxt UI v4 + Tailwind CSS
   │
   ├── Composable: useTrafficBackend()
   │
IPC Bridge: Tauri 2 Async Invoke
   │
Backend Core: Rust Tokio Async Runtime (src-tauri/)
   │
   ├── Local Runner: Playwright Automation Sidecar (Chromium/Firefox/WebKit)
   └── Cloud Bridge: BrowserStack Automate REST API
```

---

## Getting Started

### Prerequisites
- [Bun](https://bun.sh/) (v1.2+)
- [Rust & Cargo](https://rustup.rs/) (v1.80+)
- [Node.js](https://nodejs.org/) (v20+)

### Installation & Development

```bash
# Clone the repository
git clone https://github.com/narr07/flashtraffic.git
cd flashtraffic

# Install frontend dependencies
bun install

# Run frontend development server
bun run dev

# Run desktop app in development mode
bun run desktop:dev
```

### Verification & Testing

```bash
# Lint frontend code
bun run lint

# TypeScript verification
bun run typecheck

# Rust unit tests
bun run test:rust

# Build static production bundle
bun run generate

# Build desktop release executable
bun run desktop:build
```

---

## Windows SmartScreen Notice

FlashTraffic binaries built from source or downloaded from public GitHub Actions releases are open-source. Because they are not signed with a paid Extended Validation (EV) certificate, Windows SmartScreen may present a warning upon first launch. Click **More info** and select **Run anyway** to start the application.

---

## License

This project is licensed under the [MIT License](LICENSE).
