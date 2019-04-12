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

fn traveling(n: i32, v: Vec<Vec<i32>>) -> String {
    let mut ts: Vec<i32> = vec![0];
    let mut xs: Vec<i32> = vec![0];
    let mut ys: Vec<i32> = vec![0];
    for txy in v {
        ts.push(txy[0]);
        xs.push(txy[1]);
        ys.push(txy[2]);
    }
    let mut can = true;
    for i in 0..n {
        let i = i as usize;
        let dt = ts[i+1] - ts[i];
        let dist = (xs[i+1] - xs[i]).abs() + (ys[i+1] - ys[i]).abs();
        if dt < dist {
            can = false;
        }
        if dist % 2 != dt % 2 {
            can = false;
        }
    }
    if can {
        return "Yes".to_string();
    } else {
        return "No".to_string();
    }
}

fn main() {
    input!{
        n: i32,
        v: [[i32; 3]; n],
    }
    //let n: i32 = 1;
    //let v: Vec<Vec<i32>> = vec![vec![2, 100, 100]];
    println!("{}", traveling(n, v));
}
