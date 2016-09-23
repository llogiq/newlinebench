#![feature(test)]

extern crate test;

use std::{cmp, iter};

fn repeated(c: u8) -> String {
    String::from_utf8(iter::repeat(c).take(10000).collect()).unwrap()
}

fn count_newlines(s: &str) -> usize {
    s.as_bytes().iter().filter(|&&c| c == b'\n').count()
}

#[cfg(target_pointer_width = "16")] const USIZE_BYTES: usize = 2;
#[cfg(target_pointer_width = "32")] const USIZE_BYTES: usize = 4;
#[cfg(target_pointer_width = "64")] const USIZE_BYTES: usize = 8;
const LO : usize = ::std::usize::MAX / 255;
const HI : usize = LO * 128;
const REP_NEWLINE : usize = b'\n' as usize * LO;

fn count_newlines_fast(s: &str) -> usize {
    fn count_zero_bytes(x: usize) -> usize {
        ((x.wrapping_sub(LO)) & !x & HI) / 128 % 255
    }
    let text = s.as_bytes();
    let (ptr, len) = (text.as_ptr(), text.len());

    // search up to an aligned boundary
    let align = (ptr as usize) & (USIZE_BYTES - 1);
    let mut offset;
    let mut count;
    if align > 0 {
        offset = cmp::min(USIZE_BYTES - align, len);
        count = text[..offset].iter().filter(|b| **b == b'\n').count();
    } else {
        offset = 0;
        count = 0;
    }

    // search the body of the text
    if len >= 2 * USIZE_BYTES {
        while offset <= len - 2 * USIZE_BYTES {
            unsafe {
                let u = *(ptr.offset(offset as isize) as *const usize);
                let v = *(ptr.offset((offset + USIZE_BYTES) as isize) as *const usize);

                // break if there is a matching byte
                count += count_zero_bytes(u ^ REP_NEWLINE);
                count += count_zero_bytes(v ^ REP_NEWLINE);
            }
            offset += USIZE_BYTES * 2;
        }
    }
    // search the rest
    count + text[offset..].iter().filter(|b| **b == b'\n').count()
}


fn count_newlines_faster(s: &str) -> usize {
    fn count_zero_bytes(x: usize) -> u32 {
        ((x.wrapping_sub(LO)) & !x & HI).count_ones()
    }
    let text = s.as_bytes();
    let (ptr, len) = (text.as_ptr(), text.len());

    // search up to an aligned boundary
    let align = (ptr as usize) & (USIZE_BYTES - 1);
    let mut offset;
    let mut count;
    if align > 0 {
        offset = cmp::min(USIZE_BYTES - align, len);
        count = text[..offset].iter().filter(|b| **b == b'\n').count();
    } else {
        offset = 0;
        count = 0;
    }

    // search the body of the text
    if len >= 2 * USIZE_BYTES {
        while offset <= len - 2 * USIZE_BYTES {
            unsafe {
                let u = *(ptr.offset(offset as isize) as *const usize);
                let v = *(ptr.offset((offset + USIZE_BYTES) as isize) as *const usize);

                count += (count_zero_bytes(u ^ REP_NEWLINE) +
                    count_zero_bytes(v ^ REP_NEWLINE)) as usize;
            }
            offset += USIZE_BYTES * 2;
        }
    }
    // search the rest
    count + text[offset..].iter().filter(|b| **b == b'\n').count()
}

