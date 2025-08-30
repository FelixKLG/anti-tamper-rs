pub const MAGIC_BYTES: [u8; 8] = [0xfc, 0x10, 0x19, 0x25, 0x2e, 0x43, 0x59, 0x5f];
pub const EMBEDDED_HASH: [u8; 32] = [0; 32];

#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".rdata$z")]
// #[unsafe(link_section = "__TEXT,__rodata")]
pub static HASH_LOCATION: HashContainer = HashContainer {
    magic: MAGIC_BYTES,
    hash: EMBEDDED_HASH,
};

#[repr(C)]
pub struct HashContainer {
    pub magic: [u8; 8],
    pub hash: [u8; 32],
}