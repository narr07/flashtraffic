# Inventarisasi Fitur Berbasis Bukti: TorPedo Traffic Generator Ultimate & Rencana Modernisasi FlashTraffic

> **Status Dokumen:** Selesai (Fase 1: Riset Fitur & Analisis Biner)  
> **Target Analisis:** `C:\Program Files (x86)\TorPedo Traffic Generator Ultimate\`  
> **Artefak Bukti:** `TorpedoUltimate.dll` (v8.6.0.0, .NET 5, CefSharp 122), `Torpedo User Guide.pdf` (22 Halaman)  
> **Tanggal Analisis:** Oktober 2026

---

## 1. Identitas & Lingkungan Biner Target

| Properti | Detail Bukti Biner / Dokumen |
| :--- | :--- |
| **Nama Aplikasi Asli** | TorPedo Traffic Generator Ultimate (sebelumnya: Website Auto Traffic / WAT) |
| **Versi Rilis Target** | Versi 8.6.0.0 (Copyright © 2024 Gem’s Software) |
| **Runtime & Arsitektur** | .NETCoreApp v5.0, Windows x86_64, Windows Presentation Foundation (WPF) |
| **Lokasi Instalasi** | `C:\Program Files (x86)\TorPedo Traffic Generator Ultimate\` |
| **Biner & Assembly Inti** | `TorPedo Traffic Generator Ultimate.exe` (379 KB), `TorpedoUltimate.dll` (1.13 MB) |
| **Engine Browser Lama** | CefSharp.Wpf / CefSharp.Core v122.1.120.0 (`libcef.dll` 212 MB, Chromium 122) |
| **Library Pendukung** | `Newtonsoft.Json.dll` (13.0), `CredentialManagement.dll`, `System.Management.dll` |
| **Interaksi OS Windows** | WinInet API (`INTERNET_PROXY_INFO`) untuk manipulasi proxy level sistem |

---

## 2. Arsitektur & Cara Kerja TorPedo Lama

Berdasarkan ekstraksi dokumentasi [torpedo_user_guide.md](file:///e:/website/softwere/flashtraffic/research/torpedo_user_guide.md) dan dekompilasi metadata [TorpedoUltimate.dll](file:///C:/Program%20Files%20(x86)/TorPedo%20Traffic%20Generator%20Ultimate/TorpedoUltimate.dll):

```
+---------------------------------------------------------------------------------+
|                    WPF GUI (TorpedoUltimate.MainWindow)                         |
|  [SEO General]  [Campaigns]  [Geo Location]  [Platform]  [Scheduler]  [Settings]|
+---------------------------------------------------------------------------------+
         |                                                   |
         v                                                   v
+-----------------------------+               +-----------------------------------+
|  Traffic Configuration Data |               |   Proxy & Network Engine          |
|  - DataG (General URL/Type) |               |   - DataP (Proxy, Cred, Speed)    |
|  - DataCamp (UTM Tracking)  |               |   - AdvProxyCheck (Custom URL)    |
|  - DataSource (UserAgent)   |               |   - WinInetHelper (System Proxy)  |
|  - DataScheduler (Cron/Time)|               |   - Proxynova Scraper Auto-fetch  |
+-----------------------------+               +-----------------------------------+
         |                                                   |
         +--------------------------+------------------------+
                                    |
                                    v
            +-----------------------------------------------+
            |        Browser Engine (CefSharp / libcef)     |
            | - ResourceRequestHandlerExt (Header/Referrer) |
            | - JsHandler (Dialog suppression, scroll)      |
            | - RenderProcessMessageHandler (DOM Hook)      |
            +-----------------------------------------------+
                                    |
                                    v
                           [Target Website]
