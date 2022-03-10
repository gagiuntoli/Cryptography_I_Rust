use reqwest::Error;

const TARGET: &str = "http://crypto-class.appspot.com/po?er=";

fn query(cyphertext: &str) -> bool {
    let request_url = format!("{}{}", TARGET, cyphertext);
    println!("Trying: {}",request_url);
    let resp = reqwest::blocking::get(request_url).unwrap();
    resp.status() == 404
}

fn main() -> Result<(), Error> {
	let cyphertext = hex::decode("f20bdba6ff29eed7b046d1df9fb7000058b1ffb4210a580f748b4ac714c001bd4a61044426fb515dad3f21f18aa577c0bdf302936266926ff37dbf7035d5eeb4").unwrap();
    let nblocks = cyphertext.len() / 16;
    let mut message = vec![0u8; (nblocks-1)*16];

    let mut candidates: Vec<u8> = (65..=91).collect();
    let mut b = (97..=123).collect();
    candidates.append(&mut b);
    candidates.push(32);
    let mut block1 = vec![0u8; 16];
    let mut block2 = vec![0u8; 16];

    for b in 0..(nblocks-2) {
        block1.clone_from_slice(&cyphertext[b*16..(b+1)*16]);
        block2.clone_from_slice(&cyphertext[(b+1)*16..(b+2)*16]);
		for pad in 1..=16 {
			for j in 1..=pad {
				block1[16-j] ^= pad as u8;
            }
            let mut guess: u8 = 0;
            for g in &candidates {
				block1[16-pad] ^= g;
                let block1_s = block1.iter().map(|n| format!("{:02x}", n)).fold(String::new(), |acc, arg| acc + arg.as_str());
                let block2_s = block2.iter().map(|n| format!("{:02x}", n)).fold(String::new(), |acc, arg| acc + arg.as_str());
				if query(&format!("{}{}", block1_s, block2_s)) {
                    guess = *g;
					break;
                }
				block1[16-pad] ^= g;
            }
			for j in 1..=pad {
				block1[16-j] ^= pad as u8;
            }
			message[(b+1)*16 - pad] = guess;
        }
    }

	// Last block must be processed different since it has a valid path
    block1.clone_from_slice(&cyphertext[(nblocks-2)*16..(nblocks-1)*16]);
    block2.clone_from_slice(&cyphertext[(nblocks-1)*16..nblocks*16]);
    let block2_s = block2.iter().map(|n| format!("{:02x}", n)).fold(String::new(), |acc, arg| acc + arg.as_str());
    let mut start_byte = 0;
	for i in 0..16 {
        block1[i] ^= 127;
        let block1_s = block1.iter().map(|n| format!("{:02x}", n)).fold(String::new(), |acc, arg| acc + arg.as_str());
		if query(&format!("{}{}", block1_s, block2_s)) {
            start_byte = i;
        } else {
            block1[i] ^= 127;
            break;
        }
        block1[i] ^= 127;
    }
    let start_pad = 16 - start_byte;
	for j in (start_byte+1)..16 {
		block1[j] ^= start_pad as u8 - 1;
    }

	for pad in start_pad..=16 {
		for j in 0..pad {
			block1[15-j] ^= pad as u8;
        }
        let mut guess = 0;
		for g in &candidates {
			block1[16-pad] ^= g;
            let block1_s = block1.iter().map(|n| format!("{:02x}", n)).fold(String::new(), |acc, arg| acc + arg.as_str());
            let block2_s = block2.iter().map(|n| format!("{:02x}", n)).fold(String::new(), |acc, arg| acc + arg.as_str());
		    if query(&format!("{}{}", block1_s, block2_s)) {
                guess = *g;
				break;
            }
			block1[16-pad] ^= g;
        }
		for j in 0..pad {
			block1[15-j] ^= pad as u8;
        }
		message[(nblocks-1)*16 - pad] = guess;
    }

    println!("message = {}", std::str::from_utf8(&message).unwrap());

    Ok(())
}