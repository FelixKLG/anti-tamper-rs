const MAGIC_BYTES: [u8; 8] = [0xfc, 0x10, 0x19, 0x25, 0x2e, 0x43, 0x59, 0x5f];
const SANITY_BYTES: [u8; 2] = [0x00, 0x00];
const EMBEDDED_HASH: [u8; 32] = [0; 32];

#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".rodata")]
pub static HASH_LOCATION: HashContainer = HashContainer {
    magic: MAGIC_BYTES,
    sanity: SANITY_BYTES,
    hash: EMBEDDED_HASH,
};

#[repr(C)]
pub struct HashContainer {
    pub magic: [u8; 8],
    pub sanity: [u8; 2],
    pub hash: [u8; 32],
}
