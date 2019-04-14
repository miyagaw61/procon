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

fn get_result(a_vec: &mut Vec<i32>) -> (i32, i32) {
    let mut alice_sum = 0;
    let mut bob_sum = 0;
    let mut cnt = 2;
    loop { 
        let a = a_vec.pop();
        cnt = cnt + 1;
        match a {
            Some(a) => {
                if cnt % 2 == 1 {
                    alice_sum = alice_sum + a;
                } else {
                    bob_sum = bob_sum + a;
                }
            },
            None => {
                return (alice_sum, bob_sum);
            }
        }
    }
}

fn alice_vs_bob(a_vec: &mut Vec<i32>) -> Result<i32, String> {
    let (alice_sum, bob_sum) = get_result(a_vec);
    println!("{}", alice_sum - bob_sum);
    return Ok(0);
}

fn main() {
    input! {
        n: i32,
        a_vec_src: [i32; n]
    }
    let mut a_vec = a_vec_src.clone();
    a_vec.sort();
    match alice_vs_bob(&mut a_vec) {
        Ok(_) => {},
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
