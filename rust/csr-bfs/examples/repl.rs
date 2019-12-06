use csr_bfs::CsrGraph;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::num::ParseIntError;

#[derive(Debug)]
enum WorkErrors {
    IntError(ParseIntError),
    InvalidCommand(String),
    ArgError,
    InvalidNode,
}

impl fmt::Display for WorkErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IntError(e) => e.fmt(f),
            Self::InvalidCommand(s) => write!(f, "Invalid command {}", s),
            Self::ArgError => write!(f, "Invalid argument"),
            Self::InvalidNode => write!(f, "Invalid node"),
        }
    }
}

impl From<ParseIntError> for WorkErrors {
    fn from(err: ParseIntError) -> Self {
        Self::IntError(err)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = std::env::args().nth(1).unwrap_or("-".to_string());
    let g = if data == "-" {
        read(io::stdin().lock())
    } else {
        let data = File::open(data)?;
        let data = BufReader::new(data);
        read(data)
    };
    let g = g?;

    loop {
        let mut buf = String::new();
        print!("> ");
        io::stdout().flush()?;
        match io::stdin().read_line(&mut buf)? {
            0 => {
                println!();
                break Ok(());
            }
            1 => continue,
            _ => {
                if let Err(e) = work(&g, &buf) {
                    eprintln!("{}", e);
                    if let WorkErrors::InvalidCommand(_) = e {
                        eprintln!("Available: help, bid, bfs");
                    }
                }
            }
        }
    }
}

fn read(mut stream: impl BufRead) -> Result<CsrGraph, Box<dyn Error>> {
    let mut node_cnt = String::new();
    stream.read_line(&mut node_cnt)?;

    let node_cnt: usize = node_cnt.trim().parse()?;
    let mut g = CsrGraph::with_nodes(node_cnt);
    for li in stream.lines() {
        let li = li?;
        let ns: Vec<_> = li.split(' ').collect();
        let n0: usize = ns[0].parse()?;
        let n1: usize = ns[1].parse()?;
        g.add_edge(n0, n1, 1);
    }
    Ok(g)
}

fn work(g: &CsrGraph, input: &str) -> Result<(), WorkErrors> {
    let cmds: Vec<_> = input.trim().split_ascii_whitespace().collect();
    match cmds[0] {
        "?" | "help" => {
            println!("help bid bfs");
            Ok(())
        }
        "bid" => {
            let src: usize = cmds.get(1).ok_or(WorkErrors::ArgError)?.parse()?;
            let dst: usize = cmds.get(2).ok_or(WorkErrors::ArgError)?.parse()?;
            if src < g.node_count() && dst < g.node_count() {
                println!("{:?}", g.bidir_bfs(src, dst));
                Ok(())
            } else {
                Err(WorkErrors::InvalidNode)
            }
        }
        "bfs" => {
            let src: usize = cmds.get(1).ok_or(WorkErrors::ArgError)?.parse()?;
            if src < g.node_count() {
                g.bfs(src, |n| print!("{} ", n));
                println!();
                Ok(())
            } else {
                Err(WorkErrors::InvalidNode)
            }
        }
        s => Err(WorkErrors::InvalidCommand(s.to_string())),
    }
}
