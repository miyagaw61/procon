extern crate renert;
use renert::*;

fn main() {
    println!("{}", solve(stdin!()));
}

fn is_prime(n: i32) -> bool {
    if n == 2 {
        return true;
    }
    if n & 1 == 0 {
        return false;
    }
    let mut i = 3;
    loop {
        if i == n {
            return true;
        }
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
}

fn solve(src: String) -> String {
    input! {
        source = src,
        x: i32
    }
    let mut x = x;
    loop {
        if is_prime(x) {
            return x.to_string();
        }
        x += 1;
    }
}
