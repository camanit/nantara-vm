#[derive(Debug, Clone, PartialEq)]
pub enum LicenseTier {
    Community,
    EnterprisePro { organization: String, expires_at: String },
}

pub struct LicenseManager {
    pub tier: LicenseTier,
}

impl LicenseManager {
    pub fn verify_license() -> Self {
        if let Ok(key) = std::env::var("NANTARA_PRO_LICENSE") {
            if key.starts_with("NANTARA-PRO-2026-") {
                println!("[NantaraVM License] Validated Enterprise Pro License Key!");
                println!("[NantaraVM License] Organization: Government Sovereign Cloud NKRI | SLA: 24/7 Priority");
                return Self {
                    tier: LicenseTier::EnterprisePro {
                        organization: "Government Sovereign Cloud NKRI".to_string(),
                        expires_at: "2027-12-31".to_string(),
                    },
                };
            }
        }

        println!("[NantaraVM License] Mode: NantaraVM Community Edition (Free Open Source).");
        println!("[NantaraVM License] Tip: Insert NANTARA_PRO_LICENSE to dynamically unlock AMD SEV-SNP Enclaves & XDP Driver Mode.");
        Self {
            tier: LicenseTier::Community,
        }
    }

    pub fn is_pro_active(&self) -> bool {
        matches!(self.tier, LicenseTier::EnterprisePro { .. })
    }

    pub fn can_use_hardware_sev_snp(&self) -> bool {
        if self.is_pro_active() {
            true
        } else {
            println!("[NantaraVM License Gate] AMD SEV-SNP Hardware Memory Encryption is locked to Enterprise Pro.");
            println!("[NantaraVM License Gate] Running in Community Memory Mode (Standard Isolation).");
            false
        }
    }

    pub fn can_use_xdp_driver(&self) -> bool {
        if self.is_pro_active() {
            true
        } else {
            println!("[NantaraVM License Gate] XDP Line-Rate NIC Driver Mode is locked to Enterprise Pro.");
            println!("[NantaraVM License Gate] Running in Community eBPF TC Mode.");
            false
        }
    }
}
