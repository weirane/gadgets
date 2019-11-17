use lcs::lcs;
use std::io;

fn main() -> io::Result<()> {
    let mut xs = String::new();
    io::stdin().read_line(&mut xs)?;
    let xs = xs.trim_end_matches('\n');

    let mut ys = String::new();
    io::stdin().read_line(&mut ys)?;
    let ys = ys.trim_end_matches('\n');

    let ret = lcs(&xs.as_bytes(), &ys.as_bytes());
    println!("{}", String::from_utf8_lossy(&ret));

    Ok(())
}