fn count_newlines_fastest(s: &str) -> usize {
    fn mask_zero(x: usize) -> usize {
        ((x.wrapping_sub(LO)) & !x & HI)
    }
    let text = s.as_bytes();
    let (ptr, len) = (text.as_ptr(), text.len());

    let align = (ptr as usize) & (USIZE_BYTES - 1);
    let mut offset;
    let mut count;
    if align > 0 {
        offset = cmp::min(USIZE_BYTES - align, len);
        count = text[..offset].iter().filter(|b| **b == b'\n').count();
    } else {
        offset = 0;
        count = 0;
    }
    while offset + 8 * USIZE_BYTES <= len {
        unsafe {
            let x0 = *(ptr.offset(offset as isize) as *const usize);
            let x1 = *(ptr.offset((offset + USIZE_BYTES) as isize) as *const usize);
            let x2 = *(ptr.offset((offset + USIZE_BYTES * 2) as isize) as *const usize);
            let x3 = *(ptr.offset((offset + USIZE_BYTES * 3) as isize) as *const usize);
            let x4 = *(ptr.offset((offset + USIZE_BYTES * 4) as isize) as *const usize);
            let x5 = *(ptr.offset((offset + USIZE_BYTES * 5) as isize) as *const usize);
            let x6 = *(ptr.offset((offset + USIZE_BYTES * 6) as isize) as *const usize);
            let x7 = *(ptr.offset((offset + USIZE_BYTES * 7) as isize) as *const usize);

            count += (mask_zero(x0 ^ REP_NEWLINE)
                | mask_zero(x1 ^ REP_NEWLINE) >> 1
                | mask_zero(x2 ^ REP_NEWLINE) >> 2
                | mask_zero(x3 ^ REP_NEWLINE) >> 3
                | mask_zero(x4 ^ REP_NEWLINE) >> 4
                | mask_zero(x5 ^ REP_NEWLINE) >> 5
                | mask_zero(x6 ^ REP_NEWLINE) >> 6
                | mask_zero(x7 ^ REP_NEWLINE) >> 7
                ).count_ones() as usize;
        }
        offset += USIZE_BYTES * 8;
    }
    while offset + 4 * USIZE_BYTES <= len {
        unsafe {
            let x0 = *(ptr.offset(offset as isize) as *const usize);
            let x1 = *(ptr.offset((offset + USIZE_BYTES) as isize) as *const usize);
            let x2 = *(ptr.offset((offset + USIZE_BYTES * 2) as isize) as *const usize);
            let x3 = *(ptr.offset((offset + USIZE_BYTES * 3) as isize) as *const usize);

            count += (mask_zero(x0 ^ REP_NEWLINE)
                | mask_zero(x1 ^ REP_NEWLINE) >> 1
                | mask_zero(x2 ^ REP_NEWLINE) >> 2
                | mask_zero(x3 ^ REP_NEWLINE) >> 3
                ).count_ones() as usize;
        }
        offset += USIZE_BYTES * 4;
    }
    while offset + 2 * USIZE_BYTES <= len {
        unsafe {
            let x0 = *(ptr.offset(offset as isize) as *const usize);
            let x1 = *(ptr.offset((offset + USIZE_BYTES) as isize) as *const usize);

            count += (mask_zero(x0 ^ REP_NEWLINE) |
                mask_zero(x1 ^ REP_NEWLINE) >> 1).count_ones() as usize;
        }
        offset += USIZE_BYTES * 2;
    }
    while offset + USIZE_BYTES <= len {
        let x0 = unsafe { *(ptr.offset(offset as isize) as *const usize) };
        count += mask_zero(x0 ^ REP_NEWLINE).count_ones() as usize;
        offset += USIZE_BYTES;
    }
    count + text[offset..].iter().filter(|b| **b == b'\n').count()
}

