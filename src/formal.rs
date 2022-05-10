use crate::*;


// cargo kani  2,40s user 0,15s system 99% cpu 2,572 total
#[kani::proof]
fn test_formal_u64() {

    let i = kani::any();
    assert_eq!(soft::u64_to_f32_round(i), (i as f32).to_bits());
    assert_eq!(fast::u64_to_f32_round(i), i as f32);

    let f = i as f64;
//    let t = if f as u64 > i || i == u64::max_value() {
//        f64::from_bits(f.to_bits() - 1)
//    } else {
//        f
//    };

    assert_eq!(soft::u64_to_f64_round(i), f.to_bits());
//    assert_eq!(soft::u64_to_f64_truncate(i), t.to_bits());

    assert_eq!(fast::u64_to_f64_round(i), f);
//    assert_eq!(fast::u64_to_f64_truncate(i), t);
}


// cargo kani  71,92s user 0,43s system 99% cpu 1:12,72 total
#[kani::proof]
fn test_formal_u128() {

    let i = kani::any();
    assert_eq!(soft::u128_to_f32_round(i), (i as f32).to_bits());
    assert_eq!(fast::u128_to_f32_round(i), i as f32);
    let f = i as f64;
//    let t = if f as u128 > i || i == u128::max_value() {
//        f64::from_bits(f.to_bits() - 1)
//    } else {
//        f
//    };
    assert_eq!(soft::u128_to_f64_round(i), f.to_bits());
//    assert_eq!(soft::u128_to_f64_truncate(i), t.to_bits());
    assert_eq!(fast::u128_to_f64_round(i), f);
//    assert_eq!(fast::u128_to_f64_truncate(i), t);
}


// cargo kani  2,11s user 0,20s system 100% cpu 2,301 total
#[kani::proof]
fn test_formal_i32() {

    let i = kani::any();
    assert_eq!(soft::i32_to_f64(i), (i as f64).to_bits());
    assert_eq!(fast::i32_to_f64(i), i as f64);
    assert_eq!(soft::i32_to_f32_round(i), (i as f32).to_bits());
    assert_eq!(fast::i32_to_f32_round(i), i as f32);
}

// cargo kani  8,31s user 0,24s system 99% cpu 8,571 total
#[kani::proof]
fn test_formal_i64() {

    let i = kani::any();
    assert_eq!(soft::i64_to_f32_round(i), (i as f32).to_bits());
    assert_eq!(fast::i64_to_f32_round(i), i as f32);
    assert_eq!(soft::i64_to_f64_round(i), (i as f64).to_bits());
    assert_eq!(fast::i64_to_f64_round(i), i as f64);
}

// cargo kani  481,00s user 1,19s system 98% cpu 8:07,21 total
#[kani::proof]
fn test_formal_i128() {

    let i = kani::any();
    assert_eq!(soft::i128_to_f32_round(i), (i as f32).to_bits());
    assert_eq!(fast::i128_to_f32_round(i), i as f32);
    assert_eq!(soft::i128_to_f64_round(i), (i as f64).to_bits());
    assert_eq!(fast::i128_to_f64_round(i), i as f64);
}
