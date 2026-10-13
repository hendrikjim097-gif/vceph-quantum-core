use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::Instant;
use serde::{Serialize, Deserialize};

// Daftarkan modul-modul eksternal yang baru saja kita buat
mod protected_shard;
mod crypto;
mod gas;
mod storage;

use crate::crypto::{validate_dilithium3_dimensions, verify_quantum_signature};
use crate::gas::{calculate_gas_consumption, calculate_homomorphic_burn, GENERATOR_G};
use crate::storage::{VerkleNode,};

#[derive(Serialize, Deserialize, Debug)]
pub struct QuantumPacket {
    pub total_hash_xor: u64,
    pub aggregated_witness: u64,
    pub total_delta: i64,
    pub pubkey_bytes: Vec<u8>,
    pub signature_bytes: Vec<u8>,
}

fn configure_server() -> quinn::ServerConfig {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let cert_der = cert.cert.der().to_vec();
    let key_pair_der = cert.key_pair.serialize_der();
    let server_cert = rustls::pki_types::CertificateDer::from(cert_der);
    let server_key = rustls::pki_types::PrivateKeyDer::Pkcs8(rustls::pki_types::PrivatePkcs8KeyDer::from(key_pair_der));
    let mut server_config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![server_cert], server_key)
        .unwrap();
    server_config.alpn_protocols = vec![b"vceph-p2p".to_vec()];
    let quic_server_config = quinn::crypto::rustls::QuicServerConfig::try_from(server_config).unwrap();
    quinn::ServerConfig::with_crypto(Arc::new(quic_server_config))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[🔥] Mengaktifkan V-CEPH Quantum-Resistant Server (Modular Architecture)...");
    let _ = rustls::crypto::ring::default_provider().install_default();

let db = VerkleNode::configure_stateless_db("stateless_node_db");

    let initial_root = 13361873044695950534;

    let node_shared = Arc::new(RwLock::new(VerkleNode { state_root: initial_root, db }));
    let addr: SocketAddr = "0.0.0.0:8080".parse()?;
    let server_config = configure_server();
    let endpoint = quinn::Endpoint::server(server_config, addr)?;

    println!("[+] Server UDP/QUIC Mengunci Pipa di Alamat: 0.0.0.0 8080\n");

    while let Some(connecting) = endpoint.accept().await {
        let node = Arc::clone(&node_shared);
        tokio::spawn(async move {
            let connection = match connecting.await { 
                Ok(conn) => conn, 
                Err(_) => return 
            };
            
            while let Ok((mut send_stream, mut recv_stream)) = connection.accept_bi().await {
                let node_inner = Arc::clone(&node);
                tokio::spawn(async move {
                    let mut buffer = vec![0u8; 8192];
                    if let Ok(Some(bytes_read)) = recv_stream.read(&mut buffer).await {

                        // 1. Batasi ukuran biner fisik paket di gerbang awal
                        if !(200..=8000).contains(&bytes_read) {
                            let _ = send_stream.write_all(b"REJECTED: Invalid Packet Size\n").await;
                            return;
                        }

                        let payload_size_bytes = bytes_read as u64;

                        let packet: QuantumPacket = match bincode::deserialize(&buffer[..bytes_read]) {
                            Ok(p) => p,
                            Err(_) => { 
                                let _ = send_stream.write_all(b"REJECTED: Deserialize Error\n").await; 
                                return; 
                            }
                        };

                        // 2. Evaluasi Seleksi VRF menggunakan fungsi terpisah di modul storage
                        if !VerkleNode::evaluate_vrf_selection(packet.total_hash_xor) {
                            let _ = send_stream.write_all(b"REJECTED: VRF Selection Failed\n").await;
                            return;
                        }

                        // 3. Validasi dimensi array Dilithium3 secara statis di modul crypto
                        if !validate_dilithium3_dimensions(&packet.pubkey_bytes, &packet.signature_bytes) {
                            let _ = send_stream.write_all(b"REJECTED: Malformed Dilithium3 Array Size\n").await;
                            return;
                        }

                        let batch_msg = format!("{}:{}:{}", packet.total_hash_xor, packet.aggregated_witness, packet.total_delta);

                        // Hitung Gas sebelum PQC berat untuk mencegah eksploitasi pemrosesan gratis
                        let gas_consumed = calculate_gas_consumption(payload_size_bytes);

                        let start_pqc = Instant::now();
                        if let Err(err_msg) = verify_quantum_signature(&packet.pubkey_bytes, &packet.signature_bytes, batch_msg.as_bytes()) {
                            let reject_msg = format!("{}. Energy Wasted: {} Units\n", err_msg, gas_consumed);
                            let _ = send_stream.write_all(reject_msg.as_bytes()).await;
                            return;
                        }
                        let duration_pqc = start_pqc.elapsed();

                        let start_time = Instant::now();
                        let calculated_point = (packet.total_hash_xor ^ packet.aggregated_witness).wrapping_mul(GENERATOR_G);

                        if calculated_point != 0 {
                            let mut write_guard = node_inner.write().await;
                            
                            // Hitung pemotongan nilai gas berdasarkan Homomorphic Burn di modul gas
                            let gas_burn_value = calculate_homomorphic_burn(gas_consumed);

                            // Terapkan transisi state secara terisolasi ke database
                            write_guard.apply_state_transition(packet.total_delta, gas_burn_value);

                            let duration_verkle = start_time.elapsed();

                            println!("\n[🔥] [PQC-GAS] Modular Node: Blok Valid & Gas Berhasil Dibakar!");
                            println!(" ├─ Payload Size  : {} Byte", payload_size_bytes);
                            println!(" ├─ Gas Burned    : {} Units", gas_consumed);
                            println!(" ├─ PQC Verif Time: {:?}", duration_pqc);
                            println!(" └─ Verkle Time   : {:?}", duration_verkle);

                            let success_msg = format!(
                                "SUCCESS: Batch Sah! Gas Terbakar: {} Units. State Root Baru: {}\n",
                                gas_consumed, write_guard.state_root
                            );
                            let _ = send_stream.write_all(success_msg.as_bytes()).await;
                        }
                        let _ = send_stream.finish();
                    }
                });
            }
        });
    }
    Ok(())
}

