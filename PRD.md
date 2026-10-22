# Product Requirements Document (PRD): FlashTraffic Desktop Application

> **Nama Produk:** FlashTraffic  
> **Kategori:** Modern Web Traffic Simulation & Analytics Load Testing Desktop Platform  
> **Arsitektur:** Nuxt 4 + Nuxt UI v4 + Tauri 2 (Rust) + Playwright Runner + BrowserStack Cloud Bridge  
> **Status Dokumen:** Draf Siap Review (Fase 2)  
> **Lokasi Kerja:** `E:\website\softwere\flashtraffic`  
> **Target OS:** Windows 10 / 11 (64-bit)

---

## 1. Product Overview & Problem Statement

### 1.1 Latar Belakang
Alat simulasi traffic generasi lama seperti TorPedo Traffic Generator Ultimate (WAT) mengandalkan teknologi desktop usang (WPF .NET 5 dan CefSharp/Chromium monolitik) yang rentan mengalami *freezing*, kebocoran memori, deteksi instan oleh bot protection modern (Cloudflare Turnstile, Datadome, Akamai), serta keterbatasan eksekusi hanya pada satu mesin lokal.

Dalam ekosistem web modern saat ini (Single Page Applications, Server-Side Rendering, analitik GA4, dan CDN global), QA Engineer, Web Developer, dan Digital Marketer membutuhkan generator traffic generasi baru yang:
1. Meniru interaksi manusia secara presisi (kurva mouse bezier, scroll bertingkat, delay acak, navigasi multi-halaman).
2. Memiliki fleksibilitas eksekusi hybrid: mesin lokal berkecepatan tinggi via **Playwright** dan mesin cloud perangkat riil via **BrowserStack**.
3. Memiliki antarmuka desktop modern, responsif, dan ringan tanpa membekukan sistem operasi.

### 1.2 Visi & Nilai Tambah Produk
**FlashTraffic** hadir sebagai platform desktop modern berkinerja tinggi yang menggabungkan keandalan backend Rust (Tauri 2), keanggunan antarmuka Nuxt UI v4, kecanggihan otomatisasi browser Playwright, dan kapabilitas cloud testing global dari BrowserStack.

---

## 2. User Personas & Use Cases

### 2.1 Persona Pengguna
1. **Andi – Lead QA Automation & Performance Engineer**:
   - *Tujuan*: Menguji ketahanan web server, CDN cache, dan performa arsitektur frontend di bawah beban ribuan simulasi kunjungan bertahap.
   - *Masalah*: Alat stress-test konvensional (JMeter/K6) tidak mengeksekusi JavaScript dan tag analitik di browser sebenarnya.
2. **Budi – Growth & SEO Analytics Specialist**:
   - *Tujuan*: Memvalidasi pelacakan atribusi analitik (Google Analytics 4, Meta Pixel, UTM Campaign) dari berbagai kanal (Direct, Organic Search, Referral, Social) dan lintas perangkat.
   - *Masalah*: Tool lama sering terdeteksi sebagai bot dan diblokir sebelum tag analitik sempat terpicu.
3. **Cici – Web Agency Technical Consultant**:
   - *Tujuan*: Menguji tampilan website klien dari berbagai perangkat mobile asli (iPhone, iPad, Samsung Galaxy) dan geolokasi global.
   - *Masalah*: Memerlukan integrasi perangkat fisik nyata di cloud tanpa perlu membeli ratusan ponsel fisik.

### 2.2 Skenario Penggunaan Utama (Core Use Cases)
- **Skenario A (SEO Attribution & Tag Verification)**: Menjalankan skenario Organic Traffic dengan keyword tertentu melalui mesin pencari (Google/Bing) untuk memverifikasi apakah event GA4 terpicu dengan benar.
- **Skenario B (Real-User Multi-Page Surfing)**: Mensimulasikan pengunjung mendarat di halaman beranda, menggulir layar, berhenti sejenak membaca, mengklik tautan internal, dan membaca halaman kedua guna menjaga metrik bounce rate realistis.
- **Skenario C (Global Cloud Verification via BrowserStack)**: Mengarahkan campaign untuk dieksekusi di cloud BrowserStack menggunakan perangkat fisik nyata di Amerika Serikat, Eropa, dan Asia.
- **Skenario D (Automated Proxy Stress Test)**: Mengimpor daftar 500 proxy, melakukan uji latensi asinkron otomatis dalam hitungan detik, dan merotasi IP bersih selama kampanye berjalan.

---

## 3. System Architecture & Tech Stack

### 3.1 Diagram Arsitektur Sistem

