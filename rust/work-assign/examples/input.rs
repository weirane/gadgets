use std::error::Error;
use std::io;
use work_assign::work_assign;

fn main() -> Result<(), Box<dyn Error>> {
    let mut costs = Vec::new();
    loop {
        let mut buf = String::new();
        if io::stdin().read_line(&mut buf)? <= 1 {
            break;
        }

        let cost: Vec<_> = buf
            .split_whitespace()
            .map(|s| Ok(s.parse()?))
            .collect::<Result<_, std::num::ParseIntError>>()?;

        costs.push(cost);
    }

    let (cost, assign) = work_assign(&costs);
    println!("cost = {}", cost);
    println!("assign = {:?}", assign);

    Ok(())
}