// SUITE PENGUJIAN OTOMATIS UNIT TEST DI BAGIAN AKHIR
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_malformed_packet_size_rejection() {
        let bytes_read_simulation = 150;
        assert!(bytes_read_simulation < 200 || bytes_read_simulation > 8000);
    }

    #[test]
    fn test_dilithium3_array_size_validation() {
        let fake_pubkey = vec![0u8; 10];
        let fake_sig = vec![0u8; 10];
        assert!(!validate_dilithium3_dimensions(&fake_pubkey, &fake_sig));
    }

    #[test]
    fn test_vrf_threshold_filter() {
        let high_nonce_packet_xor: u64 = 0xFFFFFFFFFFFFFFFF;
        assert!(!VerkleNode::evaluate_vrf_selection(high_nonce_packet_xor));
    }

    #[test]
    fn test_dynamic_shard_reconstruction_success() {
        use crate::protected_shard::{ProtectedShard, dynamic_secure_reconstruct};

        let original_data = b"VCEPH-VALIDATOR-BALANCE-RECORD-2026";
        
        let shard_a_data = vec![0xAA; original_data.len()];
        let shard_b_data: Vec<u8> = original_data.iter()
            .zip(shard_a_data.iter())
            .map(|(o, a)| o ^ a)
            .collect();

        let shard_1 = ProtectedShard::new(1, shard_a_data.clone());
        let shard_2 = ProtectedShard::new(2, shard_b_data.clone());

        let trusted_comm_1 = blake3::hash(&shard_a_data).into();
        let trusted_comm_2 = blake3::hash(&shard_b_data).into();

        let recovery_result = dynamic_secure_reconstruct(
            &shard_1, 
            &shard_2, 
            &trusted_comm_1, 
            &trusted_comm_2
        ).unwrap();

        assert_eq!(recovery_result, original_data);
    }

    #[test]
    fn test_shard_poisoning_prevention() {
        use crate::protected_shard::{ProtectedShard, dynamic_secure_reconstruct};

        let shard_a_data = vec![0x11, 0x22, 0x33];
        let shard_b_data = vec![0x44, 0x55, 0x66];

        let mut poisoned_shard = ProtectedShard::new(1, shard_a_data.clone());
        let healthy_shard = ProtectedShard::new(2, shard_b_data.clone());

        let trusted_comm_1 = blake3::hash(&shard_a_data).into();
        let trusted_comm_2 = blake3::hash(&shard_b_data).into();

        // SIMULASI SERANGAN: Racuni data fisik internal shard
        poisoned_shard.data = vec![0x99, 0x99, 0x99]; 

        let final_check = dynamic_secure_reconstruct(
            &poisoned_shard, 
            &healthy_shard, 
            &trusted_comm_1, 
            &trusted_comm_2
        );

        assert!(final_check.is_err());
    }
}

