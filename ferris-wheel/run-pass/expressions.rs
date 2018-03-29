// FIXME: Make me pass! Diff budget: 10 lines.
// Do not `use` any items.

// Do not change the following two lines.
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
struct IntWrapper(isize);

use std::cmp::Ordering::*;

fn max<T: PartialOrd<T>>(lhs: T, rhs: T) -> T {
    match lhs.partial_cmp(&rhs).unwrap() {
        Less => rhs,
        _ => lhs
    }
}

pub fn main() {
    assert_eq!(max(1usize, 3), 3);
    assert_eq!(max(1u8, 3), 3);
    assert_eq!(max(1u8, 3), 3);
    assert_eq!(max(IntWrapper(120), IntWrapper(248)), IntWrapper(248));
}