```
+---------------------------------------------------------------------------------------+
|                                FRONTEND: Nuxt 4 + Nuxt UI v4                          |
|  [UDashboardGroup]                                                                    |
|  - Dashboard Overview (UCard, UProgress, Live Charts)                                 |
|  - Campaign Manager (UTable, UForm, UModal)                                           |
|  - Proxy Orchestrator (UTable, UBadge, Latency Tester)                                |
|  - Playwright Engine Settings (Headless/Headed, Viewport, Stealth)                    |
|  - BrowserStack Cloud Console (Device Matrix, Live Stream Link)                       |
|  - State Management: Pinia Store | IPC Composable: useTrafficBackend()                |
+---------------------------------------------------------------------------------------+
                                           |
                           Tauri 2 IPC (Async Commands & Events)
                                           |
+---------------------------------------------------------------------------------------+
|                              BACKEND: Tauri 2 (Rust Core)                             |
|  - Task Scheduler & Runner (Tokio Async Runtime)                                      |
|  - Proxy Validation & Pool Manager (reqwest + async latency test)                     |
|  - Child Process Controller (Playwright Node Sidecar / Worker Process)                |
|  - BrowserStack REST API Client (Automate & Device Sessions)                         |
|  - Process Lifecycle & Safety Guard (Auto kill on exit, no orphan node processes)   |
|  - System Tray & Window Manager (Minimize to tray, background execution)              |
+---------------------------------------------------------------------------------------+
                |                                                       |
                v                                                       v
+-----------------------------------------+   +-----------------------------------------+
|     LOCAL RUNNER: Playwright Sidecar    |   |     CLOUD RUNNER: BrowserStack Cloud    |
| - Chromium, Firefox, WebKit             |   | - https://live.browserstack.com/        |
| - Stealth Plugin (Anti-Detection)       |   | - BrowserStack Automate API (Hub URL)   |
| - Human Physics (Bezier, Scroll Jitter) |   | - Real iOS, Android, macOS, Win Devices|
| - Proxy Injection per BrowserContext    |   | - 100+ Geo-IP Network Locations         |
+-----------------------------------------+   +-----------------------------------------+
                                           |
                                           v
                              [TARGET WEB INFRASTRUCTURE]
```

### 3.2 Spesifikasi Teknologi
| Lapisan | Pilihan Teknologi | Versi & Alasan Pemilihan |
| :--- | :--- | :--- |
| **Frontend Framework** | Nuxt 4 (Vue 3, TypeScript) | Performa SSR/SSG tinggi, struktur file rapi, composable reaktif. |
| **Design System** | Nuxt UI v4 (`@nuxt/ui`) | 125+ komponen aksesibel siap pakai, styling Tailwind v4, token semantik elegan. |
| **State Management** | Pinia | Type-safe, manajemen state reaktif terpusat untuk campaign, proxy, dan metrik. |
| **Desktop Runtime** | Tauri 2 (Rust) | Footprint memori sangat kecil (~15 MB), aman, kompilasi biner native Windows. |
| **Local Automation** | Playwright (Node.js engine) | Standar industri otomatisasi browser modern, mendukung CDP, simulasi perangkat mobile sempurna. |
| **Cloud Automation** | BrowserStack Automate REST API | Akses ke ribuan kombinasi perangkat nyata di cloud tanpa beban komputasi lokal. |
| **Package Manager** | Bun | Instalasi cepat, kompatibel dengan ekosistem script project pengguna. |

---

## 4. Spesifikasi Fitur Detail (Functional Requirements)

### 4.1 Modul 1: Campaign Manager (Pengelola Kampanye)
1. **Multi-Traffic Types**:
   - **Direct Traffic**: Kunjungan langsung tanpa header `Referer`.
   - **Organic Search Traffic**: Simulasi pencarian via mesin pencari (Google, Bing, Yahoo, Yandex, DuckDuckGo). Otomatis menyuntikkan query kata kunci ke URL referral (misal `https://www.google.com/search?q={keyword}`).
   - **Referral Traffic**: Simulasi rujukan dari media sosial (Facebook, Twitter/X, Instagram, LinkedIn, Reddit, YouTube, TikTok) atau custom referrer URL pilihan pengguna.
2. **Target URL Management**:
   - Menambahkan banyak URL per kampanye dengan pembobotan bobot kunjungan (weighting).
   - Opsi distribusi: *Round Robin (In Order)* atau *Randomly Weighted*.
3. **Penyimpanan Kampanye**:
   - Menyimpan dan memuat konfigurasi kampanye ke berkas JSON terenkripsi/terbaca lokal (`.ftraf` / `.json`).

