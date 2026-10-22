# Instruksi: TorPedo Traffic Generator Ultimate + PRD Aplikasi Desktop Modern (FlashTraffic)

> Panduan komprehensif untuk agen AI dalam meneliti target aplikasi TorPedo Traffic Generator Ultimate, menyusun PRD mendalam, membangun aplikasi desktop modern berbasis template **Nuxtor** (Nuxt 4 + Nuxt UI + Tauri 2 + Rust), mengintegrasikan **Playwright** dan **BrowserStack**, serta menyiapkan situs dokumentasi resmi.
> **PENTING:** Kerjakan fase demi fase dan **BERHENTI di setiap checkpoint** untuk menunggu persetujuan dan arahan dari pengguna.

---

## 0. Tujuan

Membangun aplikasi desktop modern traffic generator generasi baru bernama **FlashTraffic** (atau nama yang disepakati pengguna), dengan referensi analisis fitur dari aplikasi yang terpasang di:
`C:\Program Files (x86)\TorPedo Traffic Generator Ultimate`

Seluruh proses dikerjakan di folder kerja:
`E:\website\softwere\flashtraffic`

### Deliverables Utama
1. `research/feature-inventory.md`: Inventarisasi fitur TorPedo Traffic Generator Ultimate berbasis bukti nyata (Fase 1).
2. `PRD.md`: Dokumen kebutuhan produk aplikasi baru secara lengkap, detail, dan terstruktur (Fase 2).
3. Aplikasi desktop modern berbasis Nuxtor + Tauri 2 + Rust + Playwright + BrowserStack (Fase 3).
4. Situs dokumentasi + unduhan rilis publik di GitHub Pages menggunakan template Nuxt UI Docs dengan carousel screenshot otomatis (Fase 4).

---

## 1. Keunggulan & Diferensiasi Utama

Aplikasi baru tidak sekadar mengkloning fungsi lama, melainkan memodernisasi arsitektur secara drastis:
- **Engine Modern Playwright**: Menggantikan CefSharp WPF lama dengan engine browser automation modern (Playwright: Chromium, Firefox, WebKit) untuk simulasi perilaku interaksi manusia yang realistis (scroll dinamis, klik acak, mouse trajectory, delay acak, penanganan SPA/SSR).
- **Integrasi Cloud BrowserStack**: Terintegrasi langsung dengan `https://live.browserstack.com/dashboard` dan API BrowserStack Automate untuk menjalankan traffic campaign dari puluhan perangkat nyata (real mobile/desktop devices) dan lokasi IP geografis global di cloud.
- **Proxy Management Canggih**: Dukungan HTTP, HTTPS, SOCKS4, SOCKS5 dengan auto-rotation, latency tester, IP leak guard, dan deteksi ISP/Geo IP.
- **Fingerprint & Identity Spoofing Mutakhir**: Rotasi User-Agent modern, platform, canvas/WebGL noise, resolusi viewport realistis, locale, dan HTTP headers (sec-ch-ua, referrer spoofing Google/Bing/Social Media).
- **Dashboard Desktop Responsif**: Antarmuka kelas satu dengan Nuxt UI v4 dan Tauri 2, tanpa pembekuan antarmuka (100% async Rust backend).

---

## 2. Batasan Wajib & Standar Teknis

1. **Integritas Target**: DILARANG KERAS mengubah, menghapus, atau merusak file apa pun di folder instalasi target `C:\Program Files (x86)\TorPedo Traffic Generator Ultimate`.
2. **Kepatuhan Arsitektur Nuxtor**:
   - Wajib mengikuti panduan `nuxtor-desktop-app` (`~/.agents/skills/nuxtor-desktop-app/SKILL.md`).
   - **Frontend Nuxt UI v4**: Semua komponen UI wajib menggunakan Nuxt UI (`UDashboardGroup`, `UDashboardSidebar`, `UDashboardPanel`, `UDashboardNavbar`, `UTable`, `UProgress`, `UModal`, `UTabs`, `UForm`, `UFormField`, `UInput`, `USelect`, `USwitch`, `UButton`, `UBadge`, `UAlert`, dll.). Komponen native HTML hanya boleh untuk wrapper layout (`div`) atau jika Nuxt UI belum menyediakannya.
   - **Backend Tauri 2 (Rust)**: Semua command yang berhubungan dengan I/O, eksekusi proses Playwright, jaringan, dan disk wajib menggunakan `#[tauri::command(async)]` (atau `async fn`) agar jendela UI tidak pernah membeku.
   - **IPC Satu Pintu**: Komposisi frontend hanya memanggil backend via composable terpusat (misal `useTrafficEngine.ts`), halaman tidak memanggil `invoke` langsung.
