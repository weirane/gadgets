extern crate merge_sort;
use merge_sort::merge_sort;

fn main() {
    let mut slice = [8, 4, 5, 7, 1, 3, 6, 2];
    merge_sort(&mut slice);
    println!("{:?}", slice);
}
