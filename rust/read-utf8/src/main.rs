use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::{env, str};

const BUF_SIZE: usize = 1024;

fn main() -> io::Result<()> {
    let path = env::args().nth(1).expect("argv[1] is the file to read");
    let mut file = File::open(path)?;
    let mut buf = [0u8; BUF_SIZE];

    loop {
        match file.read(&mut buf)? {
            0 => break,
            n if n > 0 => match str::from_utf8(&buf[..n]) {
                Ok(stuff) => print!("{}", stuff),
                Err(e) => {
                    let index = e.valid_up_to();
                    let padding = index as i64 - n as i64;
                    if -padding >= 4 {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "invalid UTF-8",
                        ));
                    }
                    print!("{}", str::from_utf8(&buf[..index]).unwrap());
                    file.seek(SeekFrom::Current(padding))?;
                }
            },
            _ => unreachable!(),
        }
    }

    Ok(())
}
