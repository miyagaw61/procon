#![allow(unused_macros)]
#![allow(unknown_lints)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(dead_code)]

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

#[allow(dead_code)]
fn npop(v: &mut Vec<char>, n: i32) -> Result<String, String> {
    let mut res: Vec<char> = Vec::new();
    for _ in 0..n {
        let c = v.pop().ok_or("".to_string())?;
        res.push(c);
    }
    res.reverse();
    let ret: String = res.iter().map(|c| *c).collect();
    return Ok(ret);
}

#[allow(dead_code)]
fn nget(v: &Vec<char>, n: i32) -> Result<String, String> {
    let mut tmp_v = v.clone();
    let res = npop(&mut tmp_v, n)?;
    return Ok(res);
}

#[allow(dead_code)]
fn is_valid_range(v: &Vec<char>, idx_a: i32, idx_b: i32) -> bool {
    let mut can = true;
    if idx_a < 0 {
        can = false;
    }
    let v_len = v.len() as i32;
    if v_len < idx_b {
        can = false;
    }
    return can;
}

#[allow(dead_code)]
fn get_range(v: &Vec<char>, idx_a: i32, idx_b: i32) -> Result<String, String> {
    if ! is_valid_range(v, idx_a, idx_b) {
        return Err("".to_string());
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
    return Ok(res);
}

#[allow(dead_code)]
fn pop_range(v: &mut Vec<char>, idx_a: i32, idx_b: i32) -> Result<String, String> {
    if ! is_valid_range(v, idx_a, idx_b) {
        return Err("".to_string());
    }
    let mut res = String::new();
    let mut pop_idxs: Vec<i32> = Vec::new();
    for (i,c) in v.iter().enumerate() {
        let i = i as i32;
        if i < idx_a {
            continue;
        }
        if idx_b <= i {
            break;
        }
        res.push(*c);
        pop_idxs.push(i);
    }
    for _ in 0..pop_idxs.len() {
        match pop_idxs.pop() {
            Some(idx) => {
                v.remove(idx as usize);
            },
            None => {}
        }
    }
    return Ok(res);
}

fn main() {
    input!{
        n: i32,
        h: [i32; n]
    }
    let mut res = 0;
    for i in 0..n {
        let mut can = true;
        for j in 0..i {
            if i == j {
                continue;
            }
            if h[i as usize] < h[j as usize] {
                can = false;
                break;
            }
        }
        if i == 0 {
            can = true;
        }
        if can {
            res = res + 1;
        }
    }
    println!("{}", res);
}
