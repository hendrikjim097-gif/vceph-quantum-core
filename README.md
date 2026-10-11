# V-CEPH Quantum Core
> **Ultra-Lightweight, Post-Quantum Secure Stateless Ledger Engine**

V-CEPH Quantum Core adalah mesin pembukuan (*ledger*) terdistribusi pasca-kuantum yang dirancang untuk efisiensi ekstrem. Proyek ini membuktikan bahwa enkripsi pertahanan militer **NIST Dilithium3 (ML-DSA)** dapat berjalan secara *stateless* murni dengan jejak memori yang sangat kecil, bahkan mampu mempertahankan node validator penuh secara stabil di atas perangkat seluler kelas entri.

## 🚀 Extreme Hardware Benchmark (Empirical Evidence)
Arsitektur ini telah divalidasi dan diuji secara murni (`--release`) di atas lingkungan komputasi dengan keterbatasan ekstrem:
* **Host Pengujian:** Xiaomi Redmi Note 9 (MediaTek Helio G85 ARM CPU, Terisolasi via Termux)
* **Ukuran Database Validator (Sled DB):** `32 KB` Konstan (Kebal terhadap State Bloat)
* **Ukuran Payload Paket Transmisi:** `4213 Byte` (Serde/Bincode Binary Encapsulation)

### Metrik Performa Perangkat Seluler (Helio G85):
* **PQC Signing Latency (Dilithium3):** 729.30 µs - 1.45 ms
* **Lattice Keypair Generation:** 266.46 µs - 532.30 µs
* **XOR-Linear Data Matrix Recovery:** 35.94 ms - 62.66 ms
* **QUIC Network Round-Trip Latency:** 4.42 ms - 13.52 ms

---

## 🛠️ Arsitektur Inti & Inovasi Teknologi

### 1. O(1) Stateless Verkle Roots
V-CEPH tidak menyimpan status saldo pengguna pada RAM atau disk validator. Validasi integritas status diverifikasi secara homomorfik menggunakan perkalian titik generator kurva linear terhadap agregasi saksi (*witness*):
\[\text{Calculated\_Point} = (\text{Total\_Hash\_XOR} \oplus \text{Aggregated\_Witness}) \cdot G\]

### 2. 2-of-3 XOR-Lattice Sharding (Fault-Tolerant Erasure Coding)
Data riwayat saldo dipecah secara horizontal menggunakan operasi bitwise XOR primitif yang sangat cepat untuk menghindari overhead komputasi matriks Reed-Solomon:
\[\text{Shard\_3 (Parity)} = \text{Shard\_1} \oplus \text{Shard\_2}\]
Pemulihan data klien yang kehilangan saksi (*witness local*) dapat direkonstruksi secara instan via Vault Node melalui formula:
\[\text{Reconstructed\_Witness} = \text{Local\_Shard\_1} \oplus \text{Shard\_3}\]

### 3. Thermodynamic Gas Metering Engine
Menghapus manipulasi MEV (Maximal Extractable Value) dengan mematok biaya transaksi langsung secara linear terhadap byte biner fisik yang membelah jaringan:
\[\text{Gas\_Consumed} = \text{Payload\_Size\_Bytes} \cdot \text{GAS\_PER\_BYTE\_RATE}\]

---

## 💻 Panduan Operasional Laboratorium

### 1. Jalankan Storage Shard Node (Port 8090)
```bash
cd ~/vceph_storage && cargo run --release
```
### 2. Jalankan Validator Node (Port 8080)
```bash
cd ~/rust_stateless && cargo run --release
```
### 3. Jalankan Simulasi Transaksi Klien
```bash
cd ~/vceph_client && cargo run --release
```

## 📄 Lisensi
Proyek ini dilisensikan di bawah **MIT License** - bebas digunakan untuk kepentingan riset akademis maupun adopsi komersial skala industri.

---

## 🧠 Filosofi Proyek & Cerita Latar Belakang (Human-AI Collaboration)
Proyek V-CEPH Quantum Core tidak lahir di laboratorium universitas ternama dengan server superkomputer berbiaya ribuan dolar. Proyek ini adalah eksperimen radikal yang digarap oleh seorang pengembang awam yang baru mengenal lingkungan **Termux** selama 2 bulan, berkolaborasi secara intensif dengan **Kecerdasan Buatan (AI)** sebagai mitra rekayasa kode.

### Mengapa Proyek ini Ada?
Sistem ini lahir dari satu pertanyaan sederhana: *“Kenapa sebagian besar blockchain warisan masa kini sangat lambat, boros memori, dan ketakutan setengah mati untuk bermigrasi ke Post-Quantum Cryptography (PQC)?”* 

Alih-alih menerima dogma akademis bahwa PQC harus mengorbankan performa, kami memutuskan untuk menguji batas ekstrem arsitektur sistem terdistribusi langsung di atas perangkat keras paling minimalis yang kami miliki: sebuah handphone harian **Xiaomi Redmi Note 9**. 

Di sini, manusia bertindak sebagai arsitek yang merancang konsep, batasan sistem, dan arah visi, sementara AI bertindak sebagai mesin pengetik sintaksis Rust yang presisi. V-CEPH adalah bukti nyata bahwa kolaborasi manusia-AI yang tepat dapat melompati batasan teori matematika yang rumit dan menghasilkan optimasi tingkat rendah yang fungsional.

---

## ☕ Dukung Eksperimen Ini (Dana Kopi / Rokok)

Proyek ini dikembangkan secara mandiri menggunakan lingkungan terbatas **Termux di Xiaomi Redmi Note 9**. Jika cetak biru atau konsep rekayasa sistem berbantuan AI ini bermanfaat untuk riset atau referensi Anda, Anda bisa mendukung kelanjutan eksperimen ini secara aman melalui tautan di bawah ini:

👉 **[Klik di Sini untuk Kirim Dukungan via DANA](https://m.dana.id/n/link/minta?full_url=https://qr.dana.id/v1/281012012024013182100727)**

*Catatan: Tautan di atas menggunakan fitur resmi Minta DANA untuk menjaga privasi nomor telepon pengembang.*
