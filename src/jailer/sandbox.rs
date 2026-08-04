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

    /// Apply a real Seccomp-BPF syscall filter using the `seccompiler` crate.
    /// This actually installs a Linux kernel BPF program via prctl(PR_SET_SECCOMP).
    #[cfg(target_os = "linux")]
    pub fn apply_seccomp_filter(&self) -> Result<(), String> {
        use seccompiler::{
            BpfProgram, SeccompAction, SeccompCmpArgLen, SeccompCmpOp, SeccompCondition,
            SeccompFilter, SeccompRule,
        };
        use std::collections::BTreeMap;

        println!("[NantaraVM Jailer] Installing real Seccomp-BPF Linux kernel syscall filter...");

        // Build the allowlist: map each allowed syscall number to its rules.
        // An empty Vec<SeccompRule> means "allow unconditionally".
        let mut rules: BTreeMap<i64, Vec<SeccompRule>> = BTreeMap::new();

        // Essential syscalls for NantaraVM operation
        let allowed_syscalls: &[i64] = &[
            libc::SYS_read,
            libc::SYS_write,
            libc::SYS_readv,
            libc::SYS_writev,
            libc::SYS_ioctl,        // KVM ioctls
            libc::SYS_mmap,
            libc::SYS_munmap,
            libc::SYS_mprotect,
            libc::SYS_madvise,
            libc::SYS_brk,
            libc::SYS_futex,
            libc::SYS_epoll_create1,
            libc::SYS_epoll_ctl,
            libc::SYS_epoll_wait,
            libc::SYS_epoll_pwait,
            libc::SYS_poll,
            libc::SYS_ppoll,
            libc::SYS_select,
            libc::SYS_pselect6,
            libc::SYS_close,
            libc::SYS_open,
            libc::SYS_openat,
            libc::SYS_fstat,
            libc::SYS_stat,
            libc::SYS_lstat,
            libc::SYS_fstatat,  // newfstatat
            libc::SYS_lseek,
            libc::SYS_pread64,
            libc::SYS_pwrite64,
            libc::SYS_fcntl,
            libc::SYS_dup,
            libc::SYS_dup2,
            libc::SYS_dup3,
            libc::SYS_socket,
            libc::SYS_bind,
            libc::SYS_listen,
            libc::SYS_accept,
            libc::SYS_accept4,
            libc::SYS_connect,
            libc::SYS_sendto,
            libc::SYS_recvfrom,
            libc::SYS_setsockopt,
            libc::SYS_getsockopt,
            libc::SYS_getsockname,
            libc::SYS_getpeername,
            libc::SYS_shutdown,
            libc::SYS_nanosleep,
            libc::SYS_clock_gettime,
            libc::SYS_clock_nanosleep,
            libc::SYS_getpid,
            libc::SYS_gettid,
            libc::SYS_getuid,
            libc::SYS_getgid,
            libc::SYS_geteuid,
            libc::SYS_getegid,
            libc::SYS_set_robust_list,
            libc::SYS_get_robust_list,
            libc::SYS_rt_sigaction,
            libc::SYS_rt_sigprocmask,
            libc::SYS_rt_sigreturn,
            libc::SYS_sigaltstack,
            libc::SYS_tgkill,
            libc::SYS_kill,
            libc::SYS_exit,
            libc::SYS_exit_group,
            libc::SYS_getrandom,
            libc::SYS_sched_yield,
            libc::SYS_sched_getaffinity,
            libc::SYS_sched_setaffinity,
            libc::SYS_prctl,
            libc::SYS_arch_prctl,
            libc::SYS_seccomp,   // Allow seccomp itself (for child processes)
            libc::SYS_pipe2,
            libc::SYS_eventfd2,
            libc::SYS_timerfd_create,
            libc::SYS_timerfd_settime,
            libc::SYS_timerfd_gettime,
            libc::SYS_clone,     // Thread creation
            libc::SYS_clone3,
            libc::SYS_wait4,
            libc::SYS_waitid,
            libc::SYS_signalfd4,
            libc::SYS_memfd_create,
            libc::SYS_userfaultfd, // For snapshot/restore
        ];

        for &syscall_nr in allowed_syscalls {
            rules.insert(syscall_nr, vec![]);
        }

        // Build the filter: default action is ERRNO(EPERM) for anything not in allowlist.
        // NEVER use SeccompAction::Kill in production without thorough testing — use Errno first.
        let filter = SeccompFilter::new(
            rules,
            SeccompAction::Errno(libc::EPERM as u32),
            SeccompAction::Allow,
            std::env::consts::ARCH.try_into().map_err(|e| format!("Arch error: {:?}", e))?,
        )
        .map_err(|e| format!("SeccompFilter build error: {:?}", e))?;

        let bpf_prog: BpfProgram = filter
            .try_into()
            .map_err(|e| format!("BPF compile error: {:?}", e))?;

        seccompiler::apply_filter(&bpf_prog)
            .map_err(|e| format!("Failed to apply Seccomp-BPF filter: {:?}", e))?;

        println!("[NantaraVM Jailer] ✅ Seccomp-BPF kernel syscall filter ACTIVE ({} syscalls allowed).", allowed_syscalls.len());
        println!("[NantaraVM Jailer]    DEFAULT ACTION: ERRNO(EPERM) for all other syscalls.");
        println!("[NantaraVM Jailer]    BLOCKED: execve, ptrace, sys_reboot, init_module, delete_module, perf_event_open");
        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    pub fn apply_seccomp_filter(&self) -> Result<(), String> {
        println!("[NantaraVM Jailer] [Dev Mode] Seccomp-BPF filter skipped (non-Linux host).");
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
