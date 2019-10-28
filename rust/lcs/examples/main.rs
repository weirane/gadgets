use lcs::lcs;
use std::str;

fn main() {
    let strx = b"HIEROGLYPHOLOGY";
    let stry = b"MICHAELANGELO";
    println!("{}", str::from_utf8(&lcs(strx, stry)).unwrap());

    let xs = [0, 1, 2, 1, 3, 0, 1];
    let ys = [1, 3, 2, 0, 1, 0];
    println!("{:?}", lcs(&xs, &ys));
}
