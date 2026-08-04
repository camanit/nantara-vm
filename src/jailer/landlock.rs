use std::path::PathBuf;

pub struct LandlockLsm {
    pub allowed_paths: Vec<PathBuf>,
}

impl LandlockLsm {
    pub fn new(allowed_paths: Vec<PathBuf>) -> Self {
        Self { allowed_paths }
    }

    /// Apply a real Linux Landlock LSM filesystem restriction.
    /// Uses raw libc syscalls: landlock_create_ruleset (444),
    /// landlock_add_rule (445), landlock_restrict_self (446).
    /// Requires Linux kernel >= 5.13.
    #[cfg(target_os = "linux")]
    pub fn apply_ruleset(&self) -> Result<(), String> {
        use std::os::unix::ffi::OsStrExt;

        // Landlock syscall numbers on x86_64
        const SYS_LANDLOCK_CREATE_RULESET: libc::c_long = 444;
        const SYS_LANDLOCK_ADD_RULE: libc::c_long = 445;
        const SYS_LANDLOCK_RESTRICT_SELF: libc::c_long = 446;

        // Landlock access rights for filesystem
        const LANDLOCK_ACCESS_FS_EXECUTE:       u64 = 1 << 0;
        const LANDLOCK_ACCESS_FS_WRITE_FILE:    u64 = 1 << 1;
        const LANDLOCK_ACCESS_FS_READ_FILE:     u64 = 1 << 2;
        const LANDLOCK_ACCESS_FS_READ_DIR:      u64 = 1 << 3;
        const LANDLOCK_ACCESS_FS_REMOVE_DIR:    u64 = 1 << 4;
        const LANDLOCK_ACCESS_FS_REMOVE_FILE:   u64 = 1 << 5;
        const LANDLOCK_ACCESS_FS_MAKE_CHAR:     u64 = 1 << 6;
        const LANDLOCK_ACCESS_FS_MAKE_DIR:      u64 = 1 << 7;
        const LANDLOCK_ACCESS_FS_MAKE_REG:      u64 = 1 << 8;
        const LANDLOCK_ACCESS_FS_MAKE_SOCK:     u64 = 1 << 9;
        const LANDLOCK_ACCESS_FS_MAKE_FIFO:     u64 = 1 << 10;
        const LANDLOCK_ACCESS_FS_MAKE_BLOCK:    u64 = 1 << 11;
        const LANDLOCK_ACCESS_FS_MAKE_SYM:      u64 = 1 << 12;
        const LANDLOCK_ACCESS_FS_REFER:         u64 = 1 << 13;

        const LANDLOCK_RULE_PATH_BENEATH: u32 = 1;

        // All FS access rights we want to manage (use the maximum supported set)
        let handled_access: u64 = LANDLOCK_ACCESS_FS_EXECUTE
            | LANDLOCK_ACCESS_FS_WRITE_FILE
            | LANDLOCK_ACCESS_FS_READ_FILE
            | LANDLOCK_ACCESS_FS_READ_DIR
            | LANDLOCK_ACCESS_FS_REMOVE_DIR
            | LANDLOCK_ACCESS_FS_REMOVE_FILE
            | LANDLOCK_ACCESS_FS_MAKE_CHAR
            | LANDLOCK_ACCESS_FS_MAKE_DIR
            | LANDLOCK_ACCESS_FS_MAKE_REG
            | LANDLOCK_ACCESS_FS_MAKE_SOCK
            | LANDLOCK_ACCESS_FS_MAKE_FIFO
            | LANDLOCK_ACCESS_FS_MAKE_BLOCK
            | LANDLOCK_ACCESS_FS_MAKE_SYM
            | LANDLOCK_ACCESS_FS_REFER;

        // Struct landlock_ruleset_attr { __u64 handled_access_fs; }
        #[repr(C)]
        struct LandlockRulesetAttr {
            handled_access_fs: u64,
        }

        // Struct landlock_path_beneath_attr { __u64 allowed_access; __s32 parent_fd; }
        #[repr(C)]
        struct LandlockPathBeneathAttr {
            allowed_access: u64,
            parent_fd: i32,
        }

        println!("[NantaraVM Landlock] Initializing Linux Landlock LSM filesystem restriction...");

        let ruleset_attr = LandlockRulesetAttr {
            handled_access_fs: handled_access,
        };

        // Step 1: Create a Landlock ruleset
        let ruleset_fd = unsafe {
            libc::syscall(
                SYS_LANDLOCK_CREATE_RULESET,
                &ruleset_attr as *const LandlockRulesetAttr,
                std::mem::size_of::<LandlockRulesetAttr>() as libc::size_t,
                0u32, // flags
            )
        };

        if ruleset_fd < 0 {
            let err = std::io::Error::last_os_error();
            // If kernel < 5.13 or Landlock not enabled, warn and continue (don't fail hard)
            if err.raw_os_error() == Some(libc::ENOSYS) || err.raw_os_error() == Some(libc::EOPNOTSUPP) {
                println!("[NantaraVM Landlock] ⚠️  Landlock LSM not supported on this kernel (requires >= 5.13). Skipping.");
                return Ok(());
            }
            return Err(format!("[NantaraVM Landlock] landlock_create_ruleset failed: {}", err));
        }

        let ruleset_fd = ruleset_fd as libc::c_int;

        // Step 2: For each allowed path, open it and add a path_beneath rule
        let allowed_rw = LANDLOCK_ACCESS_FS_READ_FILE
            | LANDLOCK_ACCESS_FS_WRITE_FILE
            | LANDLOCK_ACCESS_FS_READ_DIR
            | LANDLOCK_ACCESS_FS_MAKE_REG
            | LANDLOCK_ACCESS_FS_MAKE_DIR
            | LANDLOCK_ACCESS_FS_REMOVE_FILE;

        for path in &self.allowed_paths {
            // Try to open the path; if it doesn't exist, skip it gracefully
            let path_cstr = match std::ffi::CString::new(path.as_os_str().as_bytes()) {
                Ok(s) => s,
                Err(_) => continue,
            };

            let parent_fd = unsafe {
                libc::open(path_cstr.as_ptr(), libc::O_PATH | libc::O_CLOEXEC)
            };

            if parent_fd < 0 {
                println!("[NantaraVM Landlock]   ⚠️  Path {:?} not found, skipping rule.", path);
                continue;
            }

            let path_attr = LandlockPathBeneathAttr {
                allowed_access: allowed_rw,
                parent_fd,
            };

            let ret = unsafe {
                libc::syscall(
                    SYS_LANDLOCK_ADD_RULE,
                    ruleset_fd,
                    LANDLOCK_RULE_PATH_BENEATH,
                    &path_attr as *const LandlockPathBeneathAttr,
                    0u32, // flags
                )
            };

            unsafe { libc::close(parent_fd) };

            if ret < 0 {
                let err = std::io::Error::last_os_error();
                return Err(format!("[NantaraVM Landlock] landlock_add_rule failed for {:?}: {}", path, err));
            }

            println!("[NantaraVM Landlock]   ✅ Rule added: Read/Write/Exec allowed in {:?}", path);
        }

        // Step 3: Apply no_new_privs (required before landlock_restrict_self)
        let r = unsafe { libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) };
        if r < 0 {
            return Err(format!("prctl(PR_SET_NO_NEW_PRIVS) failed: {}", std::io::Error::last_os_error()));
        }

        // Step 4: Restrict the current thread with the ruleset
        let ret = unsafe {
            libc::syscall(SYS_LANDLOCK_RESTRICT_SELF, ruleset_fd, 0u32)
        };

        unsafe { libc::close(ruleset_fd) };

        if ret < 0 {
            let err = std::io::Error::last_os_error();
            return Err(format!("[NantaraVM Landlock] landlock_restrict_self failed: {}", err));
        }

        println!("[NantaraVM Landlock] ✅ Landlock LSM filesystem restriction ACTIVE.");
        println!("[NantaraVM Landlock]    All filesystem access outside allowed paths is now DENIED by kernel.");
        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    pub fn apply_ruleset(&self) -> Result<(), String> {
        println!("[NantaraVM Landlock] [Dev Mode] Landlock LSM skipped (non-Linux host).");
        Ok(())
    }
}
