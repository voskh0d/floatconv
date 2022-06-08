use crate::*;

// cargo kani  1,62s user 0,22s system 98% cpu 1,875 total
// cargo test  0,66s user 0,15s system 106% cpu 0,758 total
//#[kani::proof]
//fn test_formal_u8() {
//
//    let i = kani::any();
//
//    let a = f32::from_bits(soft::u8_to_f32(i));
//    let b = i as f32;
//    assert_eq!(a, b, "{} -> f32", i);
//
//    let a = f64::from_bits(soft::u8_to_f64(i));
//    let b = i as f64;
//    assert_eq!(a, b, "{} -> f64", i);
//
//    let i = i as i8;
//    let a = f32::from_bits(soft::i8_to_f32(i));
//    let b = i as f32;
//    assert_eq!(a, b, "{} -> f32", i);
//
//    let a = f64::from_bits(soft::i8_to_f64(i));
//    let b = i as f64;
//    assert_eq!(a, b, "{} -> f64", i);
//}

// cargo kani  1,54s user 0,21s system 100% cpu 1,753 total
// cargo test  0,42s user 0,10s system 111% cpu 0,465 total
//#[kani::proof]
//fn test_formal_u16() {
//
//    let i = kani::any();
//
//    let a = f32::from_bits(soft::u16_to_f32(i));
//    let b = i as f32;
//    assert_eq!(a, b, "{} -> f32", i);
//
//    let a = f64::from_bits(soft::u16_to_f64(i));
//    let b = i as f64;
//    assert_eq!(a, b, "{} -> f64", i);
//
//    let i = i as i16;
//    let a = f32::from_bits(soft::i16_to_f32(i));
//    let b = i as f32;
//    assert_eq!(a, b, "{} -> f32", i);
//
//    let a = f64::from_bits(soft::i16_to_f64(i));
//    let b = i as f64;
//    assert_eq!(a, b, "{} -> f64", i);
//}
//

// cargo kani  1,95s user 0,18s system 99% cpu 2,133 total
// cargo test  394,99s user 0,17s system 99% cpu 6:37,03 total
//
//#[kani::proof]
//fn test_formal_u32() {
//
//    let i = kani::any();
//
//    let a = f32::from_bits(soft::u32_to_f32_round(i));
//    let b = i as f32;
//    assert_eq!(a, b, "{} -> f32", i);
//
//    let a = f64::from_bits(soft::u32_to_f64(i));
//    let b = i as f64;
//    assert_eq!(a, b, "{} -> f64", i);
//
//    let i = i as i32;
//    let a = f32::from_bits(soft::i32_to_f32_round(i));
//    let b = i as f32;
//    assert_eq!(a, b, "{} -> f32", i);
//
//    let a = f64::from_bits(soft::i32_to_f64(i));
//    let b = i as f64;
//    assert_eq!(a, b, "{} -> f64", i);
//}




//// cargo kani  2,40s user 0,15s system 99% cpu 2,572 total
//#[kani::proof]
//fn test_formal_u64() {
//
//    let i = kani::any();
////    assert_eq!(soft::u64_to_f32_round(i), (i as f32).to_bits());
////    assert_eq!(fast::u64_to_f32_round(i), i as f32);
////
////    let f = i as f64;
////    let t = if f as u64 > i || i == u64::max_value() {
////        f64::from_bits(f.to_bits() - 1)
////    } else {
////        f
////    };
////
////    assert_eq!(soft::u64_to_f64_round(i), f.to_bits());
////    assert_eq!(soft::u64_to_f64_truncate(i), t.to_bits());
//    assert_eq!(soft::u64_to_f64_truncate(i), 0);
////
////    assert_eq!(fast::u64_to_f64_round(i), f);
////    assert_eq!(fast::u64_to_f64_truncate(i), t);
//}
////
////
////// cargo kani  71,92s user 0,43s system 99% cpu 1:12,72 total
////#[kani::proof]
////fn test_formal_u128() {
////
////    let i = kani::any();
////    assert_eq!(soft::u128_to_f32_round(i), (i as f32).to_bits());
////    assert_eq!(fast::u128_to_f32_round(i), i as f32);
////    let f = i as f64;
//////    let t = if f as u128 > i || i == u128::max_value() {
//////        f64::from_bits(f.to_bits() - 1)
//////    } else {
//////        f
//////    };
////    assert_eq!(soft::u128_to_f64_round(i), f.to_bits());
//////    assert_eq!(soft::u128_to_f64_truncate(i), t.to_bits());
////    assert_eq!(fast::u128_to_f64_round(i), f);
//////    assert_eq!(fast::u128_to_f64_truncate(i), t);
////}
////
////
////// cargo kani  2,11s user 0,20s system 100% cpu 2,301 total
////#[kani::proof]
////fn test_formal_i32() {
////
////    let i = kani::any();
////    assert_eq!(soft::i32_to_f64(i), (i as f64).to_bits());
////    assert_eq!(fast::i32_to_f64(i), i as f64);
////    assert_eq!(soft::i32_to_f32_round(i), (i as f32).to_bits());
////    assert_eq!(fast::i32_to_f32_round(i), i as f32);
////}
////
////// cargo kani  8,31s user 0,24s system 99% cpu 8,571 total
////#[kani::proof]
////fn test_formal_i64() {
////
////    let i = kani::any();
////    assert_eq!(soft::i64_to_f32_round(i), (i as f32).to_bits());
////    assert_eq!(fast::i64_to_f32_round(i), i as f32);
////    assert_eq!(soft::i64_to_f64_round(i), (i as f64).to_bits());
////    assert_eq!(fast::i64_to_f64_round(i), i as f64);
////}
////
////// cargo kani  481,00s user 1,19s system 98% cpu 8:07,21 total
////#[kani::proof]
////fn test_formal_i128() {
////
////    let i = kani::any();
////    assert_eq!(soft::i128_to_f32_round(i), (i as f32).to_bits());
////    assert_eq!(fast::i128_to_f32_round(i), i as f32);
////    assert_eq!(soft::i128_to_f64_round(i), (i as f64).to_bits());
////    assert_eq!(fast::i128_to_f64_round(i), i as f64);
////}
