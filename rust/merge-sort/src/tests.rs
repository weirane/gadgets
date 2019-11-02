extern crate rand;
extern crate test;

use super::*;

macro_rules! assert_sort {
    ($origin:expr => $sorted:expr) => {{
        let mut array = $origin;
        merge_sort(&mut array);
        assert_eq!(array, $sorted);
    }};
}

#[test]
fn test_empty() {
    let mut v = vec![0];
    v.pop();
    let mut v2 = vec![1];
    v2.pop();
    assert_sort!(v => v2);
}

#[test]
fn test_duplicate() {
    assert_sort!([3, 1, 2, 3, 2, 3] => [1, 2, 2, 3, 3, 3]);
}

#[test]
fn test_one_two() {
    assert_sort!([3.14159] => [3.14159]);
    assert_sort!([3, 2] => [2, 3]);
}

#[test]
fn test_basic() {
    assert_sort!([3, -3, 1, 0, -2, -1, 2] => [-3, -2, -1, 0, 1, 2, 3]);
    assert_sort!([8, 4, 5, 7, 1, 3, 6, 2] => [1, 2, 3, 4, 5, 6, 7, 8]);
    assert_sort!([9, 8, 7, 6, 5, 4, 3, 2, 1] => [1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[bench]
fn bench_merge_sort(b: &mut test::Bencher) {
    const HOW_MANY: usize = 200;

    let mut arr: Vec<_> = (0..HOW_MANY).map(|_| rand::random::<i32>()).collect();
    let mut correct = arr.clone();
    correct.sort();

    b.iter(|| merge_sort(&mut arr));

    assert!((0..HOW_MANY).all(|i| arr[i] == correct[i]));
}
