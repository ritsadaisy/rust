// compile-flags: -Zmir-opt-level=1
// ignore-tidy-linelength

use std::ptr::NonNull;

pub struct LinkedList {
    head: Option<NonNull<Node>>,
    tail: Option<NonNull<Node>>,
}

pub struct Node {
    next: Option<NonNull<Node>>,
}

impl LinkedList {
    pub fn new() -> Self {
        Self { head: None, tail: None }
    }

    pub fn append(&mut self, other: &mut Self) {
        match self.tail {
            None => { },
            Some(mut tail) => {
                // `as_mut` is okay here because we have exclusive access to the entirety
                // of both lists.
                if let Some(other_head) = other.head.take() {
                    unsafe {
                        tail.as_mut().next = Some(other_head);
                    }
                }
            }
        }
    }
}

fn main() {
    let mut one = LinkedList::new();
    let mut two = LinkedList::new();
    one.append(&mut two);
}

// END RUST SOURCE
// START rustc.{{impl}}-append.SimplifyArmIdentity.before.mir
// debug self => _1;
// debug other => _2;
// let mut _0: ();
// let mut _3: isize;
// let mut _4: std::ptr::NonNull<Node>;
// let mut _5: std::option::Option<std::ptr::NonNull<Node>>;
// let mut _6: &mut std::option::Option<std::ptr::NonNull<Node>>;
// let mut _7: isize;
// let mut _9: std::option::Option<std::ptr::NonNull<Node>>;
// let mut _10: std::ptr::NonNull<Node>;
// let mut _11: &mut Node;
// let mut _12: &mut std::ptr::NonNull<Node>;
// scope 1 {
//     debug tail => _4;
//     let _8: std::ptr::NonNull<Node>;
//     scope 2 {
//         debug other_head => _8;
//         scope 3 {
//         }
//     }
// }
// bb0: {
//     _3 = discriminant(((*_1).1: std::option::Option<std::ptr::NonNull<Node>>));
//     switchInt(move _3) -> [0isize: bb3, 1isize: bb1, otherwise: bb2];
// }
// bb1: {
//     StorageLive(_4);
//     _4 = ((((*_1).1: std::option::Option<std::ptr::NonNull<Node>>) as Some).0: std::ptr::NonNull<Node>);
//     StorageLive(_5);
//     StorageLive(_6);
//     _6 = &mut ((*_2).0: std::option::Option<std::ptr::NonNull<Node>>);
//     _5 = const std::option::Option::<std::ptr::NonNull<Node>>::take(move _6) -> bb4;
// }
// bb2: {
//     unreachable;
// }
// bb3: {
//     nop;
//     goto -> bb9;
// }
// bb4: {
//     StorageDead(_6);
//     _7 = discriminant(_5);
//     switchInt(move _7) -> [1isize: bb6, otherwise: bb5];
// }
// bb5: {
//     nop;
//     goto -> bb8;
// }
// bb6: {
//     StorageLive(_8);
//     _8 = ((_5 as Some).0: std::ptr::NonNull<Node>);
//     StorageLive(_9);
//     StorageLive(_10);
//     _10 = _8;
//     ((_9 as Some).0: std::ptr::NonNull<Node>) = move _10;
//     discriminant(_9) = 1;
//     StorageDead(_10);
//     StorageLive(_11);
//     StorageLive(_12);
//     _12 = &mut _4;
//     _11 = const std::ptr::NonNull::<Node>::as_mut(move _12) -> bb7;
// }
// bb7: {
//     StorageDead(_12);
//     ((*_11).0: std::option::Option<std::ptr::NonNull<Node>>) = move _9;
//     StorageDead(_9);
//     StorageDead(_11);
//     nop;
//     StorageDead(_8);
//     goto -> bb8;
// }
// bb8: {
//     StorageDead(_5);
//     StorageDead(_4);
//     goto -> bb9;
// }
// bb9: {
//     return;
// }
// END rustc.{{impl}}-append.SimplifyArmIdentity.before.mir

// START rustc.{{impl}}-append.SimplifyArmIdentity.after.mir
// debug self => _1;
// debug other => _2;
// let mut _0: ();
// let mut _3: isize;
// let mut _4: std::ptr::NonNull<Node>;
// let mut _5: std::option::Option<std::ptr::NonNull<Node>>;
// let mut _6: &mut std::option::Option<std::ptr::NonNull<Node>>;
// let mut _7: isize;
// let mut _9: std::option::Option<std::ptr::NonNull<Node>>;
// let mut _10: std::ptr::NonNull<Node>;
// let mut _11: &mut Node;
// let mut _12: &mut std::ptr::NonNull<Node>;
// scope 1 {
//     debug tail => _4;
//     let _8: std::ptr::NonNull<Node>;
//     scope 2 {
//         debug other_head => _8;
//         scope 3 {
//         }
//     }
// }
// bb0: {
//     _3 = discriminant(((*_1).1: std::option::Option<std::ptr::NonNull<Node>>));
//     switchInt(move _3) -> [0isize: bb3, 1isize: bb1, otherwise: bb2];
// }
// bb1: {
//     StorageLive(_4);
//     _4 = ((((*_1).1: std::option::Option<std::ptr::NonNull<Node>>) as Some).0: std::ptr::NonNull<Node>);
//     StorageLive(_5);
//     StorageLive(_6);
//     _6 = &mut ((*_2).0: std::option::Option<std::ptr::NonNull<Node>>);
//     _5 = const std::option::Option::<std::ptr::NonNull<Node>>::take(move _6) -> bb4;
// }
// bb2: {
//     unreachable;
// }
// bb3: {
//     nop;
//     goto -> bb9;
// }
// bb4: {
//     StorageDead(_6);
//     _7 = discriminant(_5);
//     switchInt(move _7) -> [1isize: bb6, otherwise: bb5];
// }
// bb5: {
//     nop;
//     goto -> bb8;
// }
// bb6: {
//     StorageLive(_8);
//     _8 = ((_5 as Some).0: std::ptr::NonNull<Node>);
//     StorageLive(_9);
//     _9 = move _5;
//     StorageLive(_11);
//     StorageLive(_12);
//     _12 = &mut _4;
//     _11 = const std::ptr::NonNull::<Node>::as_mut(move _12) -> bb7;
// }
// bb7: {
//     StorageDead(_12);
//     ((*_11).0: std::option::Option<std::ptr::NonNull<Node>>) = move _9;
//     StorageDead(_9);
//     StorageDead(_11);
//     nop;
//     StorageDead(_8);
//     goto -> bb8;
// }
// bb8: {
//     StorageDead(_5);
//     StorageDead(_4);
//     goto -> bb9;
// }
// bb9: {
//     return;
// }
// END rustc.{{impl}}-append.SimplifyArmIdentity.after.mir
