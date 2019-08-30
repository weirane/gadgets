//! This crate provides a merge sort function.

#![feature(test)]

fn merge<T>(array: &mut [T], start: usize, mid: usize, end: usize, space: &mut Vec<T>)
where
    T: Copy + PartialOrd,
{
    let mut left = start;
    let mut right = mid;
    let mut i = start;
    while left < mid && right < end {
        if array[left] <= array[right] {
            space[i] = array[left];
            left += 1;
        } else {
            space[i] = array[right];
            right += 1;
        }
        i += 1;
    }
    if left < mid {
        (left..mid).for_each(|j| {
            space[i] = array[j];
            i += 1;
        });
    } else {
        (right..end).for_each(|j| {
            space[i] = array[j];
            i += 1;
        });
    }

    (start..end).for_each(|i| array[i] = space[i]);
}

/// Inner function used by `merge_sort`. Note that `array[end]` is not included
/// in the elements to be sorted.
fn merge_inner<T>(array: &mut [T], start: usize, end: usize, space: &mut Vec<T>)
where
    T: Copy + PartialOrd,
{
    if start + 1 >= end {
        return;
    }
    let mid = start + (end - start) / 2;
    merge_inner(array, start, mid, space);
    merge_inner(array, mid, end, space);
    merge(array, start, mid, end, space);
}

/// Merge sort algorithm.
pub fn merge_sort<T>(array: &mut [T])
where
    T: Copy + PartialOrd,
{
    let len = array.len();
    if len < 2 {
        return;
    }
    let mut space = vec![array[0]; len];
    merge_inner(array, 0, len, &mut space);
}

#[cfg(test)]
mod tests;
