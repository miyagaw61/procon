fn main() {
    let h = 1000;
    let w = 1000;
    println!("{} {}", h, w);
    let mut v: Vec<[char; 1000]> = Vec::new();
    let mut tmp: [char; 1000] = ['.'; 1000];
    tmp[0] = '#';
    for i in 0..h {
        v.push(tmp);
    }
    for i in 0..h {
        for j in 0..w {
            print!("{}", v[i][j]);
        }
        print!("\n");
    }
}