fn count_newlines_screaming(s: &str) -> usize {
    fn mask_zero(x: usize) -> usize {
        ((x.wrapping_sub(LO)) & !x & HI) >> 7
    }
    let text = s.as_bytes();
    let (ptr, len) = (text.as_ptr(), text.len());

    let align = (ptr as usize) & (USIZE_BYTES - 1);
    let mut offset;
    let mut count;
    if align > 0 {
        offset = cmp::min(USIZE_BYTES - align, len);
        count = text[..offset].iter().filter(|b| **b == b'\n').count();
    } else {
        offset = 0;
        count = 0;
    }
    while offset + 8 * USIZE_BYTES <= len {
        unsafe {
            let x0 = *(ptr.offset(offset as isize) as *const usize);
            let x1 = *(ptr.offset((offset + USIZE_BYTES) as isize) as *const usize);
            let x2 = *(ptr.offset((offset + USIZE_BYTES * 2) as isize) as *const usize);
            let x3 = *(ptr.offset((offset + USIZE_BYTES * 3) as isize) as *const usize);
            let x4 = *(ptr.offset((offset + USIZE_BYTES * 4) as isize) as *const usize);
            let x5 = *(ptr.offset((offset + USIZE_BYTES * 5) as isize) as *const usize);
            let x6 = *(ptr.offset((offset + USIZE_BYTES * 6) as isize) as *const usize);
            let x7 = *(ptr.offset((offset + USIZE_BYTES * 7) as isize) as *const usize);

            count += ((mask_zero(x0 ^ REP_NEWLINE) + mask_zero(x1 ^ REP_NEWLINE)
                     + mask_zero(x2 ^ REP_NEWLINE) + mask_zero(x3 ^ REP_NEWLINE))
                    + (mask_zero(x4 ^ REP_NEWLINE) + mask_zero(x5 ^ REP_NEWLINE)
                     + mask_zero(x6 ^ REP_NEWLINE) + mask_zero(x7 ^ REP_NEWLINE))
                ).wrapping_mul(LO) >> ((USIZE_BYTES - 1) * 8);
        }
        offset += USIZE_BYTES * 8;
    }
    while offset + 4 * USIZE_BYTES <= len {
        unsafe {
            let x0 = *(ptr.offset(offset as isize) as *const usize);
            let x1 = *(ptr.offset((offset + USIZE_BYTES) as isize) as *const usize);
            let x2 = *(ptr.offset((offset + USIZE_BYTES * 2) as isize) as *const usize);
            let x3 = *(ptr.offset((offset + USIZE_BYTES * 3) as isize) as *const usize);

            count += (mask_zero(x0 ^ REP_NEWLINE) + mask_zero(x1 ^ REP_NEWLINE)
                    + mask_zero(x2 ^ REP_NEWLINE) + mask_zero(x3 ^ REP_NEWLINE)
                ).wrapping_mul(LO) >> ((USIZE_BYTES - 1) * 8)
        }
        offset += USIZE_BYTES * 4;
    }
    while offset + 2 * USIZE_BYTES <= len {
        unsafe {
            let x0 = *(ptr.offset(offset as isize) as *const usize);
            let x1 = *(ptr.offset((offset + USIZE_BYTES) as isize) as *const usize);

            count += (mask_zero(x0 ^ REP_NEWLINE) + mask_zero(x1 ^ REP_NEWLINE)
                ).wrapping_mul(LO) >> ((USIZE_BYTES - 1) * 8)
        }
        offset += USIZE_BYTES * 2;
    }
    while offset + USIZE_BYTES <= len {
        let x0 = unsafe { *(ptr.offset(offset as isize) as *const usize) };
        count += mask_zero(x0 ^ REP_NEWLINE).wrapping_mul(LO) >> ((USIZE_BYTES - 1) * 8);
        offset += USIZE_BYTES;
    }
    count + text[offset..].iter().filter(|b| **b == b'\n').count()
}

#[bench]
fn test_slow_nonewlines(b: &mut test::Bencher) {
    let data = repeated(b'n');
    b.iter(move|| count_newlines(&data))
}

#[bench]
fn test_fast_nonewlines(b: &mut test::Bencher) {
    let data = repeated(b'n');
    b.iter(move|| count_newlines_fast(&data))
}

#[bench]
fn test_faster_nonewlines(b: &mut test::Bencher) {
    let data = repeated(b'n');
    b.iter(move|| count_newlines_faster(&data))
}

#[bench]
fn test_fastest_nonewlines(b: &mut test::Bencher) {
    let data = repeated(b'n');
    b.iter(move|| count_newlines_fastest(&data))
}

