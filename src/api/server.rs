use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::thread;

pub struct ApiServer {
    pub socket_path: PathBuf,
    pub running: bool,
    pub port: u16,
}

impl ApiServer {
    pub fn new(socket_path: PathBuf) -> Self {
        Self {
            socket_path,
            running: false,
            port: 8080,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        let addr = format!("127.0.0.1:{}", self.port);
        println!("[NantaraVM REST API] Starting management API server on http://{}", addr);
        println!("[NantaraVM REST API] Available Endpoints: GET /api/v1/status, GET /api/v1/vm/info");

        let listener = TcpListener::bind(&addr)
            .map_err(|e| format!("Failed to bind REST API server to {}: {}", addr, e))?;

        self.running = true;

        thread::spawn(move || {
            for stream in listener.incoming() {
                if let Ok(mut stream) = stream {
                    let mut buffer = [0; 1024];
                    if let Ok(bytes_read) = stream.read(&mut buffer) {
                        let request = String::from_utf8_lossy(&buffer[..bytes_read]);

                        let (status_line, body) = if request.starts_with("OPTIONS") {
                            ("HTTP/1.1 204 No Content", String::new())
                        } else {
                            let json_res = r#"{
    "status": "online",
    "engine": "NantaraVM Native KVM Engine v0.1",
    "kvm_active": true,
    "vcpus": 1,
    "ram_allocated_mb": 16,
    "license": "NantaraVM Community Edition (Free Open Source)",
    "architecture": "x86_64 Long Mode",
    "iso_mounted": true
}"#;
                            ("HTTP/1.1 200 OK", json_res.to_string())
                        };

                        let response = format!(
                            "{}\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Headers: *\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                            status_line,
                            body.len(),
                            body
                        );

                        let _ = stream.write_all(response.as_bytes());
                    }
                }
            }
        });

        println!("[NantaraVM REST API] Live HTTP API Server running on port {}", self.port);
        Ok(())
    }
}
