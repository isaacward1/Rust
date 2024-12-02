/*
* COP 3515
* Homework #4
*/

use std::fs::File;
use std::io::Write;
use std::io::Read;
use rand::Rng;

fn gcd(a:u32, b:u32) -> u32{
    if b == 0 {
        return a;
    }
    else { return gcd(b, a % b); }
}

fn finde(t: u32) -> u32{
    let mut e:u32 = 2;
    loop {
        if gcd(e, t) == 1 || e > t {
        break;
        }
        e += 1;
    }
    return e;
}

fn findk(e:u32, t:u32) -> u32 {
    let mut k:u32 = 0;

    loop {
        if (1 + (k * t)) % e == 0{
            break;
        }
        k += 1;
    }
    return k;
}

fn smallexp(mut c:u32, mut exp:u32, n:u32) -> u32 {
// decrypted message {m} = (c^2)^(d/2)) % n
let mut m = 1;
while exp > 0 {
    if exp % 2 == 1 {
        m = (m * c) % n
    }
    c = (c * c) % n;
    exp = exp/2;
}
return m;
}

fn prime_2dig() -> u32{
    let mut rand:u32;

    loop {
        rand = rand::thread_rng().gen_range(10..100);
        let mut is_prime = true; // default true
        if rand % 2 == 0 { continue; }

        let sqrt = (rand as f64).sqrt() as u32;
        for x in 3..=sqrt {
            if rand % x == 0 { // check for divisor
                is_prime = false; // if found, not prime
                break;
                }
            }

        if is_prime {break;}
    }
    return rand;
}

fn generate() {

    let p = prime_2dig(); 
    let mut q = prime_2dig(); 

    loop { // check for uniqueness
        if p != q { break; }
        q = prime_2dig();
    }

    println!("p = {}", p);
    println!("q = {}", q);

    let n = p * q; println!("n = {}", n);
    let t = (p - 1) * (q - 1); println!("t = {}", t);

    let e = finde(t); println!("chosen e: {}", e);

    let k = findk(e, t); println!("chosen k: {}", k);
    let d = (1 + (k * t))/ e; println!("d = {}", d);

    println!("public key pair: ({}, {})", e, n);
    println!("private key pair: ({}, {})\n", d, n);

    let mut pub_file = File::create("pub.key").expect("");
    pub_file.write_all(format!("{} {}", e, n).as_bytes()).expect("");

    let mut priv_file = File::create("priv.key").expect("");
    priv_file.write_all(format!("{} {}", d, n).as_bytes()).expect("");
    
}

fn encrypt(secret: String) -> String {
    
    // get pub keys
    let mut file = std::fs::File::open("pub.key").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let parts: Vec<&str> = contents.split(' ').collect();
    let e: u32 = parts[0].parse().expect("");
    let n: u32 = parts[1].parse().expect("");

    let mut enc_ascii = String::new();

    for x in secret.chars() {
    let ascii = x as u32;
    let c = smallexp(ascii, e, n); // encrytped message {c} = (ascii ^ e) % n
    enc_ascii.push_str(&format!("{}", char::from_u32(c).expect("")));
    }
    return enc_ascii; 
}

fn decrypt(enc_passed: String)-> String {
    
    // get priv keys
    let mut file = std::fs::File::open("priv.key").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let parts: Vec<&str> = contents.split(' ').collect();
    let d: u32 = parts[0].parse().expect("");
    let n: u32 = parts[1].parse().expect("");

    let mut decrypt_str = String::new();
    for c in enc_passed.chars() {
        let m = smallexp(c as u32, d, n); // decrypted message {m} = (c ^ d) % n
        decrypt_str.push_str(&format!("{}",  char::from(m as u8)));
    }
    return decrypt_str; 
}

fn main() {

    generate();

    let secret = String::from("The greatest discovery of all time is that a person can change his future by merely changing his attitude");
    let encrypted_str = encrypt(secret);

    println!("encrypted text: \"{}\"\n", encrypted_str);
    println!("decrypted text: \"{}\"\n", decrypt(encrypted_str));

}