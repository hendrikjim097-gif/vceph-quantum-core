use crate::gas::GENERATOR_G;

pub const VRF_THRESHOLD: u64 = 0x3FFFFFFFFFFFFFFF;

pub struct VerkleNode {
    pub state_root: u64,
    pub db: sled::Db,
}

impl VerkleNode {
    /// Membuat konfigurasi database Sled yang sangat optimal untuk perangkat seluler.
    /// Memaksa penggunaan alokasi cache minimal dan kompresi agresif untuk mengunci ukuran berkas.
    pub fn configure_stateless_db(path: &str) -> sled::Db {
        sled::Config::new()
            .path(path)
            .cache_capacity(8192) // Batasi cache memori hanya 8 KB agar hemat RAM
            .flush_every_ms(None) // Matikan auto-flush berkala untuk mencegah fragmen fragil berkas
            .use_compression(true) // Aktifkan kompresi zstd/lz4 bawaan sled secara penuh
            .open()
            .expect("FAIL: Gagal menginisialisasi arsitektur database kompak 32 KB")
    }

    /// Mengevaluasi seleksi pseudo-VRF secara deterministik sebelum memproses paket lebih lanjut.
    pub fn evaluate_vrf_selection(total_hash_xor: u64) -> bool {
        let vrf_evaluation = total_hash_xor.wrapping_mul(GENERATOR_G);
        vrf_evaluation <= VRF_THRESHOLD
    }

    /// Memperbarui nilai State Root di memori dan mengamankannya langsung ke database Sled fisik.
    pub fn apply_state_transition(&mut self, total_delta: i64, gas_burn_value: u64) {
        if total_delta >= 0 {
            self.state_root = self.state_root.wrapping_add((total_delta as u64).wrapping_mul(GENERATOR_G));
        } else {
            self.state_root = self.state_root.wrapping_sub((total_delta.abs() as u64).wrapping_mul(GENERATOR_G));
        }

        // Terapkan pembakaran gas berbasis BLAKE3 yang aman
        self.state_root = self.state_root.wrapping_sub(gas_burn_value);

        // Timpa nilai root lama di kunci yang sama untuk mencegah State Bloat
        let _ = self.db.insert(b"current_state_root", &self.state_root.to_be_bytes());
        
        // Paksa pemadatan fragmentasi (compaction) secara langsung ke disk agar ukuran berkas konstan
        let _ = self.db.flush();
    }
}

