use std::io::{self, Write};

fn main() {
    println!("====================================================");
    println!(" 🔑 NantaraVM Enterprise License Generator Utility");
    println!("====================================================");

    print!("Masukkan Nama Organisasi / Klien (contoh: Bank Mandiri): ");
    io::stdout().flush().unwrap();
    let mut org = String::new();
    io::stdin().read_line(&mut org).unwrap();
    let org = org.trim();

    if org.is_empty() {
        println!("[Error] Nama organisasi tidak boleh kosong.");
        return;
    }

    let rand_hex = format!("{:08X}", rand_pseudo());
    let clean_org = org.replace(' ', "-").to_uppercase();
    let license_key = format!("NANTARA-PRO-2026-{}-{}", clean_org, rand_hex);

    println!("\n✅ License Key Enterprise Pro Berhasil Diterbitkan!");
    println!("----------------------------------------------------");
    println!(" Klien       : {}", org);
    println!(" License Key : {}", license_key);
    println!(" Masa Aktif  : 1 Tahun (365 Hari)");
    println!("----------------------------------------------------");
    println!("\n📧 Teks Surat / WA untuk dikirimkan ke Klien:");
    println!("----------------------------------------------------");
    println!("Yth. Tim IT {},", org);
    println!("Terima kasih telah berlangganan NantaraVM Enterprise Pro.");
    println!("Berikut adalah License Key resmi Anda:\n");
    println!("  LICENSE KEY : {}", license_key);
    println!("\nInstruksi Aktivasi di Server Linux:");
    println!("  export NANTARA_PRO_LICENSE=\"{}\"", license_key);
    println!("  ./nantara-vm\n");
    println!("Hubungi SLA Support 24/7 via WA: +62 812-6000-6666");
    println!("====================================================");
}

fn rand_pseudo() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
    nanos ^ 0x8F3A2B10
}
