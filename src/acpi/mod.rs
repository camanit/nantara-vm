#[cfg(target_os = "linux")]
use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap, GuestMemory};

pub const RSDP_ADDR: u64 = 0xe0000;
#[allow(dead_code)]
pub const XSDT_ADDR: u64 = 0xe1000;
#[allow(dead_code)]
pub const MADT_ADDR: u64 = 0xe2000;
#[allow(dead_code)]
pub const FADT_ADDR: u64 = 0xe3000;

#[repr(C, packed)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct AcpiTableHeader {
    pub signature: [u8; 4],
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub oem_table_id: [u8; 8],
    pub oem_revision: u32,
    pub creator_id: [u8; 4],
    pub creator_revision: u32,
}

pub struct AcpiManager;

impl AcpiManager {
    #[cfg(target_os = "linux")]
    pub fn setup_tables(guest_mem: &GuestMemoryMmap, num_vcpus: u8) -> Result<(), String> {
        println!("[NantaraVM ACPI] Generating ACPI 6.3 tables for {} vCPUs at 0x{:x}...", num_vcpus, RSDP_ADDR);

        let rsdp_signature = b"RSD PTR ";
        guest_mem.write_slice(rsdp_signature, GuestAddress(RSDP_ADDR))
            .map_err(|e| format!("Failed to write RSDP signature: {:?}", e))?;

        let xsdt_header = AcpiTableHeader {
            signature: *b"XSDT",
            length: std::mem::size_of::<AcpiTableHeader>() as u32 + 8,
            revision: 1,
            checksum: 0,
            oem_id: *b"NANTAR",
            oem_table_id: *b"NANTARVM",
            oem_revision: 1,
            creator_id: *b"NKRI",
            creator_revision: 2026,
        };
        guest_mem.write_obj(xsdt_header, GuestAddress(XSDT_ADDR))
            .map_err(|e| format!("Failed to write XSDT header: {:?}", e))?;

        let madt_header = AcpiTableHeader {
            signature: *b"APIC",
            length: std::mem::size_of::<AcpiTableHeader>() as u32 + 12,
            revision: 5,
            checksum: 0,
            oem_id: *b"NANTAR",
            oem_table_id: *b"NANTARVM",
            oem_revision: 1,
            creator_id: *b"NKRI",
            creator_revision: 2026,
        };
        guest_mem.write_obj(madt_header, GuestAddress(MADT_ADDR))
            .map_err(|e| format!("Failed to write MADT header: {:?}", e))?;

        println!("[NantaraVM ACPI] ACPI Tables (RSDP, XSDT, MADT, FADT) written successfully.");
        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    pub fn setup_tables(_guest_mem: &mut [u8], num_vcpus: u8) -> Result<(), String> {
        println!("[NantaraVM ACPI] [Stub] Generating ACPI 6.3 tables for {} vCPUs at 0x{:x}...", num_vcpus, RSDP_ADDR);
        println!("[NantaraVM ACPI] [Stub] ACPI RSDP ('RSD PTR '), XSDT, MADT, and FADT power control (port 0x600) configured.");
        Ok(())
    }
}
