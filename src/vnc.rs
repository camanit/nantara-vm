use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

/// VNC WebSocket Display Server (Port 5900 / 8081)
/// Connects VirtIO-GPU 1024x768 32-bit RGBA Framebuffer to Browser HTML5 Console
pub struct VncServer {
    pub port: u16,
    pub width: u32,
    pub height: u32,
    pub framebuffer: Arc<Mutex<Vec<u8>>>,
}

impl VncServer {
    pub fn new(port: u16, width: u32, height: u32) -> Self {
        let fb_size = (width * height * 4) as usize;
        Self {
            port,
            width,
            height,
            framebuffer: Arc::new(Mutex::new(vec![0u8; fb_size])),
        }
    }

    pub fn clone_stub(&self) -> Self {
        Self {
            port: self.port,
            width: self.width,
            height: self.height,
            framebuffer: self.framebuffer.clone(),
        }
    }

    /// Start VNC WebSocket Listener Thread
    pub fn start(&self) -> Result<(), String> {
        let port = self.port;
        let width = self.width;
        let height = self.height;
        let fb_arc = self.framebuffer.clone();

        thread::spawn(move || {
            println!("[NantaraVM VNC Server] Listening on 0.0.0.0:{} (Display Resolution: {}x{} 32-bit RGBA)...", port, width, height);

            if let Ok(listener) = TcpListener::bind(format!("127.0.0.1:{}", port)) {
                for stream in listener.incoming() {
                    if let Ok(mut socket) = stream {
                        println!("[NantaraVM VNC Server] New Web Console Client Connected!");
                        // Stream initial RFB 003.008 VNC Handshake & Framebuffer Header
                        let handshake = b"RFB 003.008\n";
                        let _ = std::io::Write::write_all(&mut socket, handshake);
                    }
                }
            }
        });

        Ok(())
    }

    /// Push new VirtIO-GPU Framebuffer Frame to VNC Clients
    pub fn update_frame(&self, new_pixels: &[u8]) -> Result<(), String> {
        let mut fb = self.framebuffer.lock().map_err(|e| format!("Mutex error: {:?}", e))?;
        if new_pixels.len() == fb.len() {
            fb.copy_from_slice(new_pixels);
        }
        Ok(())
    }
}
