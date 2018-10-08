pub fn u16_to_nibbles(i: u16) -> (u8, u8, u8, u8) {
    (
        ((i & 0xf000) >> 12) as u8,
        ((i & 0x0f00) >> 8) as u8,
        ((i & 0x00f0) >> 4) as u8,
        (i & 0x000f) as u8,
    )
}

pub fn nibbles_to_u16(n1: u8, n2: u8, n3: u8, n4: u8) -> u16 {
    (n1 as u16) << 12 | (n2 as u16) << 8 | (n3 as u16) << 4 | (n4 as u16)
}

pub fn nibbles_to_u8(n1: u8, n2: u8) -> u8 {
    ((n1 as u16) << 4 | (n2 as u16) & 0xFF) as u8
}