3. **Keamanan & Guarding**:
   - Seluruh proses child (Playwright/Node) wajib dikontrol daur hidupnya (clean shutdown saat window ditutup, tidak meninggalkan proses yatim/orphan di background).
   - Simpan kredensial (seperti BrowserStack API Key) dengan aman, jangan disimpan dalam plain text di log publik.
4. **Aturan Git Bersih**:
   - Dilarang menyertakan nama AI (Gemini, Claude, Roo, Antigravity, dll.) dalam commit message, PR, branch name, file release, maupun kode sumber.
   - Jangan commit file spesifik AI (`.agents/`, `.gemini/`, `.claude/`, `.roo/`, `AGENTS.md`, `GEMINI.md`, dll.) ke dalam git repositori.
   - Push hanya jika diminta secara eksplisit oleh pengguna.

---

## 3. Persiapan Lingkungan

1. **Pelajari Skill Terkait**:
   - Wajib membaca `nuxtor-desktop-app` (`~/.agents/skills/nuxtor-desktop-app/SKILL.md`).
   - Pelajari skill `reverse-engineer-anything` untuk teknik analisis biner.
   - Pelajari skill `pdf-inspector-windows` untuk membaca berkas dokumentasi PDF (`Torpedo User Guide.pdf`).
2. **Ketersediaan Tool REA**:
   - Jika diperlukan inspeksi assembly .NET, jalankan diagnosa terlebih dahulu:
     `npx -y rea-agents@6.5.0 doctor --client gemini_cli --json` (atau sesuai client aktif).
   - Jangan menjalankan instalasi invasif tanpa izin.
3. **Struktur Folder Kerja**:
   - Root: `E:\website\softwere\flashtraffic`
   - Siapkan folder `research/` untuk menampung hasil analisis.
   - Buat `.gitignore` yang bersih sejak awal (tidak mengabaikan `src-tauri/src`).

---

## 4. Fase 1: Riset Fitur & Reverse Engineering Berbasis Bukti

Tujuan fase ini adalah mengekstrak seluruh kapabilitas, logika kerja, arsitektur data, dan skenario traffic dari TorPedo Traffic Generator Ultimate ke dalam `research/feature-inventory.md`.

### Langkah Kerja Fase 1:
1. **Analisis Dokumentasi Asli (PDF)**:
   - Target: `C:\Program Files (x86)\TorPedo Traffic Generator Ultimate\Torpedo User Guide.pdf`
   - Ekstrak isi dan panduan pengguna menggunakan tool `pdf-inspector-windows` (`& "$env:USERPROFILE\.pdf-inspector-venv\Scripts\python.exe" "<skill_dir>\scripts\read_pdf.py"`).
   - Catat semua modul fitur: Direct Traffic, Organic Search Traffic (Google, Bing, Yahoo), Referral Traffic, Social Media Traffic, Campaign Schedule, Multi-threading, Bounce Rate Control, dan Scroll/Click Behavior.
