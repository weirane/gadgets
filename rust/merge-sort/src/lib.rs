//! This crate provides a merge sort function.

#![feature(test)]

use std::ptr;

fn merge<T>(array: &mut [T], start: usize, mid: usize, end: usize)
where
    T: PartialOrd,
{
    let mut left = start;
    let mut right = mid;
    let mut i = 0;
    let mut space = Vec::<T>::with_capacity(end - start);

    unsafe {
        space.set_len(end - start);
    }

    while left < mid && right < end {
        if array[left] <= array[right] {
            unsafe {
                // space[i] = array[left];
                ptr::copy_nonoverlapping(array.as_ptr().add(left), space.as_mut_ptr().add(i), 1);
            }
            left += 1;
        } else {
            unsafe {
                // space[i] = array[right];
                ptr::copy_nonoverlapping(array.as_ptr().add(right), space.as_mut_ptr().add(i), 1);
            }
            right += 1;
        }
        i += 1;
    }

    unsafe {
        if left < mid {
            // space[i..] = array[left..mid]
            ptr::copy_nonoverlapping(
                array.as_ptr().add(left),
                space.as_mut_ptr().add(i),
                mid - left,
            );
        } else {
            // space[i..] = array[right..end]
            ptr::copy_nonoverlapping(
                array.as_ptr().add(right),
                space.as_mut_ptr().add(i),
                end - right,
            );
        }

        // array[start..end] = space[..]
        ptr::copy_nonoverlapping(space.as_ptr(), array.as_mut_ptr().add(start), end - start);
    }
}

/// Inner function used by `merge_sort`. Note that `array[end]` is not included
/// in the elements to be sorted.
fn merge_inner<T>(array: &mut [T], start: usize, end: usize)
where
    T: PartialOrd,
{
    if start + 1 >= end {
        return;
    }
    let mid = start + (end - start) / 2;
    merge_inner(array, start, mid);
    merge_inner(array, mid, end);
    merge(array, start, mid, end);
}

/// Merge sort algorithm.
pub fn merge_sort<T>(array: &mut [T])
where
    T: PartialOrd,
{
    match array.len() {
        0 | 1 => (),
        len => merge_inner(array, 0, len),
    }
}

#[cfg(test)]
mod tests;
