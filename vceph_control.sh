#!/data/data/com.termux/files/usr/bin/bash

clear
echo "=================================================="
echo "    V-CEPH QUANTUM CORE - NODE CONTROLLER PANEL   "
echo "=================================================="
echo "1. Nyalakan Storage Shard Node (Port 8090)"
echo "2. Nyalakan Validator Node + Gas Engine (Port 8080)"
echo "3. Tembak Transaksi Klien (Simulasi)"
echo "4. Jalankan Uji Performa (Cargo Bench)"
echo "5. Keluar"
echo "=================================================="
read -p "Pilih operasi laboratorium [1-5]: " opsi

case $opsi in
    1)
        echo "[*] Meluncurkan Storage Shard Node..."
        cd ~/vceph_storage && cargo run --release
        ;;
    2)
        echo "[*] Meluncurkan Validator Node..."
        cd ~/rust_stateless && cargo run --release
        ;;
    3)
        echo "[*] Meluncurkan Transaksi Klien..."
        cd ~/vceph_client && cargo run --release
        ;;
    4)
        echo "[*] Menjalankan Uji Performa Nanosekon..."
        cd ~/rust_stateless && cargo bench
        ;;
    5)
        echo "[*] Selesai. Keluar dari laboratorium."
        exit 0
        ;;
    *)
        echo "[!] Opsi tidak valid!"
        ;;
esac