### 4.2 Modul 2: UTM Campaign Builder (Google Analytics Integration)
1. Pembuat parameter pelacakan bawaan untuk GA4 dan platform analitik:
   - `utm_source`: Menentukan asal traffic (e.g. `google`, `newsletter`, `facebook`).
   - `utm_medium`: Menentukan jenis media (e.g. `organic`, `cpc`, `email`, `social`).
   - `utm_campaign`: Nama kampanye spesifik.
   - `utm_content`: Variasi konten/iklan untuk A/B testing.
   - `utm_term`: Kata kunci pencarian atau audiens spesifik.
2. Generator otomatis: Menggabungkan URL target dengan string query parameter yang valid secara instan.

### 4.3 Modul 3: Playwright Local Automation Engine (Simulasi Manusia)
1. **Multi-Browser Engine**: Pilihan browser lokal: Chromium, Firefox, WebKit.
2. **Headless & Headed Toggle**: Pengguna dapat memilih mode tersembunyi (headless) untuk efisiensi RAM/CPU atau mode jendela terlihat (headed) untuk memantau langsung.
3. **Humanized Interaction Physics**:
   - **Natural Mouse Movement**: Kursor bergerak menggunakan kurva Bezier dengan variasi kecepatan dan sedikit goyangan (*jitter*) alami.
   - **Smooth Human Scrolling**: Menggulir halaman ke bawah secara bertahap dengan kecepatan variabel, diselingi jeda baca (*reading pauses*) sebelum menggulir kembali.
   - **Multi-Page Surfing**: Pengunjung memilih tautan internal yang valid di dalam domain yang sama, mengkliknya, dan membaca halaman lanjutan dalam sesi yang sama (menurunkan bounce rate secara realistis).
   - **Random Text Selection & Hover**: Simulasi seleksi teks singkat dan hover pada elemen navigasi atau kartu konten.
4. **Stealth Antidetection**:
   - Menghapus flag `navigator.webdriver = true`.
   - Menyuntikkan plugins, languages, hardwareConcurrency, dan audio/canvas fingerprint noise realistis.

### 4.4 Modul 4: Integrasi Cloud BrowserStack (`https://live.browserstack.com/dashboard`)
1. **Manajemen Kredensial**:
   - Input aman untuk BrowserStack Username dan Access Key.
   - Penyimpanan kredensial terenkripsi di OS credential manager atau local secure store.
2. **Device & Browser Matrix**:
   - Memilih kombinasi OS & Browser dari BrowserStack Cloud:
     - Real Devices: iPhone 15/16 Pro, Samsung Galaxy S24, Google Pixel, iPad Pro.
     - Desktop Browsers: Windows 11 (Chrome/Edge/Firefox), macOS Sonoma (Safari/Chrome).
3. **Cloud Execution & Monitoring**:
   - Mengarahkan sesi kampanye ke BrowserStack Remote WebDriver/CDP URL (`https://hub-cloud.browserstack.com/wd/hub`).
   - Menyediakan tautan langsung ke sesi dashboard BrowserStack (`https://live.browserstack.com/dashboard`) untuk melihat rekaman video sesi, log konsol, dan screenshot jaringan secara langsung.

### 4.5 Modul 5: Proxy & Network Orchestrator
1. **Format Input**:
   - Mendukung format: `IP:Port`, `IP:Port:Username:Password`, dan `Host:Port`.
   - Impor massal dari file `.txt`, clipboard, atau URL proxy subscription.
2. **Protokol Jaringan**:
   - HTTP, HTTPS, SOCKS4, SOCKS5.
3. **Pemeriksaan Latensi & Kualitas Asinkron (Rust Async Pool)**:
   - Tes ping dan TLS handshake simultan (hingga 100 proxy paralel dalam hitungan detik).
   - Indikator kecepatan visual:
     - Hijau (Cepat: < 800ms)
     - Kuning (Sedang: 800ms - 2500ms)
     - Merah (Lambat: > 2500ms)
     - Abu-abu / coret (Mati / Timeout)
   - Auto-reject proxy mati atau lambat.
4. **Rotasi Fleksibel**:
   - Rotasi setiap sesi baru (Per-Request).
   - Rotasi berdasarkan interval waktu acak (Min - Max detik).
   - Rotasi acak dari kumpulan proxy sehat (Health Pool).

### 4.6 Modul 6: Fingerprint & Device Identity Profiles
1. **Katalog Profil Perangkat Terkini**:
   - Kumpulan User-Agent modern (Windows 11 Chrome 128+, macOS Safari 17+, iOS 18 Safari, Android 14 Chrome).
   - Resolusi viewport realistis (e.g. 1920×1080, 1440×900, 390×844, 412×915).
