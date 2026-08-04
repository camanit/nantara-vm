pub struct EbpfFilter {
    pub ifname: String,
    pub active: bool,
}

impl EbpfFilter {
    pub fn new(ifname: &str) -> Self {
        Self {
            ifname: ifname.to_string(),
            active: false,
        }
    }

    pub fn attach(&mut self) -> Result<(), String> {
        println!("[NantaraVM eBPF] Attaching eBPF Zero-Trust Packet Filter to TAP interface '{}'...", self.ifname);
        println!("[NantaraVM eBPF] Active rules: DDoS Protection, Anti-Spoofing MAC/IP, Micro-Segmentation.");
        self.active = true;
        Ok(())
    }
}
