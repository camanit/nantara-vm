/// NantaraVM v0.2 — Real QEMU Engine Entry Point
/// Menjalankan HTTP server nyata yang mengendalikan QEMU untuk boot ISO Windows/Linux.

mod qemu_engine;
mod http_server;

use http_server::ApiServer;

fn main() {
    println!("====================================================");
    println!(" 🚀 NantaraVM v0.2 - Real QEMU Engine (NKRI 2026)");
    println!("====================================================");
    println!();

    let server = ApiServer::new();

    // Cek QEMU
    println!("[NantaraVM] Memeriksa QEMU...");
    let engine = qemu_engine::QemuEngine::new();
    if engine.is_qemu_available() {
        println!("[NantaraVM] ✅ QEMU ditemukan dan siap digunakan!");
    } else {
        println!("[NantaraVM] ⚠️  QEMU belum terinstall.");
        println!("[NantaraVM] 👉 Download QEMU dari: https://qemu.weilnetz.de/w64/");
        println!("[NantaraVM] ℹ️  Server tetap berjalan. Install QEMU lalu restart.");
    }

    println!();
    println!("[NantaraVM] Memulai HTTP API Server di port 8080...");
    println!("[NantaraVM] Dashboard: buka web/dashboard.html di browser");
    println!("[NantaraVM] API Docs:");
    println!("  GET  http://127.0.0.1:8080/api/v1/status");
    println!("  POST http://127.0.0.1:8080/api/v1/vm/start  body: {{\"name\":\"vm1\",\"iso\":\"C:/win10.iso\",\"ram\":4096,\"vcpu\":4}}");
    println!("  POST http://127.0.0.1:8080/api/v1/vm/stop   body: {{\"name\":\"vm1\"}}");
    println!("  GET  http://127.0.0.1:8080/api/v1/vm/info");
    println!("  GET  http://127.0.0.1:8080/api/v1/qemu/check");
    println!();

    // Jalankan server (blocking)
    server.run(8080);
}
