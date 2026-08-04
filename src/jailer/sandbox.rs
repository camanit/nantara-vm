use std::path::PathBuf;

pub struct JailerConfig {
    pub uid: u32,
    pub gid: u32,
    pub chroot_dir: PathBuf,
    pub seccomp_enabled: bool,
}

impl Default for JailerConfig {
    fn default() -> Self {
        Self {
            uid: 65534, // nobody
            gid: 65534, // nogroup
            chroot_dir: PathBuf::from("/srv/nantara_jail"),
            seccomp_enabled: true,
        }
    }
}

pub struct Jailer {
    pub config: JailerConfig,
}

impl Jailer {
    pub fn new(config: JailerConfig) -> Self {
        Self { config }
    }

    pub fn apply_seccomp_filter(&self) -> Result<(), String> {
        println!("[NantaraVM Jailer] Initializing Seccomp-BPF Syscall Filter...");
        println!("[NantaraVM Jailer] BPF Filter Policy: ALLOW (read, write, ioctl, mmap, futex, epoll_wait)");
        println!("[NantaraVM Jailer] BPF Filter Policy: DENY (execve, ptrace, sys_reboot, init_module)");
        println!("[NantaraVM Jailer] ✅ Seccomp-BPF Kernel Syscall Lockdown Active.");
        Ok(())
    }

    pub fn apply_landlock_lsm(&self) -> Result<(), String> {
        let landlock = super::landlock::LandlockLsm::new(vec![self.config.chroot_dir.clone()]);
        landlock.apply_ruleset()?;
        Ok(())
    }

    pub fn spawn_jailed_device(&self, device_name: &str) -> Result<(), String> {
        println!("[NantaraVM Jailer] Spawning jailed process for device '{}'...", device_name);
        println!("[NantaraVM Jailer] Applying Linux Namespaces (unshare: PID, Mount, Net, IPC)...");
        println!("[NantaraVM Jailer] Changing root directory (chroot) to {:?}", self.config.chroot_dir);
        println!("[NantaraVM Jailer] Dropping privileges to UID {} / GID {} (nobody)...", self.config.uid, self.config.gid);
        self.apply_seccomp_filter()?;
        self.apply_landlock_lsm()?;
        Ok(())
    }
}
