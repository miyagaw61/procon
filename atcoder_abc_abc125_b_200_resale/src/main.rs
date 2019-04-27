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

macro_rules! stdin {
    () => ({
        use std::io::Read;
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    })
}

macro_rules! test {
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

fn make_values(v: &Vec<i32>, c: &Vec<i32>, len: usize) -> i32 {
    let mut res: i32 = 0;
    for i in 0..len {
        let value = v[i] / c[i];
        if value > 0 {
            res += v[i] - c[i];
        }
    }
    res
}

fn solve(src: String) -> String {
    let mut res = String::from("");
    input!{
        source = src,
        n: usize,
        v: [i32; n],
        c: [i32; n]
    }
    let x = 0;
    let y = 0;
    let value = make_values(&v, &c, n);
    res.push_str(&value.to_string());
    res.push_str("\n");
    res
}

test! {
r"3
10 2 5
6 3 4
"
=>
r"5
",
r"4
13 21 6 19
11 30 6 15
"
=>
r"6
",
r"1
1
50
"
=>
r"0
"
}
