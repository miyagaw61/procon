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

#[allow(dead_code)]
fn npop(v: &mut Vec<char>, n: i32) -> Result<String, String> {
    let mut res = String::new();
    for _ in 0..n {
        let c = v.pop().ok_or("".to_string())?;
        res.push(c);
    }
    return Ok(res);
}

#[allow(dead_code)]
fn npop_s(v: &Vec<char>, n: i32) -> Result<String, String> {
    let mut tmp_v = v.clone();
    let res = npop(&mut tmp_v, n)?;
    return Ok(res);
}

#[allow(dead_code)]
fn get_range(v: &Vec<char>, idx_a: i32, idx_b: i32) -> Option<String> {
    if idx_a < 0 {
        return None
    }
    let v_len = v.len() as i32;
    if v_len < idx_b {
        return None
    }
    let mut res = String::new();
    for (i,c) in v.iter().enumerate() {
        let i = i as i32;
        if i < idx_a {
            continue;
        }
        if idx_b <= i {
            break;
        }
        res.push(*c);
    }
    return Some(res);
}

//#[allow(unused_must_use)]
//#[allow(unused_variables)]
//#[allow(unused_mut)]
fn main() {
    input!{
        s: String
    }
    let xs = ["dream", "dreamer", "erase", "eraser"];
    let cs: Vec<char> = s.chars().collect();
    let mut can = true;
    let mut idx = cs.len() as i32;
    loop {
        let mut can2 = false;
        for x in xs.iter() {
            let x_len = x.len() as i32;
            let start = idx - x_len;
            match get_range(&cs, start, idx) {
                Some(fragment) => {
                    if &fragment == x {
                        idx = idx - x_len;
                        can2 = true;
                    }
                    continue;
                },
                None => {
                    continue;
                }
            }
        }
        if ! can2 {
            can = false;
            break;
        }
        if idx <= 0 {
            break;
        }
    }
    if can {
        println!("YES");
    } else {
        println!("NO");
    }
}