```

### Karakteristik Kunci TorPedo Lama:
1. **Model Multithreading WPF**: Mengandalkan thread UI utama yang didelegasikan ke thread background (`StartManager`, `TTestProxy`, dll.). Sering kali menyebabkan freezing jika timeout koneksi jaringan terjadi.
2. **Ketergantungan CefSharp**: Memerlukan redistributable runtime Chromium yang sangat besar (~220 MB), konsumsi RAM melonjak saat menjalankan lebih dari beberapa tab browser virtual.
3. **Deteksi Bot**: Menggunakan browser Chromium tertanam tanpa patch antideteksi canggih (navigator.webdriver masih terekspos, fingerprinting canvas/audio/webgl konsisten terlihat sebagai browser otomatis).

---

## 3. Katalog Fitur Komprehensif (Berbasis Bukti)

### 3.1 Modul 1: SEO General Traffic (Organic, Direct, Referral)
Fitur untuk mengarahkan traffic umum ke daftar URL target dengan parameter attribution buatan.

| Parameter / Fitur | Deskripsi & Bukti Implementasi |
| :--- | :--- |
| **Target URL List** | Daftar URL halaman target. Disimpan dalam model `DataG` (`URL`, `BaseURL`). |
| **Traffic Type: Direct** | Membuka URL langsung di address bar tanpa HTTP `Referer`. |
| **Traffic Type: Organic Search** | Mensimulasikan klik hasil pencarian mesin pencari. HTTP `Referer` di-spoofing sesuai search engine (Google, Bing, Yahoo, Yandex, Ask, AOL). Parameter `Keywords` disisipkan ke query search engine. |
| **Traffic Type: Referral** | Mensimulasikan klik tautan dari situs media sosial (Facebook, Twitter/X, Instagram, LinkedIn, Reddit) atau domain rujukan kustom (`Custom Referral`). |
| **Order Selection** | Pilihan urutan kunjungan halaman: *In Order* (berurutan) atau *Randomly Select*. |
| **Page Distribution** | Opsi target traffic: *Total Traffic* (dibagi antar halaman) atau *Average Traffic per Page* (tiap halaman menerima kuota sama). |

### 3.2 Modul 2: SEO Campaign Traffic (Google Analytics Tracking)
Khusus dirancang untuk mensimulasikan traffic kampanye berbayar atau pemasaran yang dapat diidentifikasi oleh Google Analytics (Universal Analytics & GA4).

| Parameter / Fitur | Model Data `DataCamp` | Fungsi |
| :--- | :--- | :--- |
| **Campaign Name** | `CampName` | Menghasilkan parameter `utm_campaign`. |
| **Campaign Medium** | `CampMedium` | Menghasilkan `utm_medium` (e.g. `cpc`, `email`, `banner`, `social`). |
| **Campaign Source** | `CampSource` | Menghasilkan `utm_source` (e.g. `newsletter`, `google`, `partner_site`). |
| **Campaign Content** | `CampContent` | Menghasilkan `utm_content` (opsional untuk A/B testing). |
| **Campaign Term** | `CampTerm` | Menghasilkan `utm_term` (kata kunci iklan). |

### 3.3 Modul 3: Geolocation & Proxy Orchestration
Manajemen proxy untuk memanipulasi lokasi geografis kunjungan.

| Fitur | Bukti Implementasi TorPedo |
| :--- | :--- |
| **Dukungan Protokol** | HTTP, HTTPS, SOCKS4, SOCKS5 (mulai v8.1+ wajib HTTPS/SSL). |
| **Format Proxy** | `IP:Port`, `IP:Port:User:Pass`, domain name `host:port` (didukung di v8.2+). |
| **Sumber Proxy Otomatis** | Impor file teks atau auto-fetch berkala dari URL web scraper (termasuk modul internal `GetProxyFromProxynova`). |
| **Klasifikasi Anonimitas** | *Transparent* (IP asli bocor), *Anonymous* (proxy terdeteksi tapi IP asli aman), *Elite* (proxy tidak terdeteksi sama sekali). |
| **Proxy Health Checker** | Pengecekan latency (`Speed` dalam persentase warna: Green >= 85%, Red 50-84%, Gray < 50%). Auto-reject proxy mati atau lambat. |
| **Advanced Proxy Checker** | `AdvProxyCheck.xaml`: Menguji apakah proxy tertentu berhasil me-load halaman URL target riil dengan lengkap sebelum dimasukkan ke antrean. |
| **Rotasi Proxy** | Pilihan: *Per Request*, *Random Proxy*, atau *Time-based Interval (ms)* acak antara Min dan Max ms. |

### 3.4 Modul 4: Platform & Device Identity Spoofing
Mensimulasikan berbagai perangkat, browser, dan resolusi layar pengguna.

| Fitur | Detail Bukti Biner (`DataSource`) |
| :--- | :--- |
| **Sistem Operasi** | Windows, Linux, Android, macOS, iOS (iPhone/iPad). |
| **Browser Didukung** | Google Chrome, Mozilla Firefox, Microsoft Edge, Safari, Opera, Maxthon, Internet Explorer. |
| **Resolusi Layar** | Konfigurasi resolusi viewport (`ScreenRes`), misal: `1920x1080`, `1366x768`, `414x896`, `375x667`. |
| **Strategi Pergantian** | *Single Platform* (tetap), *Random Platform* (setiap kunjungan berbeda), *Variable Time Switching* (ganti platform setelah interval acak Min-Max). |

### 3.5 Modul 5: Traffic Behavior & Session Mechanics
Pengaturan interaksi dan perilaku pengunjung di dalam website.

| Fitur Perilaku | Logika Kerja |
| :--- | :--- |
| **Traffic Presets** | 1. *Default*: Cocok untuk website baru.<br>2. *Max Sessions*: Meningkatkan durasi sesi.<br>3. *Max Pageviews*: Fokus memperbanyak hit halaman.<br>4. *Max Bounce Rate*: Tingkat pentalan tinggi.<br>5. *Min Bounce Rate*: Menurunkan pentalan dengan interaksi multi-halaman. |
| **Session Duration (Dwell Time)** | Durasi waktu bertahan di halaman, diatur konstan atau rentang acak antara `Min` dan `Max` detik. |
| **Special Modes** | 1. *Repetition Mode*: Pengunjung berulang kali merefresh halaman yang sama sesuai nilai `Count`.<br>2. *Surfing / Browsing Mode*: Pengunjung menavigasi dari satu halaman internal ke halaman internal lainnya dalam sesi yang sama sebelum berganti sesi/IP. |
| **Auto Exit Timer** | Aplikasi dapat berhenti otomatis pada jam tertentu. |

### 3.6 Modul 6: Scheduler & Background Execution
| Fitur | Detail |
| :--- | :--- |
| **Multi-Schedule Queue** | Menjadwalkan jam mulai campaign (`DataScheduler`: `Name`, `Occurrence`, `StartTime`). |
| **Resolusi Konflik** | Jika jadwal baru aktif sebelum jadwal lama selesai, sistem menghentikan jadwal lama secara aman. |
| **System Tray** | Meminimalkan aplikasi ke system tray Windows (`NotifyIcon`) agar tetap berjalan di background. |

### 3.7 Modul 7: URL / Link Fetcher (Crawler)
Window khusus (`URLFetcher.xaml`) yang menerima URL domain, men-crawl halaman untuk mengumpulkan semua tautan internal (`<a> href`), dan memungkinkan pengguna mengekspor atau langsung memindahkannya ke tabel General Traffic.

---

## 4. Model Data & Skema Parameter

Berikut adalah struktur model data asli di `TorpedoUltimate.dll`:

```csharp
// 1. General Traffic Item
public class DataG {
    public string SlNo { get; set; }
    public string URL { get; set; }
    public string BaseURL { get; set; }
    public string TrafficType { get; set; }    // Direct, Organic, Referral
    public string TrafficSource { get; set; }  // Google, Bing, Facebook, Custom URL
    public string Keywords { get; set; }       // SEO search query
}

