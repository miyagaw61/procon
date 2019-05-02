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

fn solve(src: String) -> String {
    let mut res = String::from("");
    input!{
        source = src,
        n: usize,
        ai: [i64; n]
    }
    eln!("###################################");
    d!(n);
    d!(ai);
    eln!("-----------------------------------");
    let mut ai = ai;
    let mut is_odd = false;
    let mut minus_num = 0;
    for i in 0..ai.len() {
        if ai[i] < 0 {
            minus_num += 1;
        }
    }
    if minus_num % 2 == 1 {
        is_odd = true;
    }
    let mut sum = 0;
    d!(is_odd);
    if is_odd {
        let mut min = ai[0].abs();
        for i in 0..ai.len() {
            if ai[i].abs() < min {
                min = ai[i].abs();
            }
            sum += ai[i].abs();
        }
        sum -= min.abs() * 2;
    } else {
        for i in 0..ai.len() {
            sum += ai[i].abs();
        }
    }
    res.push_str(&sum.to_string());
    d!(res);
    res.push_str("\n");
    res
}

test! {
r"3
-10 5 -4
"
=>
r"19
",
r"3
-5 10 -4
"
=>
r"19
",
r"5
10 -4 -8 -11 3
" =>
r"30
",
"11
-1000000000 1000000000 -1000000000 1000000000 -1000000000 0 1000000000 -1000000000 1000000000 -1000000000 1000000000
" =>
"10000000000
",
"3
2 -5 100
" =>
"103
"
}
