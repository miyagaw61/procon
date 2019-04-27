#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unknown_lints)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use std::collections::HashMap;
use std::cmp;
use std::process::exit;
use std::io::{
    stdin,
    stdout,
    stderr,
    Read,
    Write,
    BufReader,
    BufWriter
};

macro_rules! d {
    ($t:expr, $($e:expr),*) => {
        #[cfg(debug_assertions)]
        $({
            let (e, mut err) = (stringify!($e), stderr());
            writeln!(err, "\x1B[33m{}\x1B[0m = {:?}", $t, $e).unwrap()
        })*
    };
    ($($e:expr),*) => {
        #[cfg(debug_assertions)]
        $({
            let (e, mut err) = (stringify!($e), stderr());
            writeln!(err, "\x1B[33m{}\x1B[0m = {:?}", e, $e).unwrap()
        })*
    };
}

macro_rules! e {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            let mut err = stderr();
            let e = format!($($arg)*);
            write!(err, "\x1B[32m{}\x1B[0m", e).unwrap()
        }
    };
}

macro_rules! eln {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            let mut err = stderr();
            let e = format!($($arg)*);
            writeln!(err, "\x1B[32m{}\x1B[0m", e).unwrap()
        }
    };
}

macro_rules! stdin {
    () => ({
        use std::io::Read;
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    })
}

macro_rules! test {
    (name = $name:ident, $($input:expr => $output:expr),* $(,)*) => (
        #[test]
        fn $name() {
            $(
                assert_eq!(solve($input.to_string()), $output);
            )*
        }
    );
    ($($input:expr => $output:expr),* $(,)*) => (
        #[test]
        fn solve_test() {
            $(
                assert_eq!(solve($input.to_string()), $output);
            )*
        }
    )
}

macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn main() {
    println!("{}", solve(stdin!()));
}

#[allow(unconditional_recursion)]
fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    if b > a {
        let tmp = a;
        a = b;
        b = tmp;
    }
    if b == 0 {
        return a;
    }
    gcd(a%b, b)
}

fn csvec_new(src: &Vec<i32>) -> Vec<i32> {
    let mut cs: Vec<i32> = vec![src[0]];
    let len = src.len();
    for i in 0..len - 1 {
        let last = cs[i];
        cs.push(gcd(last, src[i + 1]));
    }
    cs
}

fn solve(src: String) -> String {
    let mut res = String::from("");
    input!{
        source = src,
        n: usize,
        a: [i32; n]
    }
    let mut a = a;
    eln!("########################################");
    d!("source", a);
    let mut cs_l: Vec<i32> = csvec_new(&a);
    a.reverse();
    let mut cs_r: Vec<i32> = csvec_new(&a);
    d!("i", 0);
    d!(cs_l);
    d!(cs_r);
    let mut max = 0;
    for i in 0..n {
        if i == 0 {
            max = cmp::max(max, cs_r[n-2]);
            continue;
        }
        if i == n - 1 {
            max = cmp::max(max, cs_l[n-2]);
            continue;
        }
        let max_l = cs_l[i - 1];
        let max_r = cs_r[n - i - 2];
        d!(i);
        d!(max_l);
        d!(max_r);
        let mut gcd_lr = gcd(max_l, max_r);
        max = cmp::max(max, gcd_lr);
    }
    res.push_str(&max.to_string());
    res.push_str("\n");
    res
}

test! {
"4
8 12 6 4
" =>
"4
",
"3
7 6 8
" =>
"2
",
"3
12 15 18
" =>
"6
",
"2
1000000000 1000000000
" =>
"1000000000
"
}

