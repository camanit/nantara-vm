document.addEventListener('DOMContentLoaded', () => {
    console.log('NantaraVM Web — Early Alpha Preview Loaded.');

    const simBody = document.getElementById('sim-body');
    const btnRunSim = document.getElementById('btn-run-sim');
    const btnTogglePro = document.getElementById('btn-toggle-pro');

    // Remove license generator if exists (no longer needed)
    const btnGenLicense = document.getElementById('btn-gen-license');
    const licenseSection = document.getElementById('license-portal');
    if (licenseSection) licenseSection.style.display = 'none';

    // === Architecture Demo Terminal ===
    // Shows what the OUTPUT would look like when NantaraVM runs.
    // This is a simulation of the planned boot sequence, not a live system.
    function runSimulation() {
        if (!simBody) return;
        simBody.innerHTML = '';

        const logs = [
            { text: '====================================================', type: 'info' },
            { text: ' 🏗️  NantaraVM Architecture Demo — Rencana Output Boot', type: 'info' },
            { text: '====================================================', type: 'info' },
            { text: '[DEMO] Ini adalah simulasi output yang direncanakan.', type: 'warn' },
            { text: '[DEMO] Bukan output dari sistem yang sedang berjalan.', type: 'warn' },
            { text: '', type: 'info' },
            { text: '[NantaraVM] Allocating 512 MB Guest Physical RAM...', type: 'normal' },
            { text: '[NantaraVM] Opening /dev/kvm interface...  ← butuh Linux + KVM', type: 'normal' },
            { text: '[NantaraVM] KVM VM created successfully.', type: 'normal' },
            { text: '', type: 'info' },
            { text: '--- [Jailer: Process Isolation] ---', type: 'info' },
            { text: '[Jailer] Spawning device process in new namespaces...', type: 'normal' },
            { text: '[Landlock] Applying filesystem restriction rules...', type: 'normal' },
            { text: '', type: 'info' },
            { text: '--- [VirtIO Devices] ---', type: 'info' },
            { text: '[VirtIO-blk] MMIO device registered at 0xd0000000', type: 'normal' },
            { text: '[VirtIO-net] MMIO device registered at 0xd0000200 (tap0)', type: 'normal' },
            { text: '', type: 'info' },
            { text: '--- [Kernel Boot] ---', type: 'info' },
            { text: '[Boot] Loading vmlinux via PVH Direct Boot...', type: 'normal' },
            { text: '[Boot] Zero page configured. Entry RIP = 0x100000', type: 'normal' },
            { text: '[vCPU-0] Starting execution loop...', type: 'normal' },
            { text: '', type: 'info' },
            { text: '--- [Planned: Guest OS Output via Serial] ---', type: 'info' },
            { text: '[ttyS0] Booting Linux 6.6.0...', type: 'success' },
            { text: '[ttyS0] VirtIO drivers loaded.', type: 'success' },
            { text: '[ttyS0] Login: root', type: 'success' },
            { text: '', type: 'info' },
            { text: '====================================================', type: 'info' },
            { text: ' ✅ Demo selesai. Implementasi nyata sedang dikerjakan!', type: 'success' },
            { text: '    GitHub: github.com/camanit/nantara-vm', type: 'success' },
            { text: '====================================================', type: 'info' },
        ];

        logs.forEach((log, idx) => {
            setTimeout(() => {
                const div = document.createElement('div');
                div.className = `log-line log-${log.type}`;
                div.textContent = log.text;
                simBody.appendChild(div);
                simBody.scrollTop = simBody.scrollHeight;
            }, idx * 75);
        });
    }

    if (btnRunSim) {
        btnRunSim.addEventListener('click', runSimulation);
    }

    // Remove "Toggle Pro Mode" button or repurpose it
    if (btnTogglePro) {
        btnTogglePro.textContent = '🔗 Lihat GitHub';
        btnTogglePro.addEventListener('click', () => {
            window.open('https://github.com/camanit/nantara-vm', '_blank');
        });
    }

    // AI Chatbot — honest responses
    const aiBubble = document.getElementById('ai-bubble');
    const aiDrawer = document.getElementById('ai-drawer');
    const aiClose  = document.getElementById('ai-close');
    const aiMessages = document.getElementById('ai-messages');
    const aiInput  = document.getElementById('ai-input');
    const aiSend   = document.getElementById('ai-send');

    if (aiBubble && aiDrawer) {
        aiBubble.addEventListener('click', () => {
            aiDrawer.style.removeProperty('display');
            aiDrawer.classList.toggle('open');
        });
    }

    if (aiClose && aiDrawer) {
        aiClose.addEventListener('click', () => {
            aiDrawer.classList.remove('open');
        });
    }

    function sendAiMessage() {
        if (!aiInput) return;
        const text = aiInput.value.trim();
        if (!text) return;

        const userMsg = document.createElement('div');
        userMsg.className = 'msg msg-user';
        userMsg.textContent = text;
        aiMessages.appendChild(userMsg);
        aiInput.value = '';
        aiMessages.scrollTop = aiMessages.scrollHeight;

        setTimeout(() => {
            const aiMsg = document.createElement('div');
            aiMsg.className = 'msg msg-ai';
            aiMsg.textContent = getAiResponse(text);
            aiMessages.appendChild(aiMsg);
            aiMessages.scrollTop = aiMessages.scrollHeight;
        }, 500);
    }

    function getAiResponse(input) {
        const q = input.toLowerCase();

        if (q.includes('download') || q.includes('unduh') || q.includes('install')) {
            return "NantaraVM saat ini tersedia sebagai source code di GitHub. Build dari source menggunakan `cargo build` di Linux. Binary release akan tersedia setelah Fase 1 selesai.";
        } else if (q.includes('windows') || q.includes('win10') || q.includes('win11')) {
            return "Boot Windows 10/11 di NantaraVM adalah target jangka panjang (2026+). Saat ini membutuhkan UEFI/OVMF dan VirtIO driver yang belum diimplementasi. Untuk sekarang, gunakan Hyper-V atau VirtualBox.";
        } else if (q.includes('lisensi') || q.includes('pro') || q.includes('bayar') || q.includes('harga')) {
            return "NantaraVM adalah proyek open-source gratis. Saat ini tidak ada lisensi berbayar. Fokus kami adalah membangun produk yang benar-benar berfungsi dulu.";
        } else if (q.includes('kvm') || q.includes('boot') || q.includes('bisa jalan')) {
            return "NantaraVM membutuhkan Linux dengan /dev/kvm untuk menjalankan VM nyata. Di WSL2, perlu virtualisasi diaktifkan di BIOS/UEFI. Saat ini fitur boot Linux sederhana sedang dalam pengembangan aktif.";
        } else if (q.includes('sev') || q.includes('enkripsi') || q.includes('keamanan')) {
            return "AMD SEV-SNP adalah roadmap jangka panjang dan membutuhkan server fisik dengan CPU AMD EPYC yang mendukung fitur tersebut. Belum diimplementasi saat ini.";
        } else if (q.includes('kontribusi') || q.includes('ikut') || q.includes('bantu')) {
            return "Terima kasih! Kontribusi sangat disambut. Cek GitHub Issues kami untuk task yang tersedia. Yang paling dibutuhkan sekarang: implementasi vCPU run loop dan VirtIO device di Rust.";
        } else if (q.includes('wa') || q.includes('whatsapp') || q.includes('kontak')) {
            return "Hubungi kami via WhatsApp di +62 812-6000-6666 atau buka GitHub repository untuk diskusi teknis.";
        } else {
            return "Halo! Saya Nantara AI Assistant. NantaraVM adalah proyek open-source hypervisor Rust buatan Indonesia yang sedang aktif dikembangkan. Ada yang ingin Anda tanyakan tentang arsitektur, status pengembangan, atau cara berkontribusi?";
        }
    }

    if (aiSend) aiSend.addEventListener('click', sendAiMessage);
    if (aiInput) aiInput.addEventListener('keypress', e => { if (e.key === 'Enter') sendAiMessage(); });

    // Auto-run simulation on load
    runSimulation();
});
