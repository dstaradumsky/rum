fn shl(word: u32, bits: u32) -> u32 {
    assert!(bits <= 32);
    if bits == 32 {
        0
    } else {
        word << bits
    }
}

fn shr(word: u32, bits: u32) -> u32 {
    assert!(bits <= 32);
    if bits == 32 {
        0
    } else {
        word >> bits
    }
}

pub fn getu(word: u32, width: u32, lsb: u32) -> u32 {
    let hi = lsb + width;
    assert!(hi <= 32);
    shr(shl(word, 32 - hi), 32 - width)
}
