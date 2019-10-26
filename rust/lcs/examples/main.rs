use lcs::lcs;
use std::str;

fn main() {
    let strx = b"HIEROGLYPHOLOGY";
    let stry = b"MICHAELANGELO";

    println!("{}", str::from_utf8(&lcs(strx, stry)).unwrap());
}
