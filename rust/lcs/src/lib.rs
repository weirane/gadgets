#![feature(test)]
extern crate test;

use std::cmp::max;

/// Checks whether `sub` is a subsequence of `xs`.
pub fn is_sub_seq<T: Eq>(sub: &[T], xs: &[T]) -> bool {
    if sub.is_empty() {
        true
    } else if xs.is_empty() {
        false
    } else if xs[0] == sub[0] {
        is_sub_seq(&sub[1..], &xs[1..])
    } else {
        is_sub_seq(&sub, &xs[1..])
    }
}

/// Finds a longest common subsequence of `xs` and `ys`.
pub fn lcs<T: Eq + Copy>(xs: &[T], ys: &[T]) -> Vec<T> {
    get_lcs(&lcs_table(&xs, &ys), xs.len(), ys.len())
        .iter()
        .map(|&i| xs[i - 1])
        .collect()
}

fn get_lcs(table: &[Vec<usize>], i: usize, j: usize) -> Vec<usize> {
    if i == 0 || j == 0 {
        Vec::new()
    } else if table[i][j] == table[i - 1][j] {
        get_lcs(&table, i - 1, j)
    } else if table[i][j] == table[i][j - 1] {
        get_lcs(&table, i, j - 1)
    } else if table[i][j] == table[i - 1][j - 1] + 1 {
        let mut ret = get_lcs(&table, i - 1, j - 1);
        ret.push(i);
        ret
    } else {
        unreachable!("get_lcs");
    }
}

fn lcs_table<T: Eq>(xs: &[T], ys: &[T]) -> Vec<Vec<usize>> {
    let mut ret = vec![vec![0; ys.len() + 1]; xs.len() + 1];

    for (i, x) in xs.iter().enumerate() {
        let i = i + 1;
        for (j, y) in ys.iter().enumerate() {
            let j = j + 1;
            ret[i][j] = if x == y {
                ret[i - 1][j - 1] + 1
            } else {
                max(ret[i - 1][j], ret[i][j - 1])
            }
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_lcs {
        ($xs:expr, $ys:expr) => {
            let seq = lcs($xs, $ys);
            assert!(is_sub_seq(&seq, $xs));
            assert!(is_sub_seq(&seq, $ys));
        };
    }

    #[test]
    fn is_sub() {
        assert!(is_sub_seq(b"", b"abcde"));
        assert!(is_sub_seq(b"a", b"abcde"));
        assert!(is_sub_seq(b"ade", b"abcde"));
    }

    #[test]
    fn not_is_sub() {
        assert!(!is_sub_seq(b"afe", b"abcde"));
        assert!(!is_sub_seq(b"shorts", b"short"));
    }

    #[test]
    fn test_lcs() {
        assert_lcs!(b"HIEROGLYPHOLOGY", b"MICHAELANGELO");
        assert_lcs!(b"10010101", b"010110110");
    }

    #[bench]
    fn lcs_bench(b: &mut test::Bencher) {
        b.iter(|| lcs(b"HIEROGLYPHOLOGY", b"MICHAELANGELO"));
    }
}
