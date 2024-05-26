use std::str::FromStr;
use std::env;
fn main() {
    let mut number: Vec<u64> = Vec::new();

    for arg in env::args().skip(1) {
        number.push(u64::from_str(&arg).expect("Error parsing arguement"));
    }
    if number.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = number[0];
    for m in &number[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", number, d);
}

fn gcd (mut n: u64, mut m: u64) -> u64 {
    assert!(m != 0 && n != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(
        2 * 3 * 5 * 11,
        3 * 5 * 7 * 11 * 19),
        165     );
}


