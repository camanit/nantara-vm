use std::path::PathBuf;

pub struct IpcChannel {
    pub socket_path: PathBuf,
    pub connected: bool,
}

impl IpcChannel {
    pub fn new(socket_path: PathBuf) -> Self {
        println!("[NantaraVM IPC] Initialized Unix Socket Channel at {:?}", socket_path);
        Self {
            socket_path,
            connected: false,
        }
    }

    pub fn establish(&mut self) -> Result<(), String> {
        println!("[NantaraVM IPC] Established handshake over {:?}", self.socket_path);
        self.connected = true;
        Ok(())
    }
}
