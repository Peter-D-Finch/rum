/// Returns true iff the signed value `n` fits into `width` signed bits.
/// 
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    assert!(width <= 64);
    let mut abs_n: u64 = n as u64;
    if n < 0 { abs_n = (!n + 1) as u64; }
    return fitsu(abs_n, width-1);
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
/// 
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    assert!(width <= 64);
    if width == 0 && n == 0 { return true; }
    if width == 0 { return false; }
    let base: u64 = 2;
    let check = base.pow(width as u32) - 1;
    if n > check  { return false; }
    return true;
}

/// Retrieve a signed value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn gets(word: u64, width: u64, lsb: u64) -> i64 {
    assert!((width + lsb) <= 64);
    let base: u64 = 2;
    let check: u64 = ((1 as u64) << (width+lsb-1)) & word;  
    let mut sign = 0;
    if check > 0  { sign = 1; }
    return (getu(word, width, lsb) as i64) - (sign * base.pow(width as u32)) as i64;
}

/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {
    assert!((width + lsb) <= 64);
    let value: u64;
    let base: u64 = 2;
    let mut temp: u64 = base.pow(width as u32)-1;
    if lsb != 64 { temp = temp << lsb; }
    value = (temp & word) >> lsb;
    return value;
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the unsigned `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` unsigned bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the unsigned value to place into that bit field
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    assert!((width+lsb) < 64);
    if fitsu(value, width) {
        let base: u64 = 2;
        let mut temp: u64 = base.pow(width as u32) - 1;
        temp = !(temp << (lsb));
        let result = (word & temp ) | (value << (lsb));
        Some(result)
    }
    else {
        None
    }
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the signed `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` signed bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    if fitss(value, width) { 
        assert!(fitss(value, width) && 0 < lsb && (width+lsb) < 64);
        let base: u64 = 2;
        let placeholders: u64 = base.pow(width as u32) - 1;
        let new_value: u64 = (value as u64) & placeholders;
        Some(newu(word, width, lsb, new_value).unwrap())
    }
    else {
        None
    }
}


#[cfg(test)]
mod tests {
    use crate::bitpack::*;
    #[test]
    fn fitss_test() {
        let n: i64 = -1000;
        let width: u64 = 4;
        assert!(!fitss(n, width));
        let n: i64 = -3;
        let width: u64 = 4;
        assert!(fitss(n, width));
    }
    #[test]
    fn fitsu_test() {
        let n: u64 = 1000;
        let width: u64 = 4;
        assert!(!fitsu(n, width));
        let n: u64 = 3;
        let width: u64 = 4;
        assert!(fitsu(n, width));
    }
    #[test]
    fn newu_getu_test() {
        let n: u64 = 0;
        let packed = newu(n, 5, 0, 8).unwrap();
        let unpacked = getu(packed, 5, 0);
        assert!(unpacked == 8);
    }
    #[test]
    fn news_gets_test() {
        let n: u64 = 0;
        let packed = news(n, 8, 1, -1).unwrap();
        let unpacked = gets(packed, 8, 1);
        assert!(unpacked == -1);
    }
}