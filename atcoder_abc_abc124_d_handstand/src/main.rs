#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unknown_lints)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use std::collections::HashMap;
use std::io::{stderr, Write};
use std::process::exit;

macro_rules! debug {
    ($($e:expr),*) => {
        #[cfg(debug_assertions)]
        $({
            let (e, mut err) = (stringify!($e), stderr());
            writeln!(err, "\x1B[33m{}\x1B[0m = {:?}", e, $e).unwrap()
        })*
    };
}

macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
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
    input!{
        n: i32,
        k: i32,
        s: String
    }
    let mut s: Vec<char> = s.chars().collect();

    //let mut n: i32 = 4;
    //let mut k: i32 = 1;
    //let mut s: Vec<char> = Vec::new();
    //let mut s: Vec<char> = vec!['0', '0', '1', '0', '1', '1'];

    let mut l: usize = 0;
    let s_len = s.len();
    let mut sums: Vec<usize> = Vec::new();
    let mut r: usize = 1;
    let i = 0;
    for i in 0..k {
        while r < s_len && s[r] == s[l] {
            r = r + 1;
        }
        while r < s_len && s[r] != s[l] {
            r = r + 1;
        }
        while r < s_len && s[r] == s[l] {
            if s[r] == '1' {
                r = r + 1;
            } else if i == k-1 {
                break;
            } else {
                r = r + 1;
            }
        }
        r = r - 1;
        debug!(l);
        debug!(r);
    }
    let sum = r - l + 1;
    debug!(l);
    debug!(r);
    debug!(sum);
    sums.push(sum);
    while r < s_len-1 {
        let start = s[l];
        while s[l] == start && l < s_len-1 {
            l = l + 1;
        }
        while start != '0' && s[l] != start && l < s_len-1 {
        //while s[l] != start && l < s_len-1 {
            l = l + 1;
        }
        if r < s_len {
            r = r + 1;
        }
        if start == '0' {
            while r < s_len && s[r] == '1' {
                r = r + 1;
            }
            while r < s_len && s[r] == '0' {
                r = r + 1;
            }
            while r < s_len && s[r] == '1' {
                r = r + 1;
            }
        } else {
            while r < s_len && s[r] != start {
                r = r + 1;
            }
            if start == '1' {
                while r < s_len && s[r] == start {
                    r = r + 1;
                }
            }
        }
        r = r - 1;
        let sum = r - l + 1;
        debug!(l);
        debug!(r);
        debug!(sum);
        sums.push(sum);
    }
    let max = sums.iter().max();
    match max {
        Some(max) => {
            println!("{}", max);
        },
        None => {}
    }
}
