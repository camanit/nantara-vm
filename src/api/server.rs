use std::path::PathBuf;

pub struct ApiServer {
    pub socket_path: PathBuf,
    pub running: bool,
}

impl ApiServer {
    pub fn new(socket_path: PathBuf) -> Self {
        Self {
            socket_path,
            running: false,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        println!("[NantaraVM REST API] Starting management API server on {:?}", self.socket_path);
        println!("[NantaraVM REST API] Available Endpoints: GET /api/v1/vm/info, POST /api/v1/vm/pause, POST /api/v1/vm/resume, POST /api/v1/vm/snapshot");
        self.running = true;
        Ok(())
    }
}
