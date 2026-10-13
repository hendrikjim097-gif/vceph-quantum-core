use pqcrypto_dilithium::dilithium3::{verify_detached_signature, DetachedSignature as DilithiumSig, PublicKey as DilithiumPub};
use pqcrypto_traits::sign::PublicKey as TraitPub;
use pqcrypto_traits::sign::DetachedSignature as TraitSig;

/// Memvalidasi struktur array byte tanda tangan dan kunci publik Dilithium3 secara statis.
/// Dilithium3 mewajibkan ukuran Kunci Publik = 1952 byte dan Tanda Tangan = 3296 byte.
pub fn validate_dilithium3_dimensions(pubkey: &[u8], signature: &[u8]) -> bool {
    pubkey.len() == 1952 && signature.len() == 3296
}

/// Melakukan verifikasi tanda tangan pasca-kuantum (ML-DSA / Dilithium3) terhadap pesan batch.
pub fn verify_quantum_signature(
    pubkey_bytes: &[u8],
    signature_bytes: &[u8],
    msg: &[u8],
) -> Result<(), &'static str> {
    let pqc_pubkey = DilithiumPub::from_bytes(pubkey_bytes)
        .map_err(|_| "REJECTED: Invalid PubKey")?;
        
    let pqc_sig = DilithiumSig::from_bytes(signature_bytes)
        .map_err(|_| "REJECTED: Invalid Signature")?;

    verify_detached_signature(&pqc_sig, msg, &pqc_pubkey)
        .map_err(|_| "REJECTED: PQC Auth Failed")
}

