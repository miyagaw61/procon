#[allow(unused_macros)]
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

#[allow(unused_macros)]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[allow(unused_macros)]
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

#[allow(unused_must_use)]
#[allow(unused_variables)]
#[allow(unused_mut)]
fn otoshidama(n:i32, y:i32) -> (i32, i32, i32) {
    let mut res10000 = -1;
    let mut res5000 = -1;
    let mut res1000 = -1;
    let mut i_vec = Vec::with_capacity(n as usize);
    let mut j_vec = Vec::with_capacity(n as usize);
    for m in 0..n+1 {
        i_vec.push(m);
        j_vec.push(m);
    }
    let i_itr = i_vec.iter();
    let j_itr = j_vec.iter();
    for i in i_itr.as_ref() {
        for j in j_itr.as_ref() {
            let k = n - *i - *j;
            if k < 0 {
                break;
            }
            let total = 10000*i + 5000*j + 1000*k;
            if total == y {
                res10000 = *i;
                res5000 = *j;
                res1000 = k;
            }
        }
    }
    return (res10000, res5000, res1000);
}

fn main() {
    input!{
        n: i32,
        y: i32
    }
    let (res10000, res5000, res1000) = otoshidama(n, y);
    println!("{} {} {}", res10000, res5000, res1000);
}
