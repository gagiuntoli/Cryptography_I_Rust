use std::fs;
use std::env;
use sha2::{Sha256, Digest};

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {

    let args: Vec<String> = env::args().collect();
    let filename = args[1].clone();
    const BLOCK_SIZE: usize = 1024;
    let bytes = fs::read(filename)?;
    let num_blocks = (bytes.len() + BLOCK_SIZE - 1) / BLOCK_SIZE;
    let last_block_size = bytes.len() % BLOCK_SIZE;

    println!("video length = {} bytes", bytes.len());
    println!("number of blocks = {}", num_blocks);
    println!("last block size = {}", bytes.len() % BLOCK_SIZE);

    let mut last_block = vec![0u8; last_block_size];
    for j in 0..last_block_size {
        last_block[j] = bytes[((num_blocks-1)*BLOCK_SIZE) + j];
    }

    let mut hasher = Sha256::new();
    hasher.update(last_block);
    let mut hash: Vec<u8> = hasher.finalize().into_iter().collect();

    for i in (0..(num_blocks-1)).rev() {

        let mut block = [0u8; (1024 + 32)];
        for j in 0..1024 {
            block[j] = bytes[(i*BLOCK_SIZE) + j];
        }
        for j in 0..32 {
            block[1024+j] = hash[j];
        }

        let mut hasher = Sha256::new();
        hasher.update(block);
        hash = hasher.finalize().into_iter().collect();
    }

    println!("{}", hash.iter().map(|n| format!("{:02x}", n)).fold(String::new(), |acc, arg| acc + arg.as_str()));

    Ok(())
}