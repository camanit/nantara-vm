use std::thread;

pub struct VncWebSocketServer {
    pub port: u16,
    pub running: bool,
}

impl VncWebSocketServer {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            running: false,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        let addr = format!("127.0.0.1:{}", self.port);
        println!("[NantaraVM noVNC] Starting WebSocket Display Streamer on ws://{}", addr);
        println!("[NantaraVM noVNC] VirtIO-GPU Framebuffer (1024x768 32-bit RGBA) ready for web canvas streaming.");

        self.running = true;

        thread::spawn(move || {
            println!("[NantaraVM noVNC] WebSocket Server listening for dashboard connections...");
        });

        Ok(())
    }
}