2. **Analisis Assembly Biner (.NET & CefSharp)**:
   - Target: `TorpedoUltimate.dll` dan biner terkait di `C:\Program Files (x86)\TorPedo Traffic Generator Ultimate\`.
   - Gunakan REA (`inspect_managed_artifact`, `inspect_managed_members`, atau `open_binary`) untuk memeriksa struktur class, model data campaign, format file penyimpanan (.torpedo/.json), algoritma rotasi proxy, dan cara aplikasi lama mengontrol CefSharp.
3. **Penyusunan Inventarisasi Fitur**:
   - Buat file `research/feature-inventory.md` yang memuat:
     - **Ringkasan Arsitektur Lama**: CefSharp WPF, .NET runtime, multithreading model.
     - **Katalog Fitur Komprehensif**: Tabel detail setiap fitur lama, parameter konfigurasi, dan batasan teknologinya.
     - **Gap Analysis & Rencana Modernisasi**: Perbandingan fitur lama vs padanan modern yang akan diimplementasikan di FlashTraffic (misal CefSharp diganti Playwright, integrasi cloud BrowserStack, UI Nuxt UI).
     - **Logika & Algoritma Utama**: Pengaturan interval kunjungan, dwell time, simulasi bounce rate, kalkulasi scroll depth, random path clicking.

> 🛑 **CHECKPOINT 1:**
> Berhenti di sini. Tampilkan ringkasan hasil riset fitur dari `research/feature-inventory.md` kepada pengguna. Minta konfirmasi dan catatan tambahan sebelum melangkah ke penulisan PRD.

---

## 5. Fase 2: Penyusunan Dokumen PRD (`PRD.md`)

Setelah inventarisasi fitur disetujui, susun dokumen `PRD.md` di root proyek (`E:\website\softwere\flashtraffic\PRD.md`).

### Struktur Wajib `PRD.md`:
1. **Product Overview & Problem Statement**:
   - Latar belakang mengapa traffic generator modern dibutuhkan (testing stabilitas web, validasi analitik, load testing, SEO behavior simulation).
   - Visi produk: Aplikasi desktop tangguh, estetis, aman, dan modular.
2. **User Personas & Use Cases**:
   - Web Developer / QA Engineer (stress test & UI automation).
   - SEO Specialist & Digital Marketer (testing attribution & analytics tag verification).
3. **System Architecture**:
   - Diagram arsitektur: Frontend Nuxt 4 + Tauri 2 Rust Backend + Local Playwright Runner + Cloud BrowserStack Bridge.
   - Model komunikasi IPC (Tauri events & commands).
   - Manajemen State (Pinia).
4. **Spesifikasi Fitur Detail (Functional Requirements)**:
   - **Campaign Manager**: Multi-scenario builder (Direct, Search Organic, Referral, Social), konfigurasi URL, bobot halaman, kedalaman kunjungan (inner pages).
   - **Traffic Behavior Simulator (Playwright Engine)**:
     - Dwell time (min/max waktu di halaman).
     - Scroll simulation (kecepatan variabel, jeda baca alami).
     - Link clicking & internal navigation simulation.
     - Bounce rate regulation (persentase keluar tanpa klik lanjutan).
   - **Cloud Traffic Engine (BrowserStack Integration)**:
     - Otentikasi & manajemen kredensial BrowserStack.
     - Pemilihan browser/OS matrix (Windows, macOS, iOS, Android).
     - Live testing monitoring & capture session links.
   - **Proxy & Network Orchestrator**:
     - Format impor (IP:Port, IP:Port:User:Pass).
     - Protokol: HTTP, HTTPS, SOCKS4, SOCKS5.
     - Fitur rotasi per request, per campaign, atau per interval waktu.
     - Health checker otomatis (tes latency, tes anonimitas/IP leak).
   - **Fingerprint & Identity Rotation**:
     - User-Agent library (Desktop & Mobile terkini).
     - Viewport & screen resolution randomization.
     - Timezone, Geo-coordinates, dan HTTP Headers.
   - **Real-Time Monitoring & Analytics**:
     - Live active visitor counter, success/error rates.
     - Live execution log viewer & visual preview / snapshot capture.
5. **Rancangan Antarmuka & Pemetaan Komponen Nuxt UI**:
   - Tabel pemetaan view/halaman ke komponen Nuxt UI (misal `UDashboardGroup`, `UDashboardSidebar`, `UTable`, `UModal`, `UProgress`, dll.).
   - Aturan token tema (`app.config.ts`, `neutral`, `primary`).
6. **Non-Functional Requirements**:
   - Performa CPU/RAM, proteksi leak proses browser, kompatibilitas offline, kecepatan startup.
7. **Roadmap Rilis**:
   - MVP (Fase 1 rilis lokal), Fase Integrasi Cloud, dan Fitur Lanjutan.

> 🛑 **CHECKPOINT 2:**
> Berhenti di sini. Tampilkan draf `PRD.md` kepada pengguna. Minta review, revisi fitur, atau persetujuan resmi sebelum memulai inisialisasi kode aplikasi.

---

## 6. Fase 3: Pembangunan Aplikasi Desktop (Nuxtor + Tauri + Rust)

Setelah PRD disetujui, mulai pembangunan aplikasi di folder `E:\website\softwere\flashtraffic`:

### 6.1 Setup Proyek & Pembersihan Identitas Template
1. Clone template Nuxtor:
   ```bash
   gh repo clone NicolaSpadari/nuxtor .
   rm -rf .git && git init -b main
   bun install
   ```
2. Ganti seluruh identitas template (cari kata kunci `nuxtor` dan `Nuxtor`):
   - `package.json`: `name` (`flashtraffic`), `version` (`0.1.0`), deskripsi, lisensi.
   - `src-tauri/Cargo.toml`: nama paket, versi, penulis, deskripsi.
   - `src-tauri/tauri.conf.json`: `productName` ("FlashTraffic"), `identifier` (`com.flashtraffic.app`), judul jendela, bundle info.
   - `nuxt.config.ts`: title, meta tags, konfigurasi ikon offline.
   - Setup layout, sidebar, nama dan logo aplikasi.
3. Kunci dependensi di `package.json` dan `Cargo.toml`.

### 6.2 Frontend Nuxt UI v4
1. Terapkan tata letak berbasis Nuxt UI Dashboard:
   - Sidebar navigasi: Dashboard/Overview, Campaigns, Browser Engine (Playwright), Cloud Runner (BrowserStack), Proxies, Settings, Logs.
2. Bangun halaman dengan kepatuhan 100% pada komponen Nuxt UI:
   - Form Campaign: `UFormField`, `UInput`, `USelect`, `USwitch`, `USlider`.
   - Proxy Manager: `UTable` dengan badge status (`UBadge`), tombol aksi (`UButton`), import modal (`UModal`).
   - Live Dashboard: Metrik ringkas (`UCard`), progress eksekusi (`UProgress`), log streaming real-time.
3. Composable `useTrafficBackend.ts` sebagai perantara tunggal ke Tauri invoke.

### 6.3 Backend Tauri 2 (Rust) & Integrasi Engine
1. **Modul Rust Terpisah**:
   - `commands.rs`: Handler command async untuk kontrol campaign, manajemen proxy, konfigurasi.
   - `playwright.rs`: Modul pengelola child process Playwright (runner script Node.js / CLI), handling lifecycle proses, stdout/stderr parsing, kill signal terpusat.
   - `browserstack.rs`: REST client async (`reqwest`) untuk berkomunikasi dengan BrowserStack Automate API.
   - `proxy.rs`: Proxy validator, latency tester, IP resolver.
   - `safety.rs`: Penjaga stabilitas, pembatalan task, dan pembersihan proses saat shutdown.
2. **Keamanan & Capability**:
   - Minimal capability di `src-tauri/capabilities/default.json`.
   - Atur CSP ketat di `tauri.conf.json`.

### 6.4 Verifikasi & Quality Gates
Jalankan seluruh pengujian sebelum menyatakan aplikasi selesai:
```bash
bun run lint
bun run typecheck
bun run generate
cargo clippy --all-targets --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml
bun run desktop:build
```

> 🛑 **CHECKPOINT 3:**
> Berhenti di sini. Uji coba aplikasi desktop secara langsung bersama pengguna. Tunjukkan installer yang dihasilkan dan pastikan seluruh fitur berjalan tanpa kendala sebelum menyusun dokumentasi.

---

## 7. Fase 4: Situs Dokumentasi & Halaman Unduhan (Nuxt UI Docs)

Membangun situs dokumentasi dan halaman rilis di subdirektori `docs/` menggunakan template Nuxt UI Docs untuk di-deploy ke GitHub Pages.

### 7.1 Setup Template Nuxt UI Docs
1. Clone template resmi:
   ```bash
   gh repo clone nuxt-ui-templates/docs docs -- --depth 1
   rm -rf docs/.git
   ```
2. Pembersihan untuk GitHub Pages statis:
   - Buang: `server/`, `@nuxtjs/mcp-toolkit`, `nuxt-agent-discovery`, `nuxt-og-image` beserta folder `OgImage/`.
   - Hapus `PageHeaderLinks.vue` dan `TemplateMenu.vue`.
   - Bersihkan konten contoh bawaan, file pnpm, dan konfigurasi yang tidak perlu.
3. Konfigurasi `docs/nuxt.config.ts`:
   - `app.baseURL: '/<nama-repo>/'`
   - Kunci dependensi dan jalankan `bun install`.

### 7.2 Komponen Interaktif Kustom (di `docs/app/components/content/`)
- **`DownloadCard.vue`**:
  - Fetch dinamis rilis terbaru dari GitHub API: `https://api.github.com/repos/<user>/<repo>/releases/latest` (`useFetch` dengan `{ server: false, lazy: true }`).
  - Menampilkan tombol unduh `.exe` (utama) dan `.msi`, versi, ukuran, tanggal rilis, dan badge status.
  - Fallback informatif jika rilis belum dipublikasikan.
  - Semua elemen menggunakan `UCard`, `UButton`, `UBadge`, `UAlert`, `USkeleton`.
