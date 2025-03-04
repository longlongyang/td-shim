// Copyright (c) 2021 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent

use core::mem::size_of;
use zerocopy::{AsBytes, FromBytes};

use td_shim::e820::E820Entry;

// Reference: The Linux/x86 Boot Protocol at https://docs.kernel.org/x86/boot.html
// 1.2. The Real-Mode Kernel Header
pub const HDR_BOOT_FLAG: u16 = 0xaa55;
pub const HDR_SIGNATURE: u32 = 0x5372_6448;
// 1.3. Details of Header Fields
// Our boot loader does not have an assigned id
pub const HDR_TYPE_LOADER: u8 = 0xff;
// 1.10. Loading The Rest of The Kernel
pub const HDR_LOAD_HIGH: u8 = 0x01;
pub const HDR_MIN_VERSION: u16 = 0x0200;

#[derive(Clone, Copy, Default, Debug, AsBytes, FromBytes)]
#[repr(C, packed)]
pub struct SetupHeader {
    pub setup_sects: u8,
    pub root_flags: u16,
    pub syssize: u32,
    pub ram_size: u16,
    pub vid_mode: u16,
    pub root_dev: u16,
    pub boot_flag: u16,
    pub jump: u16,
    pub header: u32,
    pub version: u16,
    pub realmode_swtch: u32,
    pub start_sys_seg: u16,
    pub kernel_version: u16,
    pub type_of_loader: u8,
    pub loadflags: u8,
    pub setup_move_size: u16,
    pub code32_start: u32,
    pub ramdisk_image: u32,
    pub ramdisk_size: u32,
    pub bootsect_kludge: u32,
    pub heap_end_ptr: u16,
    pub ext_loader_ver: u8,
    pub ext_loader_type: u8,
    pub cmd_line_ptr: u32,
    pub initrd_addr_max: u32,
    pub kernel_alignment: u32,
    pub relocatable_kernel: u8,
    pub min_alignment: u8,
    pub xloadflags: u16,
    pub cmdline_size: u32,
    pub hardware_subarch: u32,
    pub hardware_subarch_data: u64,
    pub payload_offset: u32,
    pub payload_length: u32,
    pub setup_data: u64,
    pub pref_address: u64,
    pub init_size: u32,
    pub handover_offset: u32,
}

impl SetupHeader {
    // Read a kernel header from the first two sectors of a file
    pub fn from_file(kernel_file: &[u8]) -> Self {
        SetupHeader::read_from(&kernel_file[0x1f1..0x1f1 + size_of::<SetupHeader>()]).unwrap()
    }
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct BootParams {
    screen_info: ScreenInfo,        // 0x000/0x040
    apm_bios_info: ApmBiosInfo,     // 0x040/0x014
    _pad2: [u8; 4],                 // 0x054/0x004
    tboot_addr: u64,                // 0x058/0x002
    ist_info: IstInfo,              // 0x060/0x010
    pub acpi_rsdp_addr: u64,        // 0x070/0x008
    pub unaccepted_memory: u64,     // 0x078/0x008
    hd0_info: [u8; 16],             // 0x080/0x010 - obsolete
    hd1_info: [u8; 16],             // 0x090/0x010 - obsolete
    sys_desc_table: SysDescTable,   // 0x0a0/0x010 - obsolete
    olpc_ofw_header: OlpcOfwHeader, // 0x0b0/0x010
    ext_ramdisk_image: u32,         // 0x0c0/0x004
    ext_ramdisk_size: u32,          // 0x0c4/0x004
    ext_cmd_line_ptr: u32,          // 0x0c8/0x004
    _pad4: [u8; 116],               // 0x0cc/0x074
    edd_info: EdidInfo,             // 0x140/0x080
    efi_info: EfiInfo,              // 0x1c0/0x020
    alt_mem_k: u32,                 // 0x1e0/0x004
    scratch: u32,                   // 0x1e4/0x004
    pub e820_entries: u8,           // 0x1e8/0x001
    eddbuf_entries: u8,             // 0x1e9/0x001
    edd_mbr_sig_buf_entries: u8,    // 0x1ea/0x001
    kbd_status: u8,                 // 0x1eb/0x001
    secure_boot: u8,                // 0x1ec/0x001
    _pad5: [u8; 2],                 // 0x1ed/0x002
    sentinel: u8,                   // 0x1ef/0x001
    _pad6: [u8; 1],                 // 0x1f0/0x001
    pub hdr: SetupHeader,           // 0x1f1
    _pad7: [u8; 0x290 - 0x1f1 - size_of::<SetupHeader>()],
    edd_mbr_sig_buffer: [u32; 16],    // 0x290
    pub e820_table: [E820Entry; 128], // 0x2d0
    _pad8: [u8; 48],                  // 0xcd0
    eddbuf: [EddInfo; 6],             // 0xd00
    _pad9: [u8; 276],                 // 0xeec
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct ScreenInfo([u8; 0x40]);

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct ApmBiosInfo([u8; 0x14]);

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct IstInfo([u8; 0x10]);

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct SysDescTable([u8; 0x10]);

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct OlpcOfwHeader([u8; 0x10]);

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct EdidInfo([u8; 0x80]);

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct EfiInfo([u8; 0x20]);

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct EddInfo([u8; 0x52]);

impl Default for BootParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
