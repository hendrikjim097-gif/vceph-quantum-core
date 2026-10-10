use blake3::Hash;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ProtectedShard {
    pub shard_id: u8,
    pub data: Vec<u8>,
    pub commitment: [u8; 32], // Hash BLAKE3 dari data asli untuk validasi anti-racun
}

impl ProtectedShard {
    // 1. Membuat Shard baru sekaligus mengunci komitmen hash data
    pub fn new(shard_id: u8, data: Vec<u8>) -> Self {
        let commitment = blake3::hash(&data).into();
        Self { shard_id, data, commitment }
    }

    // 2. Fungsi Proteksi: Memeriksa apakah data shard telah dimanipulasi/diracuni
    pub fn verify_integrity(&self) -> Result<(), &'static str> {
        let calculated_hash: [u8; 32] = blake3::hash(&self.data).into();
        if calculated_hash == self.commitment {
            Ok(())
        } else {
            Err("CRITICAL: Shard poisoning detected! Data commitment mismatch.")
        }
    }
}

// 3. Rekonstruksi Aman: Memulihkan saksi saldo hanya jika lolos uji integritas
pub fn secure_reconstruct(shard_1: &ProtectedShard, shard_3: &ProtectedShard) -> Result<Vec<u8>, &'static str> {
    // Validasi kedua kepingan shard sebelum melakukan operasi XOR
    shard_1.verify_integrity()?;
    shard_3.verify_integrity()?;

    if shard_1.data.len() != shard_3.data.len() {
        return Err("Error: Shard length mismatch during recovery.");
    }

    // Eksekusi XOR murni jika data terbukti steril 100%
    let reconstructed: Vec<u8> = shard_1.data.iter()
        .zip(shard_3.data.iter())
        .map(|(a, b)| a ^ b)
        .collect();

    Ok(reconstructed)
}
