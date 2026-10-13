pub const GENERATOR_G: u64 = 0x54321AFB;
pub const GAS_PER_BYTE_RATE: u64 = 1;

/// Menghitung konsumsi energi komputasi berdasarkan byte fisik paket data yang masuk.
pub fn calculate_gas_consumption(payload_size_bytes: u64) -> u64 {
    payload_size_bytes * GAS_PER_BYTE_RATE
}

/// Menghitung nilai pemotongan state root secara kriptografis berbasis One-Way Homomorphic Burn.
/// Menggunakan BLAKE3 untuk memastikan nilai pembakaran bersifat non-linear dan tidak bisa dilacak balik.
pub fn calculate_homomorphic_burn(gas_consumed: u64) -> u64 {
    // 1. Masukkan nilai gas_consumed ke dalam bentuk byte biner
    let gas_bytes = gas_consumed.to_le_bytes();
    let generator_bytes = GENERATOR_G.to_le_bytes();
    
    // 2. Lakukan hashing satu arah menggunakan BLAKE3 menggabungkan Gas dan Generator
    let mut hasher = blake3::Hasher::new();
    hasher.update(&gas_bytes);
    hasher.update(&generator_bytes);
    let hash_result = hasher.finalize();
    
    // 3. Ambil 8 byte pertama hasil hash untuk dikonversi menjadi integer u64 yang valid
    let mut entropy_bytes = [0u8; 8];
    entropy_bytes.copy_from_slice(&hash_result.as_bytes()[0..8]);
    
    // 4. Hasil akhir berupa angka acak deterministik ber-entropi tinggi yang mengikat nilai gas asli
    u64::from_le_bytes(entropy_bytes)
}

