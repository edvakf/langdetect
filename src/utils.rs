
pub fn has32_with_default_seed(input: &str) -> u32 {
    return hash32(input.as_bytes(), input.len(), 0xBEEF);
}

#[inline]
fn decode_fixed32(a: u8, b: u8, c: u8, d: u8) -> u32 {
    return a as u32 |
        ((b as u32) << 8) |
        ((c as u32) << 16) |
        ((d as u32) << 24);
}

#[inline]
fn byte_as_32(c: u8) -> u32 {
    return (c as u32) & 0xff;
}

#[allow(dead_code)]
fn hash32(data: &[u8], n: usize, seed: u32) -> u32 {
    let mut nn = n as u32;

    // 'm' and 'r' are mixing constants generated offline.
    // They're not really 'magic', they just happen to work well.
    let m: u32 = 0x5bd1e995;
    let r = 24;

    // Initialize the hash to a 'random' value
    let mut h: u32 = seed ^ nn;

    // Mix 4 bytes at a time into the hash
    let mut i = 0;
    while nn >= 4 {
        let mut k: u32 = decode_fixed32(
            data[i],
            data[i + 1],
            data[i + 2],
            data[i + 3]
        );
        k = k.wrapping_mul(m);
        k ^= k >> r;
        k = k.wrapping_mul(m);
        h = h.wrapping_mul(m);
        h ^= k;
        nn -= 4;
        i += 4;
    }

    // Handle the last few bytes of the input array
    if nn == 3 {
        h ^= byte_as_32(data[i + 2]) << 16;
        h ^= byte_as_32(data[i + 1]) << 8;
        h ^= byte_as_32(data[i]);
        h = h.wrapping_mul(m);
    } else if nn == 2 {
        h ^= byte_as_32(data[i + 1]) << 8;
        h ^= byte_as_32(data[i]);
        h = h.wrapping_mul(m);
    } else if nn == 1 {
        h ^= byte_as_32(data[i]);
        h = h.wrapping_mul(m);
    }

    // Do a few final mixes of the hash to ensure the last few
    // bytes are well-incorporated.
    h ^= h >> 13;
    h = h.wrapping_mul(m);
    h ^= h >> 15;
    return h;
}

#[cfg(test)]
mod tests {
    use super::has32_with_default_seed;

    #[test]
    fn test_has32_with_default_seed() {
        assert_eq!(has32_with_default_seed("hoge fuga"), 3023814137);
    }
}
