#!/data/data/com.termux/files/usr/bin/bash

clear
echo "=================================================="
echo "    V-CEPH QUANTUM CORE - NODE CONTROLLER PANEL   "
echo "=================================================="
echo "1. Jalankan Validator Node (Mendengarkan WiFi & Publik - Port 8080)"
echo "2. Jalankan Uji Sterilisasi Suite (Cargo Test)"
echo "3. Jalankan Uji Performa Kriptografi Kisi (Cargo Bench)"
echo "4. Cek Alamat IP WiFi Lokal Node Ini"
echo "5. Keluar"
echo "=================================================="
read -p "Pilih operasi laboratorium [1-5]: " opsi

# Ambil direktori proyek aktif saat ini secara dinamis
PROYEK_DIR="$HOME/vceph-quantum-core"

case $opsi in
    1)
        echo "[*] Mendeteksi konfigurasi jaringan..."
        IP_LOKAL=$(ip route get 1.1.1.1 2>/dev/null | grep -oP 'src \K\S+')
        echo "[+] Node Anda dapat ditembak dari WiFi di alamat: ${IP_LOKAL:-0.0.0.0}:8080"
        echo "[*] Meluncurkan Validator Node Pasca-Kuantum..."
        cd "$PROYEK_DIR" && cargo run --release
        ;;
    2)
        echo "[*] Menjalankan Uji Sterilisasi Logika (5 Passed Suite)..."
        cd "$PROYEK_DIR" && cargo test
        ;;
    3)
        echo "[*] Menjalankan Uji Performa Kecepatan Nanosekon..."
        cd "$PROYEK_DIR" && cargo bench
        ;;
    4)
        echo "[*] Membaca kartu jaringan internal..."
        IP_LOKAL=$(ip route get 1.1.1.1 2>/dev/null | grep -oP 'src \K\S+')
        if [ -z "$IP_LOKAL" ]; then
            echo "[!] Gagal mendeteksi IP otomatis. Pastikan WiFi terhubung."
        else
            echo "=================================================="
            echo " WARGA JARINGAN WIFI (LAN) BISA MENEMBAK NODE DI:"
            echo " -> Address: $IP_LOKAL"
            echo " -> Port   : 8080"
            echo "=================================================="
        fi
        ;;
    5)
        echo "[*] Selesai. Keluar dari laboratorium."
        exit 0
        ;;
    *)
        echo "[!] Opsi tidak valid!"
        ;;
esac