2. **Spoofing Metadata Lengkap**:
   - HTTP Client Hints (`Sec-Ch-Ua`, `Sec-Ch-Ua-Mobile`, `Sec-Ch-Ua-Platform`).
   - Geolocation coordinates (Latitude/Longitude) yang sinkron dengan lokasi proxy IP.
   - Sinkronisasi Timezone browser dengan zona waktu proxy.

### 4.7 Modul 7: Live Analytics Dashboard & Diagnostics
1. **Live Metrics**:
   - Total Kunjungan Berhasil, Total Gagal/Timeout.
   - Pengunjung Aktif Saat Ini (Concurrent Active Visitors).
   - Rata-rata Durasi Sesi (Average Dwell Time).
   - Persentase Bounce Rate Aktual.
2. **Real-Time Log Stream**:
   - Terminal log berwarna di UI (`[INFO]`, `[SUCCESS]`, `[WARN]`, `[ERROR]`).
   - Opsi auto-scroll dan pembersihan log otomatis setiap jam.
3. **Visual Live Canvas / Snapshot Preview**:
   - Menampilkan screenshot halaman terkini yang sedang dikunjungi browser untuk memverifikasi tampilan secara visual.

### 4.8 Modul 8: Scheduler & Background Execution
1. Penjadwalan kampanye berbasis waktu (Mulai sekarang, Jadwal tertentu, Berulang harian/mingguan).
2. Mode Minimize ke System Tray Windows (`tauri-plugin-tray`) agar tidak mengganggu aktivitas desktop pengguna.
3. Auto-shutdown / Auto-exit timer jika kuota traffic tercapai.

### 4.9 Modul 9: Link Fetcher (Crawler Internal)
1. Memasukkan URL domain website.
2. Engine Rust crawler mengekstrak seluruh tautan internal (`<a> href`) dalam domain yang sama hingga kedalaman (*depth*) tertentu.
3. Tombol sekali klik untuk langsung memindahkan hasil tautan ke daftar URL kampanye.

---

## 5. Rancangan Antarmuka & Pemetaan Komponen Nuxt UI

Mengikuti standar ketat skill `nuxtor-desktop-app`, seluruh antarmuka 100% menggunakan komponen Nuxt UI v4:

| Halaman / View | Struktur Layout Nuxt UI | Komponen Nuxt UI yang Digunakan |
| :--- | :--- | :--- |
| **Root Shell** | `UDashboardGroup` | `UDashboardSidebar`, `UDashboardPanel`, `UDashboardNavbar`, `UDashboardToolbar` |
| **Overview Dashboard** | Grid panel metrik & status | `UCard` (ringkasan metrik), `UProgress` (progres kuota), `UBadge` (status engine), `UButton` (Start/Pause/Stop) |
| **Campaign Studio** | Multi-step form & table | `UTabs`, `UTable` (daftar URL/bobot), `UForm`, `UFormField`, `UInput`, `USelect`, `USwitch`, `USlider` |
| **Proxy Manager** | Data table dengan bulk actions | `UTable` (kolom IP, port, latency, status), `UBadge` (warna latensi), `UModal` (dialog import massal), `UButton` |
| **Playwright Engine** | Form konfigurasi browser | `UFormField`, `USelect` (Chromium/Firefox/WebKit), `USwitch` (Headed mode), `USlider` (Dwell time min/max) |
| **BrowserStack Cloud** | Kredensial & device selector | `UInput` (API keys dengan mode password mask), `USelect` (Device/OS Matrix), `UAlert` (status koneksi cloud) |
| **Link Fetcher** | Crawler tool modal / page | `UInput` (Domain URL), `UProgress` (proses crawling), `UTable` (hasil tautan), `UButton` (Add to Campaign) |
| **Live Logs & Visual** | Log stream panel & snapshot preview | `UCard`, `UBadge` (level log), `UButton` (Clear/Export), pembungkus gambar snapshot dengan border semantik |
| **Confirm & Delete** | Dialog keamanan aksi | `UModal` dengan slot `#body` dan `#footer`, `UButton` variant `error` khusus aksi destruktif |

### Token Desain & Tema (`app.config.ts`):
- Warna `primary`: Cyan / Emerald (modern, tech-savvy).
- Warna `neutral`: Slate / Zinc.
- Warna semantik: `text-muted`, `text-highlighted`, `bg-elevated`, `border-default`.
- Seluruh angka metrik dan data tabel wajib jujur dan bersumber langsung dari backend Rust.

