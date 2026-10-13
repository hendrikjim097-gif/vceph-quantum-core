# 🗺️ V-CEPH Quantum Core - Technical Architecture

Dokumen ini menjelaskan spesifikasi matematis dan rekayasa internal yang mendasari mesin *stateless ledger* V-CEPH.

## 1. Jaringan & Enkripsi Lapis Batas (Border Validation)

Server mendengarkan pada port `8080` (UDP/QUIC via `quinn`) menggunakan kebijakan pembatasan fisik paket:
- Ukuran minimum paket biner: **200 Byte**
- Ukuran maksimum paket biner: **8000 Byte**
- Validasi dimensi kunci publik Dilithium3 (ML-DSA): **1952 Byte**
- Validasi dimensi tanda tangan digital Dilithium3 (ML-DSA): **3296 Byte**

## 2. Konsensus Pseudo-VRF (Verifiable Random Function)

Setiap paket dievaluasi menggunakan *deterministic random multiplier* sebelum memicu komputasi kriptografi yang berat:

\[\text{VRF\_Evaluation} = \text{total\_hash\_xor} \times \text{GENERATOR\_G} \pmod{2^{64}}\]

Jika VRF\_Evaluation > VRF\_THRESHOLD (di mana threshold diatur pada nilai 0x3FFFFFFFFFFFFFFF), paket langsung dibuang demi mengamankan CPU dari serangan Sybil/DoS.

## 3. One-Way Homomorphic Gas Burn Engine

Untuk mencegah pelacakan balik (*reversibility*) terhadap *State Root* oleh entitas luar, konsumsi gas dilewatkan melalui fungsi *one-way cryptographic hashing* **BLAKE3**:

1. Gas\_Consumed = Payload\_Size\_Bytes × GAS\_PER\_BYTE\_RATE
2. \(\text{Entropy\_Nonce} = \text{BLAKE3}(\text{Gas\_Consumed} \parallel \text{GENERATOR\_G})\)
3. \(\text{Gas\_Burn\_Value} = \text{Extract\_U64}(\text{Entropy\_Nonce}_{[0..8]})\)

Nilai transisi pembakaran akhir disuntikkan ke status ledger:

\[\text{New\_State\_Root} = \text{Old\_State\_Root} - \text{Gas\_Burn\_Value}\]

## 4. Dynamic 2-of-3 Threshold Sharding

Distribusi data ke *Vault Node* menggunakan skema pembagian bitwise XOR dinamis yang kebal terhadap manipulasi internal (*anti-shard poisoning*):

- **Kombinasi Valid:** S1 \(\oplus\) S2, S2 \(\oplus\) S3, atau S1 \(\oplus\) S3.
- **Validasi Integritas:** Sebelum rekonstruksi dilakukan, setiap pecahan biner wajib divalidasi silang menggunakan data komitmen hash tepercaya yang tercatat pada *ledger state root* utama:
  \[\text{BLAKE3}(\text{Shard\_Data}) \equiv \text{Trusted\_Commitment}\]

