/// NantaraVM HTTP API Server — Real REST API
/// Server ini SUNGGUHAN berjalan di port 8080 dan mengendalikan QEMU secara nyata.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::qemu_engine::{QemuEngine, VmConfig};

fn read_body(stream: &mut TcpStream) -> String {
    let mut headers_buf = [0u8; 4096];
    let _ = stream.read(&mut headers_buf);
    let raw = String::from_utf8_lossy(&headers_buf);

    // Extract Content-Length dan body
    if let Some(body_start) = raw.find("\r\n\r\n") {
        raw[body_start + 4..].trim_matches('\0').to_string()
    } else {
        String::new()
    }
}

fn json_response(stream: &mut TcpStream, status: &str, body: &str) {
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nContent-Length: {}\r\n\r\n{}",
        status,
        body.len(),
        body
    );
    let _ = stream.write_all(response.as_bytes());
}

fn simple_json_parse(body: &str, key: &str) -> Option<String> {
    // Parser JSON sederhana: cari "key":"value" atau "key":number
    let search = format!("\"{}\"", key);
    if let Some(pos) = body.find(&search) {
        let after = &body[pos + search.len()..];
        let after = after.trim_start_matches(':').trim();
        if after.starts_with('"') {
            // String value
            let inner = &after[1..];
            if let Some(end) = inner.find('"') {
                return Some(inner[..end].to_string());
            }
        } else {
            // Number value
            let end = after.find(|c: char| !c.is_ascii_digit()).unwrap_or(after.len());
            return Some(after[..end].to_string());
        }
    }
    None
}

pub struct ApiServer {
    engine: Arc<QemuEngine>,
}

impl ApiServer {
    pub fn new() -> Self {
        ApiServer {
            engine: Arc::new(QemuEngine::new()),
        }
    }

    pub fn run(&self, port: u16) {
        let listener = match TcpListener::bind(format!("0.0.0.0:{}", port)) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("[NantaraVM API] Gagal bind port {}: {}", port, e);
                return;
            }
        };

        println!("[NantaraVM API] ✅ Server berjalan di http://127.0.0.1:{}", port);
        println!("[NantaraVM API] Endpoint tersedia:");
        println!("  GET  http://127.0.0.1:{}/api/v1/status", port);
        println!("  GET  http://127.0.0.1:{}/api/v1/vm/info", port);
        println!("  POST http://127.0.0.1:{}/api/v1/vm/start", port);
        println!("  POST http://127.0.0.1:{}/api/v1/vm/stop", port);
        println!("  POST http://127.0.0.1:{}/api/v1/qemu/check", port);

        let engine = Arc::clone(&self.engine);

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let engine = Arc::clone(&engine);
                    std::thread::spawn(move || {
                        Self::handle_request(&mut stream, &engine);
                    });
                }
                Err(e) => eprintln!("[NantaraVM API] Connection error: {}", e),
            }
        }
    }

    fn handle_request(stream: &mut TcpStream, engine: &QemuEngine) {
        let mut buf = [0u8; 8192];
        let n = stream.read(&mut buf).unwrap_or(0);
        if n == 0 { return; }

        let request = String::from_utf8_lossy(&buf[..n]);
        let first_line = request.lines().next().unwrap_or("");

        // CORS preflight
        if first_line.starts_with("OPTIONS") {
            let resp = "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nContent-Length: 0\r\n\r\n";
            let _ = stream.write_all(resp.as_bytes());
            return;
        }

        // Extract body dari request
        let body = if let Some(body_start) = request.find("\r\n\r\n") {
            request[body_start + 4..].trim_matches('\0').to_string()
        } else {
            String::new()
        };

        // Route requests
        if first_line.contains("GET /api/v1/status") {
            let qemu_ok = engine.is_qemu_available();
            let body = format!(
                "{{\"status\":\"ok\",\"engine\":\"NantaraVM v0.2 QEMU Backend\",\"qemu_available\":{},\"port\":8080}}",
                qemu_ok
            );
            json_response(stream, "200 OK", &body);

        } else if first_line.contains("GET /api/v1/vm/info") {
            let vms = engine.list_running_vms();
            let vm_list: Vec<String> = vms.iter()
                .map(|(name, pid)| format!("{{\"name\":\"{}\",\"pid\":{}}}", name, pid))
                .collect();
            let body = format!("{{\"running_vms\":[{}],\"count\":{}}}", vm_list.join(","), vms.len());
            json_response(stream, "200 OK", &body);

        } else if first_line.contains("POST /api/v1/vm/start") {
            // Parse body JSON: {"name":"...", "iso":"...", "ram":4096, "vcpu":4}
            let vm_name = simple_json_parse(&body, "name")
                .unwrap_or_else(|| "nantara-vm-01".to_string());
            let iso_path = simple_json_parse(&body, "iso");
            let drive_path = simple_json_parse(&body, "drive");
            let ram_mb = simple_json_parse(&body, "ram")
                .and_then(|s| s.parse().ok())
                .unwrap_or(2048u32);
            let vcpu = simple_json_parse(&body, "vcpu")
                .and_then(|s| s.parse().ok())
                .unwrap_or(2u32);

            let config = VmConfig { name: vm_name.clone(), iso_path, drive_path, ram_mb, vcpu };

            match engine.start_vm(config) {
                Ok(msg) => {
                    let resp = format!("{{\"status\":\"started\",\"message\":\"{}\",\"vm\":\"{}\"}}", msg, vm_name);
                    json_response(stream, "200 OK", &resp);
                }
                Err(e) => {
                    let resp = format!("{{\"status\":\"error\",\"message\":\"{}\"}}", e.replace('"', "'"));
                    json_response(stream, "500 Internal Server Error", &resp);
                }
            }

        } else if first_line.contains("POST /api/v1/vm/stop") {
            let vm_name = simple_json_parse(&body, "name")
                .unwrap_or_else(|| "nantara-vm-01".to_string());

            match engine.stop_vm(&vm_name) {
                Ok(msg) => {
                    let resp = format!("{{\"status\":\"stopped\",\"message\":\"{}\"}}", msg);
                    json_response(stream, "200 OK", &resp);
                }
                Err(e) => {
                    let resp = format!("{{\"status\":\"error\",\"message\":\"{}\"}}", e);
                    json_response(stream, "404 Not Found", &resp);
                }
            }

        } else if first_line.contains("GET /api/v1/qemu/check") {
            let available = engine.is_qemu_available();
            let msg = if available {
                "QEMU tersedia dan siap digunakan!"
            } else {
                "QEMU belum terinstall. Download dari https://qemu.weilnetz.de/w64/"
            };
            let body = format!("{{\"qemu_available\":{},\"message\":\"{}\"}}", available, msg);
            json_response(stream, "200 OK", &body);

        } else {
            let body = "{\"status\":\"error\",\"message\":\"Endpoint tidak ditemukan\"}";
            json_response(stream, "404 Not Found", body);
        }
    }
}