---

## 6. Tauri IPC & Rust Backend Commands Specification

Seluruh komunikasi frontend-backend dipusatkan pada composable `useTrafficBackend.ts` yang memanggil Rust command asinkron:

```rust
// Modul Rust: src-tauri/src/commands.rs

#[tauri::command(async)]
pub async fn start_campaign(campaign: CampaignConfig) -> Result<CampaignSessionId, String>;

#[tauri::command(async)]
pub async fn stop_campaign(session_id: CampaignSessionId) -> Result<(), String>;

#[tauri::command(async)]
pub async fn pause_campaign(session_id: CampaignSessionId) -> Result<(), String>;

#[tauri::command(async)]
pub async fn test_proxy_pool(proxies: Vec<ProxyItem>) -> Result<Vec<ProxyTestResult>, String>;

#[tauri::command(async)]
pub async fn fetch_sitemap_urls(target_domain: String, max_depth: u32) -> Result<Vec<String>, String>;

#[tauri::command(async)]
pub async fn test_browserstack_credentials(username: String, access_key: String) -> Result<BrowserStackAuthStatus, String>;

#[tauri::command(async)]
pub async fn get_system_metrics() -> Result<LiveSystemMetrics, String>;
```

### Event Streaming (Tauri Event Window Emit):
- `traffic://log-stream`: Mengirim payload log live ke frontend (`timestamp`, `level`, `message`).
- `traffic://progress`: Mengirim update kuota traffic dan counter pengunjung aktif.
- `traffic://snapshot`: Mengirim buffer base64 screenshot terkini untuk visual preview.

---

## 7. Keamanan, Stabilitas & Rust Safety Guidelines

1. **Pembersihan Proses Yatim (Orphan Process Prevention)**:
   - Backend Rust melacak seluruh Process ID (PID) browser Playwright yang dibuka.
   - Menerapkan hook shutdown window dan panic handler untuk mengirim sinyal terminasi (`kill`) ke semua proses child browser jika aplikasi ditutup.
2. **Kerahasiaan Kredensial**:
   - Access Key BrowserStack dan kredensial proxy tidak pernah dicetak dalam log visual atau diekspor dalam file laporan publik.
3. **Isolasi Jaringan**:
   - Tidak mengubah setelan proxy WinInet sistem operasi Windows secara global. Seluruh proxy diinjeksi pada level konteks browser Playwright terisolasi (`BrowserContextOptions.proxy`).
4. **Pembatasan Resource (Throttling)**:
   - Limitasi konkurensi tab browser lokal (default maksimal 5 tab simultan pada PC standar) guna mencegah lonjakan pemakaian RAM dan CPU 100%.

---

## 8. Non-Functional Requirements (Kualitas Sistem)

1. **Performa**:
   - Ukuran installer desktop < 25 MB (dibandingkan TorPedo lama > 220 MB).
   - Waktu startup aplikasi desktop < 1.5 detik.
   - Pengecekan 100 proxy selesai dalam < 3 detik (via async Tokio network pool).
2. **Kestabilan & Responsivitas**:
   - UI tidak boleh mengalami freezing (0% blocking operations di main thread).
3. **Kompatibilitas**:
   - Windows 10 (Build 1903+) dan Windows 11 (64-bit).
   - Dukungan offline penuh untuk UI dan pemuatan ikon (`icon: { provider: 'none' }`).

---

## 9. Roadmap Pengembangan & Rilis

- **Milestone 1 (Fondasi & Setup Proyek)**: Setup template Nuxtor, penggantian identitas template, struktur shell UI Nuxt UI v4.
- **Milestone 2 (Backend Rust & Proxy Pool)**: Implementasi async proxy tester, parser, dan scheduler di Rust.
- **Milestone 3 (Playwright Automation Sidecar)**: Engine otomatisasi lokal dengan simulasi kurva mouse manusia dan multi-page surfing.
- **Milestone 4 (Integrasi Cloud BrowserStack)**: Hubungan REST API BrowserStack, device selector, dan live monitor bridge.
- **Milestone 5 (Testing & Quality Gates)**: Linting, typecheck, clippy, unit test, dan pembuatan installer rilis lokal (`.exe` & `.msi`).
- **Milestone 6 (Dokumentasi & Rilis Publik)**: Setup situs dokumentasi Nuxt UI Docs di `docs/` dengan carousel screenshot dan GitHub Actions workflow.

---

## 10. Persetujuan & Checkpoint

Dokumen ini merupakan acuan mutlak spesifikasi produk untuk pembangunan aplikasi FlashTraffic pada **Fase 3**.
