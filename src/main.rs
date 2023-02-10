use std::fs::File;
use std::io::{Write,Read,BufReader};
use log::{info, debug};

static DUMMY_DATA: &str = "data/dummy.hex";

const PROFILE_NUM: usize = 3200;
const HEADER_NUM: usize =4 ;
const BYTE_SIZE: usize = 4;

const LOAD_SIZE: usize = BYTE_SIZE *(HEADER_NUM + PROFILE_NUM);
// TODO constとstaticの違いについて調べる

fn main() -> anyhow::Result<()>{
    my_logger::init();
    info!("logger initialized");
    wait_until_enter();

    let mut reader = BufReader::new(File::open(DUMMY_DATA)?);
    info!("create reader successfully");
    debug!("create reader successfully");
    wait_until_enter();

    let iter = ProfileBytesIter::new(reader);
    // / iterは一回使うと消費されれる
    let mut count = 0;
    for _ in iter{
        count += 1;

    }
    println!("count: {:?}", count);
    wait_until_enter();

    Ok(())
}
// chatGPTに聞いた
struct ProfileBytesIter<R: Read> {
    reader: BufReader<R>,
    buffer: [u8; LOAD_SIZE],
    bytes_read: usize,
}

impl<R: Read> ProfileBytesIter<R> {
    fn new(reader: BufReader<R>) -> Self {
        ProfileBytesIter {
            reader,
            buffer: [0; LOAD_SIZE],
            bytes_read: 0,
        }
    }
}

impl<R: Read> Iterator for ProfileBytesIter<R> {
    type Item = [u8; LOAD_SIZE];

    fn next(&mut self) -> Option<Self::Item> {
        self.bytes_read = self.reader.read(&mut self.buffer).unwrap();

        if self.bytes_read == 0 {
            None
        } else {
            Some(self.buffer)
        }
    }
}



fn wait_until_enter(){
    print!("wait until press enter: ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}