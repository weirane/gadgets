#![feature(test)]
extern crate test;

use std::u32::MAX as U32_MAX;

/// Returns the minimal cost and a vector of `vec[i] == j` indicating the
/// i-th person gets the j-th job. Indexes starts at 0.
pub fn work_assign(costs: &[Vec<u32>]) -> (u32, Vec<usize>) {
    let num = costs.len();

    if num == 0 {
        return (0, Vec::new());
    }
    if costs.iter().any(|v| v.len() != num) {
        panic!("Invalid input: costs is not a square matrix");
    }

    let mut assign: Vec<_> = (0..num).collect();
    let mut cost = U32_MAX;
    work_assign_inner(num, costs, 0, &mut assign.clone(), &mut cost, &mut assign);
    (cost, assign)
}

fn work_assign_inner(
    n: usize,
    costs: &[Vec<u32>],
    curr: usize,
    curr_assign: &mut Vec<usize>,
    cost: &mut u32,
    assign: &mut Vec<usize>,
) {
    if curr >= n {
        match calc_cost(n, curr_assign, costs) {
            c if c < *cost => {
                *assign = curr_assign.clone();
                *cost = c;
            }
            _ => (),
        }
        return;
    }

    for i in curr..n {
        curr_assign.swap(curr, i);
        if calc_cost(curr + 1, curr_assign, costs) < *cost {
            work_assign_inner(n, costs, curr + 1, curr_assign, cost, assign);
        }
        curr_assign.swap(curr, i);
    }
}

fn calc_cost(num: usize, assign: &[usize], costs: &[Vec<u32>]) -> u32 {
    (0..num).map(|i| costs[i][assign[i]]).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_wa {
        ($costs:expr, $cost:expr, $assign:expr) => {
            let (cost, assign) = work_assign(&$costs);
            assert_eq!(cost, $cost);
            assert_eq!(assign, $assign);
        };
    }

    #[test]
    fn wa() {
        assert_wa!(
            vec![
                vec![12, 7, 9, 7, 9],
                vec![8, 9, 6, 6, 6],
                vec![7, 17, 12, 14, 9],
                vec![15, 14, 6, 6, 10],
                vec![4, 10, 7, 10, 9],
            ],
            32,
            vec![1, 2, 4, 3, 0]
        );
        assert_wa!(
            vec![
                vec![15, 18, 21, 24],
                vec![19, 23, 22, 18],
                vec![26, 17, 16, 19],
                vec![19, 21, 23, 17],
            ],
            70,
            vec![0, 3, 2, 1]
        );
    }

    #[test]
    fn wa_zero() {
        assert_wa!(Vec::new(), 0, vec![]);
    }

    #[test]
    #[should_panic(expected = "Invalid input: costs is not a square matrix")]
    fn wa_not_square() {
        let cost = vec![
            vec![1, 2],    // lf
            vec![3, 4, 5], // lf
        ];
        work_assign(&cost);
    }

    #[bench]
    fn wa_bench(b: &mut test::Bencher) {
        let costs = vec![
            vec![12, 7, 9, 7, 9],
            vec![8, 9, 6, 6, 6],
            vec![7, 17, 12, 14, 9],
            vec![15, 14, 6, 6, 10],
            vec![4, 10, 7, 10, 9],
        ];
        b.iter(|| work_assign(&costs));
    }
}
