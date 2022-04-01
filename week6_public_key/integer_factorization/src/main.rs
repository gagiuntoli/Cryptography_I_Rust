use num_bigint::{BigInt};
use num::{One, Zero};

fn main() {

    let n1 = BigInt::parse_bytes(b"179769313486231590772930519078902473361797697894230657273430081157732675805505620686985379449212982959585501387537164015710139858647833778606925583497541085196591615128057575940752635007475935288710823649949940771895617054361149474865046711015101563940680527540071584560878577663743040086340742855278549092581", 10).unwrap();
    let (p1, q1) = factor(n1.clone());
    println!("r1 = {}", p1.clone().min(q1.clone()));

    let n2 = BigInt::parse_bytes(b"648455842808071669662824265346772278726343720706976263060439070378797308618081116462714015276061417569195587321840254520655424906719892428844841839353281972988531310511738648965962582821502504990264452100885281673303711142296421027840289307657458645233683357077834689715838646088239640236866252211790085787877", 10).unwrap();
    let (p2, q2) = factor(n2);
    println!("r2 = {}", p2.min(q2));

    let n3 = BigInt::parse_bytes(b"720062263747350425279564435525583738338084451473999841826653057981916355690188337790423408664187663938485175264994017897083524079135686877441155132015188279331812309091996246361896836573643119174094961348524639707885238799396839230364676670221627018353299443241192173812729276147530748597302192751375739387929", 10).unwrap();
    let (p3, q3) = factor_2(n3);
    println!("r3 = {}", p3.min(q3));

    let cyphertext = BigInt::parse_bytes(b"22096451867410381776306561134883418017410069787892831071731839143676135600120538004282329650473509424343946219751512256465839967942889460764542040581564748988013734864120452325229320176487916666402997509188729971690526083222067771600019329260870009579993724077458967773697817571267229951148662959627934791540", 10).unwrap();
    let e: BigInt = BigInt::from(65537);
    let phi_n1 = (&p1 - BigInt::one()) * (&q1 - BigInt::one());
    let d = modinv(e, phi_n1);
    let x = cyphertext.modpow(&d, &n1);
    println!("r4 = {}", std::str::from_utf8(&x.to_bytes_be().1[100..128]).unwrap());
}

fn modinv(a0: BigInt, m0: BigInt) -> BigInt {
    if m0 == BigInt::one() {
        return BigInt::one()
    }
    let (mut a, mut m, mut x0, mut inv) = (a0, m0.clone(), BigInt::zero(), BigInt::one());
    while a > BigInt::one() {
        inv -= &a / &m * &x0;
        a = &a % &m;
        std::mem::swap(&mut a, &mut m);
        std::mem::swap(&mut x0, &mut inv)
    }
    if inv < BigInt::zero() {
        inv += m0
    }
    inv
}

fn factor(n: BigInt) -> (BigInt, BigInt) {
    let mut i = 1;
    loop {
        let a : BigInt = n.sqrt() + i;
        let a_pow2 = num::pow(a.clone(), 2);
        let x_pow2 = a_pow2 - &n;
        let x = x_pow2.sqrt();
        let p = &a - &x;
        let q = &a + &x;
        if &p * &q == n {
            return (p, q)
        }
        i += 1;
    }
}

fn factor_2(n: BigInt) -> (BigInt, BigInt) {

    let a = (BigInt::from(24u32) * &n).sqrt() + BigInt::one();
    let x = (&a * &a - BigInt::from(24u32) * &n).sqrt();
    let p : BigInt = (&a - &x) / BigInt::from(6u32);
    let q : BigInt = (&a + &x) / BigInt::from(4u32);
    assert!(&p * &q == n);
    (p, q)
}