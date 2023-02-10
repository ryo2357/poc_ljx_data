use std::fs::File;
use std::io::{Write,Read,BufReader};
use log::{info};

static DUMMY_DATA: &str = "data/dummy.hex";

static PROFILE_NUM: usize = 3200;
static HEADER_NUM: usize =4 ;
static BYTE_SIZE: usize = 4;

static LOAD_SIZE: usize = BYTE_SIZE *(HEADER_NUM + PROFILE_NUM);

fn main() -> anyhow::Result<()>{
    my_logger::init();
    info!("logger initialized");
    wait_until_enter();

    let mut reader = BufReader::new(File::open(DUMMY_DATA)?);
    info!("create reader successfully");
    wait_until_enter();

    

    Ok(())
}


struct DataReader {
    reader:BufReader<File>,
    // byte_size:usize,
    // profile_num:usize,
    // header_num:usize,
    profile_byte_num:usize,
}

impl DataReader {
    fn create(
        path:&str,
        byte_size:usize,
        profile_num:usize,
        header_num:usize,
        brightness:bool
    ) -> anyhow::Result<Self>{
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        
        let profile_byte_num = match brightness {
            false => byte_size*(profile_num + header_num),
            true => byte_size*(profile_num *2 + header_num)
        };

        Ok(Self{
            reader: reader,
            // byte_size:byte_size,
            // profile_num:profile_num,
            // header_num:header_num,
            profile_byte_num:profile_byte_num,
        })

    }

    fn test(&mut self) -> Vec<u8>{
        let mut buf: Vec<u8> = Vec::new();

        self.reader.read_to_end(&mut buf);

        return buf

    }
}


fn wait_until_enter(){
    print!("wait until press enter: ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}