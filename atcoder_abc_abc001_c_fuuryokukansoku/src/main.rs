
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
    input! {
        deg: f64,
        dis: f64
    }
    let mut deg: f64 = deg / 10.0;
    let dis: f64 = dis / 60.0;
    let dis: f64 = (dis * 10.0).round() / 10.0;
    let dir_list: Vec<&str> = vec!["N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW", "NW", "NNW"];
    let mut dir_index: usize = 0;
    deg = deg - 11.25;
    while deg > 0.0 {
        deg = deg - 22.5;
        dir_index = dir_index + 1;
    }
    if dir_index == 16 {
        dir_index = 0;
    }
    #[allow(non_snake_case)]
    #[allow(illegal_floating_point_literal_pattern)]
    let W: i64 = match dis {
        0.0...0.2 => 0,
        0.3...1.5 => 1,
        1.6...3.3 => 2,
        3.4...5.4 => 3,
        5.5...7.9 => 4,
        8.0...10.7 => 5,
        10.8...13.8 => 6,
        13.9...17.1 => 7,
        17.2...20.7 => 8,
        20.8...24.4 => 9,
        24.5...28.4 => 10,
        28.5...32.6 => 11,
        32.7...20000.0 => 12,
        _ => 13
    };
    if W == 0 {
        print!("{} ", "C");
    } else {
        print!("{} ", dir_list[dir_index]);
    }
    println!("{}", W);
}
