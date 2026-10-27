# FlashTraffic

FlashTraffic is a modern, high-performance desktop web traffic simulation and analytics load testing platform built on **Nuxt 4**, **Nuxt UI v4**, and **Tauri 2 (Rust)**.

Equipped with a versatile multi-engine architecture, FlashTraffic lets you simulate human-like website traffic using the user's existing desktop browsers (Edge, Chrome, Brave, or custom paths) with visual window support, or an ultra-lightweight native Rust HTTP engine requiring zero external dependencies and minimal memory.

---

## Key Features

- **Multi-Channel Attribution**: Simulate Direct visits, Organic Search traffic (with keyword query injection for Google, Bing, Yahoo, DuckDuckGo), and Referral traffic (social media or custom referral URLs).
- **Dual Execution Engines**:
  - **System Browser Borrowing (Zero External Download)**:
    - Automatically discovers installed Windows browsers (Microsoft Edge, Google Chrome, Brave, Firefox) or supports custom `.exe` paths.
    - Zero need for end-users to install Node.js or download hundreds of megabytes of Playwright browser packages.
    - **Visual Window Mode**: When Headless is turned off, the real desktop browser window opens visually on screen to demonstrate live browsing.
    - **Headless Mode**: Runs silently in the background without UI interruption.
  - **Ultra-Lightweight Native Rust Engine**:
    - Direct asynchronous HTTP engine built into the FlashTraffic binary.
    - 0 MB external dependencies, ultra-low memory footprint (< 30 MB RAM), blazing-fast multi-hit load generation.
- **Built-in UTM Campaign Studio**: Generate and test Google Analytics 4 (GA4) attribution tags (`utm_campaign`, `utm_medium`, `utm_source`, `utm_term`, `utm_content`).
- **Humanized Simulation Physics**:
  - Randomized cubic Bézier mouse paths and natural micro-jitter.
  - Gradual scrolling with variable reading pauses.
  - Multi-page internal domain surfing to simulate authentic session depths and reduce bounce rates.
  - Antidetection stealth: masked `navigator.webdriver`, modern Client Hints, and native desktop user-agent rotation.
- **Smart Sitemap & URL Crawler**:
  - Deep XML sitemap parsing with automatic deduplication of localized routes and homepage links.
  - Filter-aware URL selection: accurately mirrors filtered view counts and protects against importing unselected or hidden pages.
- **Proxy & Geolocation Orchestrator**:
  - Supports HTTP, HTTPS, SOCKS4, and SOCKS5 proxies.
  - Concurrent async latency pings and auto-filtering of dead proxies.
  - Isolated browser context routing with zero modification to operating system global network settings.
- **Cloud Testing (BrowserStack)**:
  - Connect with BrowserStack Automate API to dispatch sessions across real iOS, Android, macOS, and Windows hardware devices.
- **Live Diagnostics & Terminal**: Real-time counter of concurrent visitors, success rate ratios, and interactive event logging.
- **Enterprise Installation**:
  - Windows per-machine installation to `C:\Program Files\FlashTraffic`.
  - Branded installer packages available in NSIS (`.exe`) and WiX (`.msi`).

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
   ├── Engine 1: System Browser Borrowing (Edge / Chrome / Brave / Custom .exe)
   │             └── Headless / Visual Window (--new-window)
   ├── Engine 2: Ultra-Lightweight Native Rust HTTP Engine (reqwest)
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

# Build desktop release executable (NSIS & MSI)
bun run desktop:build
```

---

## Windows SmartScreen Notice

FlashTraffic binaries built from source or downloaded from public GitHub Actions releases are open-source. Because they are not signed with a paid Extended Validation (EV) certificate, Windows SmartScreen may present a warning upon first launch. Click **More info** and select **Run anyway** to start the application.

---

## License

This project is licensed under the [MIT License](LICENSE).
