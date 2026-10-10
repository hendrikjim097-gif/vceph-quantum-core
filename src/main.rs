use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::Instant;
use serde::{Serialize, Deserialize};

use pqcrypto_dilithium::dilithium3::{verify_detached_signature, DetachedSignature as DilithiumSig, PublicKey as DilithiumPub};
use pqcrypto_traits::sign::PublicKey as TraitPub;
use pqcrypto_traits::sign::DetachedSignature as TraitSig;

const GENERATOR_G: u64 = 0x54321AFB;
const VRF_THRESHOLD: u64 = 0x3FFFFFFFFFFFFFFF; 

// Base Gas per Byte dari muatan tanda tangan kisi
const GAS_PER_BYTE_RATE: u64 = 1; 

#[derive(Serialize, Deserialize, Debug)]
struct QuantumPacket {
    total_hash_xor: u64,
    aggregated_witness: u64,
    total_delta: i64,
    pubkey_bytes: Vec<u8>,
    signature_bytes: Vec<u8>,
}

struct VerkleNode {
    state_root: u64,
    db: sled::Db,
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
    println!("[🔥] Mengaktifkan V-CEPH Quantum-Resistant Server + Gas Metering...");
    let _ = rustls::crypto::ring::default_provider().install_default();

    let db = sled::open("stateless_node_db")?;
    let initial_root = 13361873044695950534; 

    let node_shared = Arc::new(RwLock::new(VerkleNode { state_root: initial_root, db }));
    let addr: SocketAddr = "127.0.0.1:8080".parse()?;
    let server_config = configure_server();
    let endpoint = quinn::Endpoint::server(server_config, addr)?;
    
    println!("[+] Server UDP/QUIC Mengunci Pipa di Alamat: 127.0.0.1 8080\n");

    while let Some(connecting) = endpoint.accept().await {
        let node = Arc::clone(&node_shared);
        tokio::spawn(async move {
            let connection = match connecting.await { Ok(conn) => conn, Err(_) => return };
            while let Ok((mut send_stream, mut recv_stream)) = connection.accept_bi().await {
                let node_inner = Arc::clone(&node);
                tokio::spawn(async move {
                    let mut buffer = vec![0u8; 8192];
                    if let Ok(Some(bytes_read)) = recv_stream.read(&mut buffer).await {
                        
                        // Kalkulasi awal ukuran muatan biner fisik yang masuk (Thermodynamic Ingestion)
                        let payload_size_bytes = bytes_read as u64;

                        let packet: QuantumPacket = match bincode::deserialize(&buffer[..bytes_read]) {
                            Ok(p) => p,
                            Err(_) => { let _ = send_stream.write_all(b"REJECTED: Deserialize Error\n").await; return; }
                        };

                        let batch_msg = format!("{}:{}:{}", packet.total_hash_xor, packet.aggregated_witness, packet.total_delta);

                        let pqc_pubkey = match DilithiumPub::from_bytes(&packet.pubkey_bytes) {
                            Ok(k) => k,
                            Err(_) => { let _ = send_stream.write_all(b"REJECTED: Invalid PubKey\n").await; return; }
                        };
                        let pqc_sig = match DilithiumSig::from_bytes(&packet.signature_bytes) {
                            Ok(s) => s,
                            Err(_) => { let _ = send_stream.write_all(b"REJECTED: Invalid Signature\n").await; return; }
                        };

                        let start_pqc = Instant::now();
                        if verify_detached_signature(&pqc_sig, batch_msg.as_bytes(), &pqc_pubkey).is_err() {
                            let _ = send_stream.write_all(b"REJECTED: PQC Auth Failed\n").await;
                            return;
                        }
                        let duration_pqc = start_pqc.elapsed();

                        // --- INTERVENTIE MODUL GAS METERING (BME ENGINE) ---
                        // Menghitung konsumsi energi komputasi berdasarkan byte fisik paket
                        let gas_consumed = payload_size_bytes * GAS_PER_BYTE_RATE;

                        let start_time = Instant::now();
                        let calculated_point = (packet.total_hash_xor ^ packet.aggregated_witness).wrapping_mul(GENERATOR_G);

                        if calculated_point != 0 {
                            let mut write_guard = node_inner.write().await;
                            
                            if packet.total_delta >= 0 {
                                write_guard.state_root = write_guard.state_root.wrapping_add((packet.total_delta as u64).wrapping_mul(GENERATOR_G));
                            } else {
                                write_guard.state_root = write_guard.state_root.wrapping_sub((packet.total_delta.abs() as u64).wrapping_mul(GENERATOR_G));
                            }
                            
                            // Suntikkan pembakaran Gas ke State Root secara matematis (Homomorphic Burn)
                            write_guard.state_root = write_guard.state_root.wrapping_sub(gas_consumed.wrapping_mul(GENERATOR_G));

                            let _ = write_guard.db.insert(b"current_state_root", &write_guard.state_root.to_be_bytes());
                            let _ = write_guard.db.flush();

                            let duration_verkle = start_time.elapsed();
                            
                            println!("\n[🔥] [PQC-GAS] Blok Lolos Sensor Kisi & Validasi Termodinamika!");
                            println!(" ├─ Payload Size  : {} Byte", payload_size_bytes);
                            println!(" ├─ Gas Burned    : {} Gwei/Units", gas_consumed);
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

