use aes::Aes128;
use aes::cipher::{
    BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};

fn main() {

    let key = hex::decode("140b41b22a29beb4061bda66b6747e14").unwrap();
    let cyphertext = hex::decode("4ca00ff4c898d61e1edbf1800618fb2828a226d160dad07883d04e008a7897ee2e4b7465d5290d0c0e6c6822236e1daafb94ffe0c5da05d9476be028ad7c1d81").unwrap();
    println!("Message = {}", decrypt_CBC(cyphertext, key));

    let key = hex::decode("140b41b22a29beb4061bda66b6747e14").unwrap();
    let cyphertext = hex::decode("5b68629feb8606f9a6667670b75b38a5b4832d0f26e1ab7da33249de7d4afc48e713ac646ace36e872ad5fb8a512428a6e21364b0c374df45503473c5242a253").unwrap();
    println!("Message = {}", decrypt_CBC(cyphertext, key));

    let key = hex::decode("36f18357be4dbd77f050515c73fcf9f2").unwrap();
    let cyphertext = hex::decode("69dda8455c7dd4254bf353b773304eec0ec7702330098ce7f7520d1cbbb20fc388d1b0adb5054dbd7370849dbf0b88d393f252e764f1f5f7ad97ef79d59ce29f5f51eeca32eabedd9afa9329").unwrap();
    println!("Message = {}", decrypt_CTR(cyphertext, key));

    let key = hex::decode("36f18357be4dbd77f050515c73fcf9f2").unwrap();
    let cyphertext = hex::decode("770b80259ec33beb2561358a9f2dc617e46218c0a53cbeca695ae45faa8952aa0e311bde9d4e01726d3184c34451").unwrap();
    println!("Message = {}", decrypt_CTR(cyphertext, key));
}

fn decrypt_CBC(cyphertext: Vec<u8>, key: Vec<u8>) -> String {

    let mut message = String::new();

    let mut key_aes = GenericArray::from([0u8; 16]);
    let mut block = GenericArray::from([0u8; 16]);
    let mut iv = GenericArray::from([0u8; 16]);

    let num_blocks = cyphertext.len() / 16;

    for i in 0..16 {
        key_aes[i] = key[i];
    }

    let cipher = Aes128::new(&key_aes);

    for b in 1..num_blocks {
        for i in 0..16 {
            iv[i] = cyphertext[(b-1) * 16 + i];
            block[i] = cyphertext[b * 16 + i];
        }
        cipher.decrypt_block(&mut block);
        for i in 0..16 {
            block[i] ^= iv[i];
        }
        message.push_str(std::str::from_utf8(&block).unwrap());
    }

    message
}

fn decrypt_CTR(cyphertext: Vec<u8>, key: Vec<u8>) -> String {

    let mut message = String::new();

    let mut key_aes = GenericArray::from([0u8; 16]);
    let mut block = GenericArray::from([0u8; 16]);
    let mut iv = GenericArray::from([0u8; 16]);

    let num_blocks = (cyphertext.len() + 15) / 16;

    for i in 0..16 {
        key_aes[i] = key[i];
    }

    let cipher = Aes128::new(&key_aes);

    for i in 0..16 {
        iv[i] = cyphertext[i];
    }

    for b in 1..num_blocks {
        for i in 0..16 {
            block[i] = iv[i];
        }
        cipher.encrypt_block(&mut block);
        for i in 0..16 {
            if b * 16 + i < cyphertext.len() {
                block[i] ^= cyphertext[b * 16 + i];
            } else {
                block[i] = 0x20;
            }
        }
        iv[15] += 1;
        message.push_str(std::str::from_utf8(&block).unwrap());
    }

    message
}