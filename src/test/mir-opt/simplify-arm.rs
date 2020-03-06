// compile-flags: -Z mir-opt-level=1

fn id(o: Option<u8>) -> Option<u8> {
    match o {
        Some(v) => Some(v),
        None => None,
    }
}

fn id_result(r: Result<u8, i32>) -> Result<u8, i32> {
    match r {
        Ok(x) => Ok(x),
        Err(y) => Err(y),
    }
}

fn id_try(r: Result<u8, i32>) -> Result<u8, i32> {
    let x = r?;
    Ok(x)
}

fn main() {
    id(None);
    id_result(Ok(4));
    id_try(Ok(4));
}

// END RUST SOURCE
// START rustc.id.SimplifyArmIdentity.before.mir
// debug o => _1;
// let mut _0: std::option::Option<u8>;
// let mut _2: isize;
// let _3: u8;
// let mut _4: u8;
// scope 1 {
//   debug v => _3;
// }
// bb0: {
//   _2 = discriminant(_1);
//   switchInt(move _2) -> [0isize: bb1, 1isize: bb3, otherwise: bb2];
// }
// bb1: {
//   discriminant(_0) = 0;
//   goto -> bb4;
// }
// bb2: {
//   unreachable;
// }
// bb3: {
//   StorageLive(_3);
//   _3 = ((_1 as Some).0: u8);
//   StorageLive(_4);
//   _4 = _3;
//   ((_0 as Some).0: u8) = move _4;
//   discriminant(_0) = 1;
//   StorageDead(_4);
//   StorageDead(_3);
//   goto -> bb4;
// }
// bb4: {
//   return;
// }
// END rustc.id.SimplifyArmIdentity.before.mir
// START rustc.id.SimplifyArmIdentity.after.mir
// debug o => _1;
// let mut _0: std::option::Option<u8>;
// let mut _2: isize;
// let _3: u8;
// let mut _4: u8;
// scope 1 {
//   debug v => _3;
// }
// bb0: {
//   _2 = discriminant(_1);
//   switchInt(move _2) -> [0isize: bb1, 1isize: bb3, otherwise: bb2];
// }
// bb1: {
//   discriminant(_0) = 0;
//   goto -> bb4;
// }
// bb2: {
//   unreachable;
// }
// bb3: {
//   _0 = move _1;
//   goto -> bb4;
// }
// bb4: {
//   return;
// }
// END rustc.id.SimplifyArmIdentity.after.mir

// START rustc.id_result.SimplifyArmIdentity.before.mir
// debug r => _1;
// let mut _0: std::result::Result<u8, i32>;
// let mut _2: isize;
// let _3: u8;
// let mut _4: u8;
// let _5: i32;
// let mut _6: i32;
// scope 1 {
//   debug x => _3;
// }
// scope 2 {
//   debug y => _5;
// }
// bb0: {
//   _2 = discriminant(_1);
//   switchInt(move _2) -> [0isize: bb3, 1isize: bb1, otherwise: bb2];
// }
// bb1: {
//   StorageLive(_5);
//   _5 = ((_1 as Err).0: i32);
//   StorageLive(_6);
//   _6 = _5;
//   ((_0 as Err).0: i32) = move _6;
//   discriminant(_0) = 1;
//   StorageDead(_6);
//   StorageDead(_5);
//   goto -> bb4;
// }
// bb2: {
//   unreachable;
// }
// bb3: {
//   StorageLive(_3);
//   _3 = ((_1 as Ok).0: u8);
//   StorageLive(_4);
//   _4 = _3;
//   ((_0 as Ok).0: u8) = move _4;
//   discriminant(_0) = 0;
//   StorageDead(_4);
//   StorageDead(_3);
//   goto -> bb4;
// }
// bb4: {
//   return;
// }
// END rustc.id_result.SimplifyArmIdentity.before.mir
// START rustc.id_result.SimplifyArmIdentity.after.mir
// debug r => _1;
// let mut _0: std::result::Result<u8, i32>;
// let mut _2: isize;
// let _3: u8;
// let mut _4: u8;
// let _5: i32;
// let mut _6: i32;
// scope 1 {
//   debug x => _3;
// }
// scope 2 {
//   debug y => _5;
// }
// bb0: {
//   _2 = discriminant(_1);
//   switchInt(move _2) -> [0isize: bb3, 1isize: bb1, otherwise: bb2];
// }
// bb1: {
//   _0 = move _1;
//   goto -> bb4;
// }
// bb2: {
//   unreachable;
// }
// bb3: {
//   _0 = move _1;
//   goto -> bb4;
// }
// bb4: {
//   return;
// }
// END rustc.id_result.SimplifyArmIdentity.after.mir
// START rustc.id_result.SimplifyBranchSame.before.mir
// debug r => _1;
// let mut _0: std::result::Result<u8, i32>;
// let mut _2: isize;
// let _3: u8;
// let mut _4: u8;
// let _5: i32;
// let mut _6: i32;
// scope 1 {
//   debug x => _3;
// }
// scope 2 {
//   debug y => _5;
// }
// bb0: {
//   _2 = discriminant(_1);
//   switchInt(move _2) -> [0isize: bb3, 1isize: bb1, otherwise: bb2];
// }
// bb1: {
//   _0 = move _1;
//   goto -> bb4;
// }
// bb2: {
//   unreachable;
// }
// bb3: {
//   _0 = move _1;
//   goto -> bb4;
// }
// bb4: {
//   return;
// }
// END rustc.id_result.SimplifyBranchSame.before.mir
// START rustc.id_result.SimplifyBranchSame.after.mir
// debug r => _1;
// let mut _0: std::result::Result<u8, i32>;
// let mut _2: isize;
// let _3: u8;
// let mut _4: u8;
// let _5: i32;
// let mut _6: i32;
// scope 1 {
//   debug x => _3;
// }
// scope 2 {
//   debug y => _5;
// }
// bb0: {
//   _2 = discriminant(_1);
//   goto -> bb1;
// }
// bb1: {
//   _0 = move _1;
//   goto -> bb2;
// }
// bb2: {
//   return;
// }
// END rustc.id_result.SimplifyBranchSame.after.mir

