
fn main() {
    let c_dawn = hex::decode("c294814d").unwrap();
    let dawn: Vec<u8> = "dawn".chars().map(|x| x as u8).collect();
    let dusk: Vec<u8> = "dusk".chars().map(|x| x as u8).collect();
    let mut key: Vec<u8> = vec![0; c_dawn.len()];
    let mut c_dusk: Vec<u8> = vec![0; c_dawn.len()];
    for i in 0..key.len() {
        key[i] = c_dawn[i] ^ dawn[i];
    }
    for i in 0..key.len() {
        c_dusk[i] = key[i] ^ dusk[i];
    }
    //println!("{:2x?}", c_dust);
    //println!("{:2x?}", key);
    println!("{:2x?}", c_dusk);
}
