use work_assign::work_assign;

fn main() {
    let cost = vec![
        vec![12, 7, 9, 7, 9],
        vec![8, 9, 6, 6, 6],
        vec![7, 17, 12, 14, 9],
        vec![15, 14, 6, 6, 10],
        vec![4, 10, 7, 10, 9],
    ];
    let (cost, assign) = work_assign(&cost);
    println!("cost = {}", cost);
    println!("assign = {:?}", assign);
}