// START rustc.id_try.SimplifyArmIdentity.before.mir
// debug r => _1;
// let mut _0: std::result::Result<u8, i32>;
// let _2: u8;
// let mut _3: std::result::Result<u8, i32>;
// let mut _4: std::result::Result<u8, i32>;
// let mut _5: isize;
// let _6: i32;
// let mut _7: !;
// let mut _8: i32;
// let mut _9: i32;
// let _10: u8;
// let mut _11: u8;
// scope 1 {
//   debug x => _2;
// }
// scope 2 {
//   debug err => _6;
//   scope 3 {
//   }
// }
// scope 4 {
//   debug val => _10;
//   scope 5 {
//   }
// }
// bb0: {
//   StorageLive(_2);
//   StorageLive(_3);
//   StorageLive(_4);
//   _4 = _1;
//   _3 = const <std::result::Result<u8, i32> as std::ops::Try>::into_result(move _4) -> bb1;
// }
// bb1: {
//   StorageDead(_4);
//   _5 = discriminant(_3);
//   switchInt(move _5) -> [0isize: bb2, 1isize: bb4, otherwise: bb3];
// }
// bb2: {
//   StorageLive(_10);
//   _10 = ((_3 as Ok).0: u8);
//   _2 = _10;
//   StorageDead(_10);
//   StorageDead(_3);
//   StorageLive(_11);
//   _11 = _2;
//   ((_0 as Ok).0: u8) = move_ 11;
//   discriminant(_0) = 0;
//   StorageDead(_11);
//   StorageDead(_2);
//   goto -> bb5;
// }
// bb3: {
//   unreachable;
// }
// bb4: {
//   StorageLive(_6);
//   _6 = ((_3 as Err).0: i32);
//   StorageLive(_8);
//   StorageLive(_9);
//   _9 = _6;
//   _8 = const <i32 as std::convert::From<i32>>::from(move _9) -> bb6;
// }
// bb5: {
//   return;
// }
// bb6: {
//   StorageDead(_9);
//   _0 = const <std::result::Result<u8, i32> as std::ops::Try>::from_error(move _8) -> bb7;
// }
// bb7: {
//   StorageDead(_8);
//   StorageDead(_6);
//   StorageDead(_3);
//   StorageDead(_2);
//   goto -> bb5;
// }
// END rustc.id_try.SimplifyArmIdentity.before.mir
// START rustc.id_try.SimplifyArmIdentity.after.mir
// debug r => _1;
// let mut _0: std::result::Result<u8, i32>;
// let _2: u8;
// let mut _3: std::result::Result<u8, i32>;
// let mut _4: std::result::Result<u8, i32>;
// let mut _5: isize;
// let _6: i32;
// let mut _7: !;
// let mut _8: i32;
// let mut _9: i32;
// let _10: u8;
// let mut _11: u8;
// scope 1 {
//   debug x => _2;
// }
// scope 2 {
//   debug err => _6;
//   scope 3 {
//   }
// }
// scope 4 {
//   debug val => _10;
//   scope 5 {
//   }
// }
// bb0: {
//   StorageLive(_2);
//   StorageLive(_3);
//   StorageLive(_4);
//   _4 = _1;
//   _3 = const <std::result::Result<u8, i32> as std::ops::Try>::into_result(move _4) -> bb1;
// }
// bb1: {
//   StorageDead(_4);
//   _5 = discriminant(_3);
//   switchInt(move _5) -> [0isize: bb2, 1isize: bb4, otherwise: bb3];
// }
// bb2: {
//   _0 = move _3;
//   StorageDead(_3);
//   StorageDead(_2);
//   goto -> bb5;
// }
// bb3: {
//   unreachable;
// }
// bb4: {
//   StorageLive(_6);
//   _6 = ((_3 as Err).0: i32);
//   StorageLive(_8);
//   StorageLive(_9);
//   _9 = _6;
//   _8 = const <i32 as std::convert::From<i32>>::from(move _9) -> bb6;
// }
// bb5: {
//   return;
// }
// bb6: {
//   StorageDead(_9);
//   _0 = const <std::result::Result<u8, i32> as std::ops::Try>::from_error(move _8) -> bb7;
// }
// bb7: {
//   StorageDead(_8);
//   StorageDead(_6);
//   StorageDead(_3);
//   StorageDead(_2);
//   goto -> bb5;
// }
// END rustc.id_try.SimplifyArmIdentity.after.mir
