extern crate num;
use num_bigint::{ToBigInt, BigInt};
use num::{Integer, One, Zero};
use std::collections::HashMap;

fn modinv(n: &BigInt, p: &BigInt) -> BigInt {
    if p.is_one() { return BigInt::one() }

    let (mut a, mut m, mut x, mut inv) = (n.clone(), p.clone(), BigInt::zero(), BigInt::one());

    while a > BigInt::one() {
        let (div, rem) = a.div_rem(&m);
        inv -= div * &x;
        a = rem;
        std::mem::swap(&mut a, &mut m);
        std::mem::swap(&mut x, &mut inv);
    }
 
    if inv < BigInt::zero() { inv += p }

    inv
}

fn main() {
    const B: u32 = 1 << 20;
    let p = BigInt::parse_bytes(b"13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084171", 10).unwrap();
    let g = BigInt::parse_bytes(b"11717829880366207009516117596335367088558084999998952205599979459063929499736583746670572176471460312928594829675428279466566527115212748467589894601965568", 10).unwrap();
    let h = BigInt::parse_bytes(b"3239475104050450443565264378728065788649097520952449527834792452971981976143292558073856937958553180532878928001494706097394108577585732452307673444020333", 10).unwrap();
    let mut left_hand_side = HashMap::<BigInt, u32>::new();

    for x1 in 0..=B {
        // h / G^x in Zp
        let g_pow_x = g.modpow(&x1.to_bigint().unwrap(), &p);
        let g_pow_x_inv = modinv(&g_pow_x, &p);
        let h_times_g_pow_x_inv = (g_pow_x_inv * &h) % &p;
        if x1 % 100000 == 0 {
            println!("x = {:#022b}", &x1);
        }
        left_hand_side.insert(h_times_g_pow_x_inv, x1);
    }

    for x0 in 0..=B {
        // G^(B*x) in Zp
        let bx = x0.to_bigint().unwrap() * B.to_bigint().unwrap();
        let g_pow_bx = g.modpow(&bx, &p);
        match left_hand_side.get(&g_pow_bx) {
            Some(x1) => {
                println!("sol = {}", x0.to_bigint().unwrap() * B.to_bigint().unwrap() + x1.to_bigint().unwrap());
                break;
            },
            None => {}
        }
    }

}