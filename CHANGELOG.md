# Changelog

All notable changes to the FlashTraffic application will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-10-22

### Added
- **Core Architecture**: Transitioned to Nuxt 4, Nuxt UI v4, and Tauri 2 with async Tokio Rust runtime.
- **Playwright Local Automation**: Dual-engine integration supporting natural Bezier mouse physics, humanized scrolling, dwell time, and multi-page internal surfing.
- **BrowserStack Cloud Integration**: REST API connection to BrowserStack Automate grid for live traffic validation on real mobile and desktop devices.
- **Campaign Studio**: Multi-channel traffic routing (Direct, Organic search with keywords, Referral via social and custom URLs) and Google Analytics UTM builder.
- **Proxy & Geolocation Orchestrator**: Concurrent asynchronous proxy latency tester supporting HTTP, HTTPS, SOCKS4, and SOCKS5.
- **Link Crawler**: Built-in recursive internal link scanner for automatic landing page extraction.
- **Safety Watchdog**: Anti-orphan child process termination on window exit with no console flickering (`CREATE_NO_WINDOW`).
- **Modern UI**: Full Nuxt UI v4 dashboard layout with semantic tokens, real-time log terminal, and KPI metric counters.
