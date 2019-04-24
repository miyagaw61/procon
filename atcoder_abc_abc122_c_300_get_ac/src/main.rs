#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unknown_lints)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use std::collections::HashMap;
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
        //let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
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

fn csvec_new(src: Vec<char>) -> Vec<i32> {
    let mut cs: Vec<i32> = vec![0];
    cs.push(0);
    debug!(cs);
    let len = src.len();
    let mut new = 0;
    for i in 1..len {
        if src[i-1] == 'A' && src[i] == 'C' {
            new = new + 1;
        }
        cs.push(new);
        debug!(src[i-1]);
        debug!(src[i]);
        debug!(cs);
    }
    cs
}

fn main() {
    input!{
        n: usize,
        q: usize,
        s: String,
        lr: [(usize, usize); q]
    } debug!(n);
    debug!(q);
    debug!(s);
    debug!(lr);
    let s: Vec<char> = s.chars().collect();
    let cs = csvec_new(s);
    debug!(cs);
    let stdout_ = stdout();
    //let mut stdout = std::io::BufWriter::new(stdout_.lock());
    let mut stdout = BufWriter::new(stdout_);
    for i in 0..q {
        debug!(i);
        let (l, r) = lr[i];
        debug!(l);
        debug!(r);
        let res = cs[r] - cs[l];
        //println!("{}", res);
        writeln!(stdout, "{}", res).unwrap();
    }
}
