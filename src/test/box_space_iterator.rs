use crate::implm::point::boxy::{BoxCoordinateSpace, CoordinateTuplet};

#[test]
fn test_zero_dimensional() {
    let space = BoxCoordinateSpace::new([]);

    let mut iter = space.iter();

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);  // Check FusedIterator contract
}

#[test]
fn test_one_dimensional() {
    let space = BoxCoordinateSpace::new([9]);

    let mut iter = space.iter();

    assert_eq!(iter.next(), Some(CoordinateTuplet::from([0])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([1])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([2])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([3])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([4])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([5])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([6])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([7])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([8])));

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);  // Check FusedIterator contract
}

#[test]
fn test_two_dimensional() {
    let space = BoxCoordinateSpace::new([4, 4]);

    let mut iter = space.iter();

    assert_eq!(iter.next(), Some(CoordinateTuplet::from([0, 0])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([1, 0])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([2, 0])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([3, 0])));

    assert_eq!(iter.next(), Some(CoordinateTuplet::from([0, 1])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([1, 1])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([2, 1])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([3, 1])));

    assert_eq!(iter.next(), Some(CoordinateTuplet::from([0, 2])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([1, 2])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([2, 2])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([3, 2])));

    assert_eq!(iter.next(), Some(CoordinateTuplet::from([0, 3])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([1, 3])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([2, 3])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([3, 3])));

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);  // Check FusedIterator contract
}

#[test]
fn test_three_dimensional() {
    let space = BoxCoordinateSpace::new([3, 3, 3]);

    let mut iter = space.iter();

    assert_eq!(iter.next(), Some(CoordinateTuplet::from([0, 0, 0])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([1, 0, 0])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([2, 0, 0])));

    assert_eq!(iter.next(), Some(CoordinateTuplet::from([0, 1, 0])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([1, 1, 0])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([2, 1, 0])));

    assert_eq!(iter.next(), Some(CoordinateTuplet::from([0, 2, 0])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([1, 2, 0])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([2, 2, 0])));

    assert_eq!(iter.next(), Some(CoordinateTuplet::from([0, 0, 1])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([1, 0, 1])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([2, 0, 1])));

    assert_eq!(iter.next(), Some(CoordinateTuplet::from([0, 1, 1])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([1, 1, 1])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([2, 1, 1])));

    assert_eq!(iter.next(), Some(CoordinateTuplet::from([0, 2, 1])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([1, 2, 1])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([2, 2, 1])));

    assert_eq!(iter.next(), Some(CoordinateTuplet::from([0, 0, 2])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([1, 0, 2])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([2, 0, 2])));

    assert_eq!(iter.next(), Some(CoordinateTuplet::from([0, 1, 2])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([1, 1, 2])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([2, 1, 2])));

    assert_eq!(iter.next(), Some(CoordinateTuplet::from([0, 2, 2])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([1, 2, 2])));
    assert_eq!(iter.next(), Some(CoordinateTuplet::from([2, 2, 2])));

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);  // Check FusedIterator contract
}