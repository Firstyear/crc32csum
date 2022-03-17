use std::fs::File;
use std::io::{BufRead, BufReader, Seek};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str))]
    /// Path of the file to compute the checksum for
    path: PathBuf,
}

fn main() {
    let Opt { path } = Opt::from_args();

    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open path - {:?}", e);
            return;
        }
    };

    if let Err(e) = file.seek(std::io::SeekFrom::Start(0)) {
        eprintln!("Unable to seek tempfile -> {:?}", e);
        return;
    };

    let mut buf_file = BufReader::with_capacity(8192, file);
    let mut crc = 0;
    loop {
        match buf_file.fill_buf() {
            Ok(buffer) => {
                let length = buffer.len();
                if length == 0 {
                    // We are done!
                    break;
                } else {
                    // we have content, proceed.
                    crc = crc32c::crc32c_append(crc, &buffer);
                    buf_file.consume(length);
                }
            }
            Err(e) => {
                eprintln!("File reader error -> {:?}", e);
                return;
            }
        }
    }
    println!("{:08x}", crc);
}
