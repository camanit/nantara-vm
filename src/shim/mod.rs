pub mod v2;

pub use self::v2::ContainerdShimV2;

pub struct ContainerdShim;

impl ContainerdShim {
    pub fn new() -> Self {
        Self
    }

    pub fn init(&self) -> Result<(), String> {
        println!("[NantaraVM] Containerd-shim-v2 interface registered.");
        Ok(())
    }

    pub fn start_shim_listener(&self) -> Result<(), String> {
        let shim_v2 = ContainerdShimV2::new("/run/containerd/nantara-shim.sock");
        shim_v2.start_shim_service()?;
        Ok(())
    }

    pub fn spawn_oci_container(&self, container_id: &str, image_ref: &str) -> Result<(), String> {
        println!("[NantaraVM containerd-shim-v2] Spawning OCI Container {} from image {}", container_id, image_ref);
        Ok(())
    }
}