// 2. Proxy Item
public class DataP {
    public string SlNo { get; set; }
    public string Proxy { get; set; }          // ip:port or host:port
    public string Credential { get; set; }     // user:pass
    public string Speed { get; set; }          // latency / quality percentage
    public string Type { get; set; }           // Transparent, Anonymous, Elite
    public string Country { get; set; }        // Geolocation Country
}

// 3. Platform / User-Agent Profile
public class DataSource {
    public string SlNo { get; set; }
    public string Browser { get; set; }        // Chrome, Firefox, Safari
    public string BrowserVersion { get; set; }
    public string OSorDevice { get; set; }     // Windows, Mac, Android, iOS
    public string OSVersion { get; set; }
    public string ScreenRes { get; set; }      // 1920x1080, etc.
}

// 4. UTM Campaign Item
public class DataCamp {
    public string SlNo { get; set; }
    public string URL { get; set; }
    public string BaseURL { get; set; }
    public string CampName { get; set; }       // utm_campaign
    public string CampMedium { get; set; }     // utm_medium
    public string CampSource { get; set; }     // utm_source
    public string CampContent { get; set; }    // utm_content
    public string CampTerm { get; set; }       // utm_term
}

// 5. Scheduler Task
public class DataScheduler {
    public string SlNo { get; set; }
    public string Name { get; set; }
    public string Occurrence { get; set; }    // Once, Daily, Weekly
    public string StartTime { get; set; }     // Scheduled time
}
```

---

## 5. Kelemahan & Batasan Arsitektur TorPedo Lama

1. **Mesin Browser Usang (CefSharp 122 WPF)**:
   - Terbatas pada satu instance Chromium monolitik lokal.
   - Sering crash atau out-of-memory saat membuka tab dalam jumlah banyak.
   - Interaksi pengguna (mouse move, scroll) hanya berupa script JavaScript dasar `window.scrollTo()` yang sangat mudah dideteksi oleh skrip anti-bot.
2. **Tidak Mendukung Eksekusi Cloud**:
   - Seluruh traffic bergantung pada bandwidth, CPU, dan koneksi internet PC lokal.
   - Tidak ada integrasi dengan platform cloud testing atau perangkat nyata.
3. **UI Membeku (Thread Blocking)**:
   - Operasi pengecekan proxy dan koneksi jaringan menggunakan `WebClient` sinkron yang sering menyebabkan UI WPF tidak merespons (Not Responding).
4. **Spoofing Mobile Palsu**:
   - Hanya mengganti string User-Agent, sedangkan arsitektur browser tetap Chromium desktop x86_64 dengan screen touch event yang tidak aktif.

---

## 6. Gap Analysis & Rencana Modernisasi FlashTraffic

| Dimensi | TorPedo Traffic Generator Ultimate (Lama) | FlashTraffic (Modernisasi Baru) | Keuntungan & Value |
| :--- | :--- | :--- | :--- |
| **Frontend Framework** | WPF (.NET 5) x86_64 monolitik | **Nuxt 4 + Nuxt UI v4** (Vue 3, Tailwind v4, UDashboard) | UI modern, responsif, dark mode, animasi halus, data-reactive |
| **Backend & Runtime** | C# .NET Core 5.0 | **Tauri 2 (Rust Core Engine)** | Bobot ringan (~15 MB vs 300+ MB), startup instan, memori hemat, 100% async |
| **Browser Engine** | CefSharp WPF tertanam (`libcef.dll`) | **Playwright Runner (Chromium, Firefox, WebKit)** | Full stealth emulation, CDP support, humanized cursor trajectory, realistis |
| **Cloud Device Integration** | *Tidak ada* (hanya lokal) | **BrowserStack Automate & Live API** | Akses ke ribuan real iOS/Android devices & desktop browser cloud di 100+ negara |
| **Simulasi Perilaku Manusia** | `window.scrollTo()` kasar | Bezier mouse curves, scroll physics dengan viewport awareness, random text selection & dwell time | Lolos deteksi bot Google Analytics 4, Cloudflare Turnstile, Cloudflare WAF |
| **Manajemen Proxy** | Text file & WebClient dasar | Async Tokio Rust Pool, HTTP/HTTPS/SOCKS5, DNS Leak Protection, Latency Ping async | Kecepatan tes proxy 20x lipat, tidak membekukan aplikasi |
| **Logging & Metrik** | Textbox scrollable biasa | Real-time streaming log, kartu metrik live (`UProgress`, `UCard`), live snapshot canvas | Visibilitas instan terhadap keberhasilan kampanye |
| **Keamanan & OS** | Menyentuh WinInet sistem | Terisolasi per-browser-context (Isolated Browser Context), tidak merusak koneksi OS | Aman, bersih, tidak meninggalkan jejak registry |

---

## 7. Kesimpulan & Rekomendasi untuk PRD

Riset fitur membuktikan bahwa seluruh kapabilitas fungsional TorPedo (Direct, Search Organic, Referral, UTM Campaign, Proxy rotasi, Device Spoofing, Scheduler, dan Link Fetcher) dapat direproduksi secara superior di **FlashTraffic** menggunakan:
1. **Frontend**: Nuxt UI v4 (`UDashboard*`, `UTable`, `UModal`, `UForm`, dll.).
2. **Backend**: Tauri 2 (Rust) dengan async task runner.
3. **Engine Automasi**: Playwright sidecar untuk pengujian lokal + BrowserStack cloud bridge untuk pengujian cloud.

Hasil riset ini siap dijadikan fondasi spesifikasi teknis pada **Fase 2: Dokumen PRD (`PRD.md`)**.