#[bench]
fn test_screaming_nonewlines(b: &mut test::Bencher) {
    let data = repeated(b'n');
    b.iter(move|| count_newlines_screaming(&data))
}

#[bench]
fn test_slow_newlines(b: &mut test::Bencher) {
    let data = repeated(b'\n');
    b.iter(move|| count_newlines(&data))
}

#[bench]
fn test_fast_newlines(b: &mut test::Bencher) {
    let data = repeated(b'\n');
    b.iter(move|| count_newlines_fast(&data))
}

#[bench]
fn test_faster_newlines(b: &mut test::Bencher) {
    let data = repeated(b'\n');
    b.iter(move|| count_newlines_faster(&data))
}

#[bench]
fn test_fastest_newlines(b: &mut test::Bencher) {
    let data = repeated(b'\n');
    b.iter(move|| count_newlines_fastest(&data))
}


#[bench]
fn test_screaming_newlines(b: &mut test::Bencher) {
    let data = repeated(b'\n');
    b.iter(move|| count_newlines_screaming(&data))
}

#[bench]
fn test_slow_somenewlines(b: &mut test::Bencher) {
    let data = "abcd\nbcda\ncdab\ndabc\n\nbc\na\n\nd\n\nc\na\nc\n\n\n\n\nx";
    b.iter(move|| count_newlines(data))
}

#[bench]
fn test_fast_somenewlines(b: &mut test::Bencher) {
    let data = "abcd\nbcda\ncdab\ndabc\n\nbc\na\n\nd\n\nc\na\nc\n\n\n\n\nx";
    b.iter(move|| count_newlines_fast(data))
}

#[bench]
fn test_faster_somenewlines(b: &mut test::Bencher) {
    let data = "abcd\nbcda\ncdab\ndabc\n\nbc\na\n\nd\n\nc\na\nc\n\n\n\n\nx";
    b.iter(move|| count_newlines_faster(data))
}

#[bench]
fn test_fastest_somenewlines(b: &mut test::Bencher) {
    let data = "abcd\nbcda\ncdab\ndabc\n\nbc\na\n\nd\n\nc\na\nc\n\n\n\n\nx";
    b.iter(move|| count_newlines_fastest(data))
}

#[bench]
fn test_screaming_somenewlines(b: &mut test::Bencher) {
    let data = "abcd\nbcda\ncdab\ndabc\n\nbc\na\n\nd\n\nc\na\nc\n\n\n\n\nx";
    b.iter(move|| count_newlines_screaming(data))
}


#[test]
fn check() {
    let nonewlines = repeated(b'X');
    assert_eq!(count_newlines(&nonewlines), count_newlines_fast(&nonewlines));
    assert_eq!(count_newlines(&nonewlines), count_newlines_faster(&nonewlines));
    assert_eq!(count_newlines(&nonewlines), count_newlines_fastest(&nonewlines));
    assert_eq!(count_newlines(&nonewlines), count_newlines_screaming(&nonewlines));
    let newlines = repeated(b'\n');
    assert_eq!(count_newlines(&newlines), count_newlines_fast(&newlines));
    assert_eq!(count_newlines(&newlines), count_newlines_faster(&newlines));
    assert_eq!(count_newlines(&newlines), count_newlines_fastest(&newlines));
    assert_eq!(count_newlines(&newlines), count_newlines_screaming(&newlines));
    let somenewlines = "abcd\nbcda\ncdab\ndabc\n\nbc\na\n\nd\n\nc\na\nc\n\n\n\n\nx";
    assert_eq!(count_newlines(somenewlines), count_newlines_fast(somenewlines));
    assert_eq!(count_newlines(somenewlines), count_newlines_faster(somenewlines));
    assert_eq!(count_newlines(somenewlines), count_newlines_fastest(somenewlines));
    assert_eq!(count_newlines(somenewlines), count_newlines_screaming(somenewlines));
}