- **`ScreenshotCarousel.vue`**:
  - Menggunakan `UCarousel` dengan prop `loop`, `arrows`, `dots`, dan `:autoplay="{ delay: 4500, stopOnMouseEnter: true, stopOnInteraction: false }"`.
  - Menampilkan screenshot asli antarmuka aplikasi dari folder `docs/public/screenshots/`.
  - Dilengkapi caption/penjelasan singkat di setiap slide.

### 7.3 Pengambilan Screenshot Visual Asli
Sesuai standar skill `nuxtor-desktop-app`, screenshot aplikasi desktop diambil secara visual dan bersih:
- Jalankan biner aplikasi dengan flag debugging:
  `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9333`
- Hubungkan via CDP (menggunakan Obscura atau script evaluasi browser), tunggu hidrasi data selesai, lalu tangkap screenshot beresolusi tinggi (misal 1600×1000) dan simpan ke `docs/public/screenshots/`.
- Periksa gambar untuk memastikan tidak ada data rahasia atau identitas pribadi yang bocor.

### 7.4 Konten Dokumentasi
- `index.md`: Hero section dengan `:download-card`, carousel fitur `:screenshot-carousel`, ringkasan keunggulan, CTA.
- `1.getting-started/`: Pengenalan, Panduan Instalasi Windows (penjelasan SmartScreen), Menjalankan Campaign Pertama.
- `2.features/`: Panduan detail Playwright Engine, BrowserStack Cloud Setup, Proxy Management, Simulasi Perilaku Manusia.
- `3.privacy-security/`: Kebijakan privasi lokal, penyimpanan kredensial, tanpa telemetri pihak ketiga.
- `4.development/`: Panduan build from source (Bun + Rust Tauri).

