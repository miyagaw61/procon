#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unknown_lints)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use std::cmp;
use std::process::exit;
use std::collections::{
    HashMap,
    VecDeque
};
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
    );
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
    print!("{}", solve(stdin!()));
}

/* TEMPLATE END */

fn solve(src: String) -> String {
    let mut res = String::from("");
    input!{
        source = src,
        h: usize,
        w: usize,
        board: [chars; h],
    }
    let mut board = board;
    eln!("###########################################");
    d!(h);
    d!(w);
    d!(board);
    eln!("-------------------------------------------");
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut cnt = 0;
    for i in 0..h {
        for j in 0..w {
            if board[i][j] == '#' {
                q.push_back((i, j));
            }
        }
    }
    let mut n_op = q.len();
    loop {
        for _ in 0..n_op {
            let (i, j) = q.pop_front().unwrap();
            if i != 0 && board[i-1][j] == '.' {
                board[i-1][j] = '#';
                q.push_back((i-1, j));
            }
            if i != h-1 && board[i+1][j] == '.' {
                board[i+1][j] = '#';
                q.push_back((i+1, j));
            }
            if j != 0 && board[i][j-1] == '.' {
                board[i][j-1] = '#';
                q.push_back((i, j-1));
            }
            if j != w-1 && board[i][j+1] == '.' {
                board[i][j+1] = '#';
                q.push_back((i, j+1));
            }
        }
        n_op = q.len();
        if n_op == 0 {
            res.push_str(&cnt.to_string());
            d!(res);
            res.push_str("\n");
            return res;
        }
        cnt += 1;
    }
}

test! {
"3 3
...
.#.
...
" =>
"2
",
"3 3
#..
.#.
#..
" =>
"2
",
"3 3
#..
#..
#..
" =>
"2
",
"6 6
..#..#
......
#..#..
......
.#....
....#.
" =>
"3
",
}