### 7.5 Workflow CI/CD (.github/workflows/)
- `ci.yml`: Linting, typechecking, test frontend & Rust.
- `release.yml`: Pembuatan installer Windows otomatis saat push git tag `v*`.
- `docs.yml`: Build statis `docs/` dan publikasi ke GitHub Pages.

> 🛑 **CHECKPOINT 4:**
> Berhenti di sini. Tunjukkan preview situs dokumentasi (`bun run dev` di `docs/`) kepada pengguna. Konfirmasi publikasi GitHub Pages dan penyelesaian seluruh alur proyek.

---

## 8. Checklist Akhir & Kepatuhan Standar

Sebelum menyatakan seluruh tugas selesai, pastikan checklist berikut tercentang 100%:
- [ ] Analisis `research/feature-inventory.md` selesai dan diverifikasi pengguna
- [ ] Dokumen `PRD.md` disetujui pengguna
- [ ] Seluruh nama dan identitas template Nuxtor dibersihkan (tanpa residu nama lama)
- [ ] 100% komponen UI menggunakan Nuxt UI
- [ ] Tidak ada kartu status palsu atau klaim fitur tanpa backend
- [ ] Command berat Rust bersifat async; UI tidak pernah freeze
- [ ] Child process Playwright tertangani secara aman tanpa leak proses
- [ ] Integrasi BrowserStack REST API teruji
- [ ] Seluruh dependensi terkunci versinya
- [ ] `bun run lint`, `typecheck`, `cargo clippy`, `test` semuanya lulus tanpa error
- [ ] Installer `.exe` dan `.msi` berhasil di-build secara lokal
- [ ] Dokumentasi `docs/` rapi dengan `DownloadCard` dan `ScreenshotCarousel`
- [ ] Git commit bersih tanpa nama atau atribusi AI
